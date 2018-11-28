use super::{TimePeriod};
use super::{TimeStamp};
use super::range::{Range};

#[derive(PartialEq, Debug, Eq, Clone, Copy)]
pub enum Period {
    OneMin = 1isize,
    FifteenMin = 2isize,
    ThirtyMin = 3isize,
    Hour = 4isize
}

pub type PeriodId = i32;

pub trait PeriodIdentity {
    fn period(self) -> Period;
}

pub trait PeriodDuration {
    fn duration(self) -> i64;
}

impl PeriodIdentity for PeriodId {
    fn period(self) -> Period {
        match self {
            _ if self == (Period::OneMin as i32) => Period::OneMin,
            _ if self == (Period::FifteenMin as i32) => Period::FifteenMin,
            _ if self == (Period::ThirtyMin as i32) => Period::ThirtyMin,
            _ if self == (Period::Hour as i32) => Period::Hour,
            _ => panic!("not a period")
        }
    }
}

impl Period {
    pub fn analysis_range(self) -> impl TimePeriod {
        Range::new(self)
    }

    pub fn range_from (self, start_timestamp: TimeStamp) -> impl TimePeriod {
        Range::create_from_start_timestamp(self, start_timestamp)
    }
}

impl PeriodDuration for Period {
    fn duration(self) -> i64 {
        match self {
            Period::OneMin => 60,
            Period::FifteenMin => 900,
            Period::ThirtyMin => 1800,
            Period::Hour => 3600
        }
    }
}
