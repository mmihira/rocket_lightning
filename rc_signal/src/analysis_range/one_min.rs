use timestamp::{ TimeStamp, TimeStampRange };
use chrono::{Utc, Timelike, Duration, TimeZone};
use super::TimeRange;
use chrono::offset;

#[derive(Debug)]
pub struct OneMin {
    start_timestamp: TimeStamp,
    end_timestamp: TimeStamp,
    prior_start_timestamp: TimeStamp
}

impl OneMin {
    pub fn new() -> OneMin {
        let now_date_time = Utc::now().naive_utc();
        let now_time = now_date_time.time();
        let now_date = now_date_time.date();
        let start_minute = now_time.minute();

        let start_timestamp = now_date.and_hms(now_time.hour(), start_minute, 0);
        let end_timestamp = start_timestamp.checked_add_signed(Duration::seconds(60)).unwrap();

        let prior_start_timestamp = start_timestamp.checked_sub_signed(Duration::seconds(60)).unwrap();

        OneMin {
            start_timestamp: start_timestamp.timestamp(),
            end_timestamp: end_timestamp.timestamp(),
            prior_start_timestamp: prior_start_timestamp.timestamp()
        }
    }

    pub fn prev_range(&self) -> OneMin {
        let start_date = offset::Utc.timestamp(self.prior_start_timestamp, 0u32);
        let prior_start_timestamp = start_date.checked_sub_signed(Duration::seconds(60)).unwrap();

        OneMin {
            start_timestamp: self.prior_start_timestamp,
            end_timestamp: self.start_timestamp,
            prior_start_timestamp: prior_start_timestamp.timestamp()
        }
    }
}

impl TimeRange for OneMin {
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
}
