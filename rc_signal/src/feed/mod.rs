pub mod bitfinex;
use timestamp::{ TimeStamp };
use models;

pub trait PollTrades {
    fn get_trades(since_timestamp: TimeStamp) -> Result<Vec<models::NewTrade>, String>;
}


