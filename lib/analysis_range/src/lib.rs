extern crate chrono;
extern crate timestamp;
pub mod period;
pub mod range;
use timestamp::{ TimeStamp };


/**
 * Period: A time period such as an hour, or day, or fifteen minutes
 * Range: A general object which has the details for some time period
 *
 */

pub use self::period::{Period, PeriodIdentity, PeriodDuration};
pub use range::{Range};

#[derive(Debug)]
pub struct TimeRange<T: TimePeriod> {
    pub range: Vec<T>,
    pub start_timestamp: TimeStamp,
    pub end_timestamp: TimeStamp
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
    fn create_from_start_timestamp(period: Period, start_timestamp: TimeStamp) -> Self;
}
