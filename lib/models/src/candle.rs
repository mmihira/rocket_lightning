use timestamp::{TimeStamp};
use rc_schema::schema::candles;
use rc_schema::schema::candles::dsl::candles as candles_dsl;
use diesel::prelude::{PgConnection};
use diesel::{Identifiable, ExpressionMethods, QueryDsl, RunQueryDsl, BoolExpressionMethods};
use diesel::result::Error as DieselError;

#[table_name="candles"]
#[primary_key(period, start_time, end_time)]
#[derive(Debug, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, PartialEq)]
pub struct Candle {
    pub period: i32,
    pub start_time: TimeStamp,
    pub end_time: TimeStamp,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub vol: f32,
    pub rsi_avg_gain: f32,
    pub rsi_avg_loss: f32,
    pub rsi: f32,
    pub rsi_smoothed: f32,
    pub sma_9: f32,
    pub sma_12: f32,
    pub sma_26: f32,
    pub ema_9: f32,
    pub ema_12: f32,
    pub ema_26: f32
}

impl Candle{
    pub fn sma_for_interval(&self, interval: i64) -> f32 {
        match interval {
            9 => self.sma_9,
            12 => self.sma_12,
            26 => self.sma_26,
            _ => panic!("Not a supported inerval!")
        }
    }
}

