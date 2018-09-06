use super::one_min;
use super::{TimeRange};

#[derive(PartialEq, Eq)]
pub enum Period {
    OneMin,
    FiveMin,
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
            1 => Period::OneMin,
            2 => Period::FiveMin,
            3 => Period::FifteenMin,
            4 => Period::ThirtyMin,
            5 => Period::OneHour,
            6 => Period::FourHour,
            _ => panic!("not a period")
        }
    }
}

impl Period {
    pub fn analysis_range(&self) -> impl TimeRange {
        match *self {
            Period::OneMin => one_min::OneMin::new(),
            _ => one_min::OneMin::new()
        }
    }
}


