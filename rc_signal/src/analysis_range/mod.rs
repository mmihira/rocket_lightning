pub mod one_min;
pub mod five_min;
pub mod period;
use timestamp::{ TimeStamp };
use self::period::{PeriodIdentity};
use std;

pub use self::one_min::OneMin;
pub use self::period::{Period};

#[derive(Debug)]
pub struct TimeRange<T: TimePeriod> {
    range: Vec<T>,
    start_timestamp: TimeStamp,
    end_timestamp: TimeStamp
}

pub trait TimePeriod: std::marker::Sized + std::fmt::Debug {
    fn range_start(&self) -> TimeStamp;
    fn range_end(&self) -> TimeStamp;
    fn prior_start(&self) -> TimeStamp;
    fn prior_end(&self) -> TimeStamp;
    fn previous_range(&self) -> Self;
    fn period(&self) -> Period;
    fn get_prev_period_range(&self, no: i64) -> Vec<Self>;
    fn debug(&self) -> String;
    fn get_prev_period_time_range(&self, no: i64) -> TimeRange<Self>;
}
