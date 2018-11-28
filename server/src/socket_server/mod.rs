use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, ErrorKind};
use ws;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use ::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use analysis_range::{ Range, Period};
use models::{ Trade };
use serde_json;
use std::{thread, time};

pub type Registry = Arc<Mutex<HashMap<String, bool>>>;
pub struct Server {
    pub out: Sender,
    pub this_id: String,
    pub registry: Registry,
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let sender = self.out.clone();
        let id_string = format!("{}", self.this_id);
        let reg_clone = self.registry.clone();
        self.registry.lock().unwrap().insert(self.this_id.clone(), false);

        self.pool.get()
            .map_err(|err| {
                ws::Error{
                    kind: ErrorKind::Internal,
                    details: std::borrow::Cow::Borrowed("Could not get pg connection")
                }
            })
            .and_then(|conn| {
                thread::spawn(move || {
                    loop {
                        thread::sleep(time::Duration::from_millis(1000));
                        {
                            let mut reg = reg_clone.lock();
                            match reg {
                                Ok(mut map) => {
                                    if (*map.get(&id_string).unwrap()) {
                                        map.remove(&id_string);
                                        info!("Stopping thread for ws_con to {}", id_string);
                                        return;
                                    }
                                },
                                Err(e) => {
                                    error!("{:?}", e);
                                    return;
                                }
                            }
                        }
                        let range = analysis_range::Range::new(Period::OneMin).prev_range();
                        let trades = Trade::trades_in_range(&conn, &range).unwrap();
                        let res = serde_json::to_string(&trades)
                            .map_err(|err| {
                                ws::Error{
                                    kind: ErrorKind::Internal,
                                    details: std::borrow::Cow::Borrowed("Error parsing server data")
                                }
                            })
                            .and_then(|msg| {
                                sender.send(Message::Text(msg))
                            });

                        if res.is_err() {
                            break;
                        }
                    };
                });
                Ok(())
            })
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }

        let mut reg = self.registry.lock().unwrap();
        reg.insert(self.this_id.clone(), true);
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("The server encountered an error: {:?}", err);
    }

}
