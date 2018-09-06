pub mod one_min;
pub mod five_min;
pub mod period;
use timestamp::{ TimeStamp };
use self::period::{PeriodIdentity};

pub use self::one_min::OneMin;
pub use self::period::{Period};

pub trait TimeRange {
    fn range_start(&self) -> TimeStamp;
    fn range_end(&self) -> TimeStamp;
    fn prior_start(&self) -> TimeStamp;
    fn prior_end(&self) -> TimeStamp;
    fn previous_range(&self) -> Self;
}

/**
 * Argument will be a period and we will return something which implement TimeRange
 */
pub fn analysis_range(periodId: period::PeriodId) -> impl TimeRange {
    match periodId.period() {
        period::Period::OneMin => OneMin::new(),
        _ => OneMin::new()
    }
}
