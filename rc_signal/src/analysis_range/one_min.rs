use timestamp::{ TimeStamp };
use chrono::{Utc, Timelike, Duration, TimeZone, NaiveDateTime};
use super::TimePeriod;
use chrono::offset;
use super::{Period, TimeRange};

#[derive(Debug)]
pub struct OneMin {
    start_timestamp: TimeStamp,
    end_timestamp: TimeStamp,
    period: Period,
    prior_start_timestamp: TimeStamp
}

const SEC_DURATION: i64 = 60;
const PERIOD: Period = Period::OneMin;

impl OneMin {
    pub fn new() -> OneMin {
        let now_date_time = Utc::now().naive_utc();
        let now_time = now_date_time.time();
        let now_date = now_date_time.date();
        let start_minute = now_time.minute();

        let start_datetime: NaiveDateTime = now_date.and_hms(now_time.hour(), start_minute, 0);
        OneMin::create_from_start_datetime(start_datetime)
    }

    fn create_from_start_datetime(start_datetime: NaiveDateTime) -> OneMin {
        let end_datetime = start_datetime.checked_add_signed(Duration::seconds(SEC_DURATION)).unwrap();

        let prior_start_datetime = start_datetime.checked_sub_signed(Duration::seconds(SEC_DURATION)).unwrap();

        OneMin {
            start_timestamp: start_datetime.timestamp(),
            end_timestamp: end_datetime.timestamp(),
            prior_start_timestamp: prior_start_datetime.timestamp(),
            period: PERIOD
        }
    }

    pub fn prev_range(&self) -> OneMin {
        let start_date = offset::Utc.timestamp(self.prior_start_timestamp, 0u32);
        let prior_start_timestamp = start_date.checked_sub_signed(Duration::seconds(SEC_DURATION)).unwrap();

        OneMin {
            start_timestamp: self.prior_start_timestamp,
            end_timestamp: self.start_timestamp,
            prior_start_timestamp: prior_start_timestamp.timestamp(),
            period: PERIOD
        }
    }
}

impl TimePeriod for OneMin {
    fn debug(&self) -> String {
        format!("{:?}", self)
    }

    fn range_start(&self) -> TimeStamp {
        self.start_timestamp
    }

    fn range_end(&self) -> TimeStamp {
        self.end_timestamp
    }

    fn prior_start(&self) -> TimeStamp {
        self.prior_start_timestamp
    }

    fn prior_end(&self) -> TimeStamp {
        self.start_timestamp
    }

    fn previous_range(&self) -> OneMin {
        self.prev_range()
    }

    fn period(&self) -> Period {
        self.period
    }

    fn get_prev_period_range(&self, no: i64) -> Vec<OneMin> {
        (0..no)
            .into_iter()
            .map(|period| {
                let base = NaiveDateTime::from_timestamp(self.start_timestamp, 0u32);
                base.checked_sub_signed(Duration::seconds(period * SEC_DURATION)).unwrap()
            })
            .map(|start_datetime| OneMin::create_from_start_datetime(start_datetime))
            .collect()
    }

    fn get_prev_period_time_range(&self, no: i64) -> TimeRange<Self> {
        let range = self.get_prev_period_range(no);
        let start = range.last().unwrap().range_start();
        let end = range.first().unwrap().range_end();
        TimeRange {
            range: range,
            start_timestamp: start,
            end_timestamp: end,
        }
    }
}
