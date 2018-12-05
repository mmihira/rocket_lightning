use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, ErrorKind};
use ws;
use std::rc::Rc;
use std::sync::{Arc, Mutex, Barrier};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use ::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use analysis_range::{ Range, Period, TimePeriod};
use models::{ Trade };
use serde_json;
use std::{thread, time};
use std::mem;

pub struct RegistryEntry {
    pub barrier: Arc<Barrier>,
    pub hasClosed: bool
}

pub type Registry = Rc<RefCell<HashMap<String, Arc<Mutex<RegistryEntry>>>>>;

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
        let reg_clone = self.registry.borrow_mut().insert(
            id_string.clone(),
            Arc::new(Mutex::new(RegistryEntry {
                barrier: Arc::new(Barrier::new(2)),
                hasClosed: false
            }))
        );
        let arc_clone = self.registry.borrow().get(&id_string).unwrap().clone();

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
                            let reg = arc_clone.lock();
                            match reg {
                                Ok(mutex) => {
                                    if (mutex.hasClosed) {
                                        info!("Stopping sending to {}", id_string);
                                        mutex.barrier.wait();
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
                        let range = analysis_range::Range::new(Period::OneMin);
                        let time_range = range.get_prev_period_time_range(2);
                        let res = Trade::in_timestamp_range(&conn, time_range.start_timestamp, time_range.end_timestamp)
                            .map_err(|err: diesel::result::Error| {
                                ws::Error{
                                    kind: ErrorKind::Internal,
                                    details: std::borrow::Cow::Borrowed("Could not get trades")
                                }
                            })
                            .and_then(|trades| {
                                serde_json::to_string(&trades)
                                    .map_err(|err| {
                                        ws::Error{
                                            kind: ErrorKind::Internal,
                                            details: std::borrow::Cow::Borrowed("Error parsing server data")
                                        }
                                    })
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

        let mut barrier: Option<Arc<Barrier>> = None;
        {
            let r = self.registry.borrow_mut();
            let mut mutexguard = r.get(&self.this_id).unwrap().lock().unwrap();
            mem::replace(&mut mutexguard.hasClosed, true);
            barrier = Some(mutexguard.barrier.clone())
        }
        barrier.unwrap().wait();
        {
            let mut r = self.registry.borrow_mut();
            let mutexguard = r.remove(&self.this_id);
            info!("Removed {} from watch list", self.this_id);
        }
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("The server encountered an error: {:?}", err);
    }

}
