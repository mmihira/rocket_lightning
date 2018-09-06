use chrono::{NaiveDateTime};

pub type TimeStamp = i64;

#[derive(Debug)]
pub struct TimeStampRange {
    pub start: TimeStamp,
    pub end: TimeStamp
}

pub trait Conversions {
    fn as_date_string(&self) -> String;
    fn start_14_5min_prior(&self) -> TimeStamp;
}

impl Conversions for TimeStamp {
    fn start_14_5min_prior(&self) -> TimeStamp {
        *self - (60 * 5) * 14
    }

    fn as_date_string(&self) -> String {
        format!("{:?}", NaiveDateTime::from_timestamp_opt(*self, 0).unwrap())
    }
}


