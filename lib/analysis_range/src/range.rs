use timestamp::{ TimeStamp };
use chrono::{Utc, Timelike, Duration, TimeZone, NaiveDateTime};
use super::TimePeriod;
use chrono::offset;
use super::{Period, PeriodDuration, TimeRange};

#[derive(Debug)]
pub struct Range {
    pub start_timestamp: TimeStamp,
    pub end_timestamp: TimeStamp,
    pub period: Period,
    pub prior_start_timestamp: TimeStamp
}

impl Range {
    pub fn new(period: Period) -> Range {
        let now_date_time = Utc::now().naive_utc();
        let now_time = now_date_time.time();
        let now_date = now_date_time.date();
        let start_minute = now_time.minute();

        let start_datetime: NaiveDateTime = now_date.and_hms(now_time.hour(), start_minute, 0);
        Range::create_from_start_datetime(period, start_datetime)
    }

    fn create_from_start_datetime(period: Period, start_datetime: NaiveDateTime) -> Range {
        let end_datetime = start_datetime
            .checked_add_signed(Duration::seconds(period.duration()))
            .unwrap();

        let prior_start_datetime = start_datetime
            .checked_sub_signed(Duration::seconds(period.duration()))
            .unwrap();

        Range {
            start_timestamp: start_datetime.timestamp(),
            end_timestamp: end_datetime.timestamp(),
            prior_start_timestamp: prior_start_datetime.timestamp(),
            period: period
        }
    }

    pub fn prev_range(&self) -> Range {
        let start_date = offset::Utc.timestamp(self.prior_start_timestamp, 0u32);
        let prior_start_timestamp = start_date
            .checked_sub_signed(Duration::seconds(self.period.duration()))
            .unwrap();

        Range {
            start_timestamp: self.prior_start_timestamp,
            end_timestamp: self.start_timestamp,
            prior_start_timestamp: prior_start_timestamp.timestamp(),
            period: self.period
        }
    }
}

impl TimePeriod for  Range {
    fn debug(&self) -> String {
        format!("{:?}", self)
    }

    fn create_from_start_timestamp(period: Period, start_timestamp: TimeStamp) -> Self {
        let now_date_time = offset::Utc.timestamp(start_timestamp, 0u32).naive_utc();
        let now_time = now_date_time.time();
        let now_date = now_date_time.date();
        let start_minute = now_time.minute();

        let start_datetime: NaiveDateTime = now_date.and_hms(now_time.hour(), start_minute, 0);
        Self::create_from_start_datetime(period, start_datetime)
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

    fn previous_range(&self) -> Self {
        self.prev_range()
    }

    fn period(&self) -> Period {
        self.period
    }

    fn get_prev_period_range(&self, no: i64) -> Vec<Self> {
        (0..no)
            .into_iter()
            .map(|period| {
                let base = NaiveDateTime::from_timestamp(self.start_timestamp, 0u32);
                base.checked_sub_signed(Duration::seconds(period * self.period.duration())).unwrap()
            })
            .map(|start_datetime| Self::create_from_start_datetime(self.period, start_datetime))
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

#[cfg(test)]
mod test {
    use super::{Range , Period, TimeRange, TimePeriod};

    fn ref_range() -> Range {
        Range {
            start_timestamp: 1538718120i64,
            end_timestamp: 15387190200i64,
            period: Period::FifteenMin,
            prior_start_timestamp: 1538717220i64,
        }
    }

    #[test]
    fn get_prev_period_range_for_one() {
        let range = ref_range();

        let prev_period_range = range.get_prev_period_range(1);
        let starts: Vec<i64> = prev_period_range.iter().map(|candle| candle.start_timestamp).collect();
        assert_eq!(starts, vec![ 1538718120 ]);
    }

    #[test]
    fn get_prev_period_range_for_multiple() {
        let range = ref_range();

        let prev_period_range = range.get_prev_period_range(3);
        let starts: Vec<i64> = prev_period_range.iter().map(|candle| candle.start_timestamp).collect();
        assert_eq!(starts, vec![
            1538718120,
            1538717220,
            1538716320,
        ]);

        let ends: Vec<i64> = prev_period_range.iter().map(|candle| candle.end_timestamp).collect();
        assert_eq!(ends, vec![
            1538719020,
            1538718120,
            1538717220,
        ]);
    }

    #[test]
    fn get_prev_period_time_range() {
        let range = ref_range();

        let time_range = range.get_prev_period_time_range(3);
        assert_eq!(time_range.start_timestamp, 1538716320);
        assert_eq!(time_range.end_timestamp, 1538719020);
    }

    #[test]
    fn prev_range() {
        let range = ref_range();

        let prev_range = range.prev_range();
        assert_eq!(prev_range.start_timestamp, 1538717220i64);
        assert_eq!(prev_range.end_timestamp, 1538718120i64);
    }
}

