use diesel::result::Error as DieselError;
use models::{Candle};
use analysis_range::{ TimePeriod, TimeRange };
use diesel::prelude::{PgConnection};

pub fn period_9<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<f32, String> {
    sma_for_interval(conn, 9, range)
}

pub fn period_12<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<f32, String> {
    sma_for_interval(conn, 12, range)
}

pub fn period_26<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<f32, String> {
    sma_for_interval(conn, 26, range)
}

pub fn ema_period_9<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<f32, String> {
    ema_for_interval(conn, 9, range)
}


fn sma_for_interval<T: TimePeriod>(conn: &PgConnection, interval: i64, range: &T) -> Result<f32, String> {
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

fn ema_for_interval<T: TimePeriod>(conn: &PgConnection, interval: i64, range: &T) -> Result<f32, String> {
    Candle::prev_candle_of_range(conn, range)
        .map_err(|err: DieselError| format!("{:?}", err))
        .and_then(|candle| {
            if candle.ema_for_interval(interval) == 0.0 {
                sma_for_interval(conn, interval, &range.previous_range())
                    .map_err(|err| format!("Initialising ema with sma, err: {}", err))
                    .map(|sma| (candle.close, sma))
            } else {
                Ok((candle.close, candle.ema_for_interval(interval)))
            }
        })
        .map(|prev_values| {
            let close = prev_values.0;
            let init_ema = prev_values.1;
            let mult = (2 / (interval + 1));
            (close - init_ema) * mult as f32 + init_ema
        })
}
