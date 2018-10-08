use super::one_min;
use super::{TimePeriod};
use super::{TimeStamp};

#[derive(PartialEq, Debug, Eq, Clone, Copy)]
pub enum Period {
    OneMin = 1isize,
    FiveMin = 2isize,
    FifteenMin,
    ThirtyMin,
    OneHour,
    FourHour,
    TwelveHour,
    OneDay,
    ThreeDay,
    Week
}

pub type PeriodId = i32;

pub trait PeriodIdentity {
    fn period(self) -> Period;
}

impl PeriodIdentity for PeriodId {
    fn period(self) -> Period {
        match self {
            _ if self == (Period::OneMin as i32) => Period::OneMin,
            _ if self == (Period::FiveMin as i32) => Period::FiveMin,
            _ if self == (Period::FifteenMin as i32) => Period::FifteenMin,
            _ if self == (Period::ThirtyMin as i32) => Period::ThirtyMin,
            _ if self == (Period::OneHour as i32) => Period::OneHour,
            _ if self == (Period::FourHour as i32)  => Period::FourHour,
            _ => panic!("not a period")
        }
    }
}

impl Period {
    pub fn analysis_range(self) -> impl TimePeriod {
        match self {
            Period::OneMin => one_min::OneMin::new(),
            _ => one_min::OneMin::new()
        }
    }

    pub fn range_from(self, start_timestamp: TimeStamp) -> impl TimePeriod {
        match self {
            Period::OneMin => one_min::OneMin::create_from_start_timestamp(start_timestamp),
            _ => one_min::OneMin::new()
        }
    }
}
