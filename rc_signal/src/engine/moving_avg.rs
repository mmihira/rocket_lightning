use diesel::result::Error as DieselError;
use models::{Candle};
use analysis_range::{ TimePeriod, TimeRange };
use diesel::prelude::{PgConnection};

pub fn period_9<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<f32, String> {
    for_period(conn, 9, range)
}

pub fn period_12<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<f32, String> {
    for_period(conn, 12, range)
}

fn for_period<T: TimePeriod>(conn: &PgConnection, interval: i64, range: &T) -> Result<f32, String> {
    let period_range = range.get_prev_period_time_range(interval);
    let TimeRange { start_timestamp, end_timestamp, ..} = period_range;

    let candles_in_range = Candle::in_range(
        conn,
        range.period(),
        start_timestamp,
        end_timestamp
    );

    candles_in_range
        .map_err(|err: DieselError| {
            format!("{}", err)
        })
        .and_then(|candles| {
            match candles.len() as i64 {
                len if len == interval => {
                    let sum: f32 = candles.iter().map(|c| c.close).sum();
                    Ok( sum / interval as f32)
                }
                len => Err(format!("Not enough data. Expected {}, got {}", interval, len))
            }
        })
}
