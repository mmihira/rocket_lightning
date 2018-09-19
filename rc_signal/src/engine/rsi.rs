use models::{Candle};
use analysis_range::{ TimePeriod, TimeRange };
use diesel::prelude::{PgConnection};
use diesel::result::Error as DieselError;

const RSI_INTERVAL: i64 = 14;

pub struct GainLoss {
    avg_gain: f32,
    avg_loss: f32,
    curr_gain: f32,
    curr_loss: f32
}

pub fn rsi_for_interval<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<f32, String> {
    let period_range = range.get_prev_period_time_range(RSI_INTERVAL);
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
                len if len == RSI_INTERVAL => Ok(candles),
                len => Err(format!("Not enough data. Expected {}, got {}", RSI_INTERVAL, len))
            }
        })
        .and_then(|candles| calc_avg_gain_loss(candles))
        .and_then(|GainLoss{ avg_gain, avg_loss, curr_loss, curr_gain }| {

            let prev_range = range.previous_range().get_prev_period_time_range(RSI_INTERVAL);
            let TimeRange {
                start_timestamp: prev_start_timestamp,
                end_timestamp: prev_end_timestamp,
                ..} = prev_range;

            let prev_range_candles = Candle::in_range(
                conn,
                range.period(),
                prev_start_timestamp,
                prev_end_timestamp
            );

            prev_range_candles
                .map_err(|err| {
                    format!("Error get prev range candles: {}", err)
                })
                .and_then(|prev_candles| {
                    match prev_candles.len() as i64 {
                        len if len == RSI_INTERVAL => {
                            let prev_gain_loss = calc_avg_gain_loss(prev_candles);
                            match prev_gain_loss {
                                Ok(GainLoss{ avg_gain: prev_gain, avg_loss: prev_loss, .. }) => {
                                    let relative_strength = (prev_gain * 13f32 + curr_gain) / (prev_loss * 13f32 + curr_loss);
                                    Ok(relative_strength)
                                },
                                Err(err) => {
                                    info!("Not enough in RSI prev range. Treating as start");
                                    Ok(avg_gain / avg_loss)
                                }
                            }
                        },
                        len => {
                            info!("Not enough in RSI prev range. Treating as start");
                            Ok(avg_gain / avg_loss)
                        }
                    }
                })
                .and_then(|relative_strength| {
                    let rsi = (100f32) - (100f32 / (1f32 + relative_strength));
                    Ok(rsi)
                })
        })
}

pub fn calc_avg_gain_loss(candles: Vec<Candle>) -> Result<GainLoss, String> {
    match candles.len() as i64 {
        len if len == RSI_INTERVAL => {
            let mut diff: [f32; 13] = [0.0f32; 13];

            for inx in 1..(candles.len() - 1) {
                diff[inx] = candles[inx].close - candles[inx - 1].close;
            }

            let mut avg_gain: f32 = diff
                .into_iter()
                .filter(|x| **x > 0f32)
                .sum();
            avg_gain = avg_gain / 14f32;

            let mut avg_loss: f32 = diff
                .into_iter()
                .filter(|x| **x < 0f32)
                .map(|x| x * -1f32)
                .sum();
            avg_loss = avg_loss / 14f32;

            let curr_diff = candles[candles.len() -1].close - candles[candles.len() -2].close;
            let curr_gain = if curr_diff > 0f32 { curr_diff } else { 0f32 };
            let curr_loss = if curr_diff > 0f32 { 0f32 } else { curr_diff };

            Ok(GainLoss {
                curr_loss: curr_loss,
                curr_gain: curr_gain,
                avg_gain: avg_gain,
                avg_loss: avg_loss
            })
        },
        len => Err(format!("Not enough data. Expected {}, got {}", RSI_INTERVAL, len))
    }
}
