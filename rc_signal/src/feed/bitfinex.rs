use std::string::FromUtf8Error;
use serde_json;
use curl::easy::Easy;
use models;
use super::PollTrades;
use timestamp::{ TimeStamp };
use std::str::FromStr;

pub struct Public { }

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Trade {
    #[serde(rename="type")] pub _type: String,
    pub timestamp: TimeStamp,
    pub exchange: String,
    pub amount: String,
    pub price: String,
    pub tid: i32
}

impl PollTrades for Public {
    fn get_trades(since_timestamp: TimeStamp) -> Result<Vec<models::NewTrade>, String> {
        let mut easy = Easy::new();
        let mut dst: Vec<u8> = Vec::new();
        {
            let url = format!(
                "https://api.bitfinex.com/v1/trades/btcusd?timestamp={}&limit_trades=200",
                since_timestamp
                );
            if let Err(e) = easy.url(&url) {
                return Err(format!("{}", e));
            }
            let mut transfer = easy.transfer();
            let transfer_result = transfer.write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            });
            if let Err(e) = transfer_result {
                return Err(format!("{}", e));
            }
            transfer.perform();
        };

        String::from_utf8(dst)
            .map_err(|err: FromUtf8Error| {
                format!("{}", err)
            })
            .and_then(|result: String|{
                serde_json::from_str(&result)
                    .map(|x| x)
                    .map_err(|x| format!("{:?}", x))
            })
            .map(|result: Vec<Trade>| {
                result
                    .iter()
                    .map(|ref trade_el| {
                        // These values *should* be decimal values
                        let vol = f32::from_str(&trade_el.amount).unwrap();
                        let price = f32::from_str(&trade_el.price).unwrap();

                        models::NewTrade {
                            timestamp: trade_el.timestamp,
                            tid: trade_el.tid,
                            vol: vol,
                            price: price
                        }
                    })
                    .collect()
            })
    }
}

