use models::{Candle};
use analysis_range::{ TimePeriod, TimeRange };
use diesel::prelude::{PgConnection};
use diesel::result::Error as DieselError;

const RSI_INTERVAL: i64 = 14;
const RSI_INTERVAL_CALC_RANGE: i64 = RSI_INTERVAL + 1;

#[derive(Debug, PartialEq)]
pub struct GainLoss {
    avg_gain: f32,
    avg_loss: f32,
    curr_gain: f32,
    curr_loss: f32
}

pub fn candles_for_rsi_calc<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<Vec<Candle>, DieselError> {
    let period_range = range.get_prev_period_time_range(RSI_INTERVAL + 1i64);
    let TimeRange { start_timestamp, end_timestamp, ..} = period_range;

    Candle::in_range(
        conn,
        range.period(),
        start_timestamp,
        end_timestamp
    )
}

pub fn rsi_for_interval<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<f32, String> {
    candles_for_rsi_calc(conn, range)
        .map_err(|err: DieselError| {
            format!("{}", err)
        })
        .and_then(|candles| {
            match candles.len() as i64 {
                len if len == RSI_INTERVAL_CALC_RANGE => Ok(candles),
                len => Err(format!("Not enough data. Expected {}, got {}", RSI_INTERVAL, len))
            }
        })
        .and_then(|candles| calc_avg_gain_loss(candles))
        .and_then(|GainLoss{ avg_gain, avg_loss, curr_loss, curr_gain }| {
            let prev_range = range.previous_range();

            candles_for_rsi_calc(conn, &prev_range)
                .map_err(|err| {
                    format!("Error get prev range candles: {}", err)
                })
                .and_then(|prev_candles| {
                    match prev_candles.len() as i64 {
                        len if len == RSI_INTERVAL_CALC_RANGE => {
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
        len if len == RSI_INTERVAL_CALC_RANGE => {
            let mut diff: [f32; 14] = [0.0f32; 14];

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
            let curr_loss = if curr_diff > 0f32 { 0f32 } else { curr_diff * -1.0f32 };

            Ok(GainLoss {
                curr_loss: curr_loss,
                curr_gain: curr_gain,
                avg_gain: avg_gain,
                avg_loss: avg_loss
            })
        },
        len => Err(format!("Not enough data. Expected {}, got {}", RSI_INTERVAL_CALC_RANGE, len))
    }
}

#[cfg(test)]
mod tests {
    use test_setup;
    use models::{Candle};
    use diesel::result::Error as DieselError;
    use analysis_range::{OneMin, Period, TimeRange, TimePeriod};

    // The fixtures are in this range
    const ref_range: OneMin = OneMin {
        start_timestamp: 1538718120i64,
        end_timestamp: 1538718180i64,
        period: Period::OneMin,
        prior_start_timestamp: 1538718060i64,
    };

    const next_ref_range: OneMin = OneMin {
        start_timestamp: 1538718180i64,
        end_timestamp: 1538718240i64,
        period: Period::OneMin,
        prior_start_timestamp: 1538718120i64,
    };

    #[test]
    fn candles_for_rsi_calc() {
        let conn = test_setup::setup().unwrap();
        let file_name = "./test/fixtures/rsi_test.csv".to_string();
        test_setup::load_candles_from_csv(&conn, &file_name).unwrap();

        let candles_for_rsi  = super::candles_for_rsi_calc(&conn, &ref_range).unwrap();
        let candles_for_rsi_starts: Vec<i64> = candles_for_rsi.iter().map(|candle| candle.start_time).collect();

        assert_eq!(candles_for_rsi_starts, vec![
            1538717280,
            1538717340,
            1538717400,
            1538717460,
            1538717520,
            1538717580,
            1538717640,
            1538717700,
            1538717760,
            1538717820,
            1538717880,
            1538717940,
            1538718000,
            1538718060,
            1538718120
        ]);
    }

    #[test]
    fn calc_avg_gain_loss() {
        let conn = test_setup::setup().unwrap();
        let file_name = "./test/fixtures/rsi_test.csv".to_string();
        test_setup::load_candles_from_csv(&conn, &file_name);

        let candles_for_rsi  = super::candles_for_rsi_calc(&conn, &ref_range).unwrap();
        let gain_loss = super::calc_avg_gain_loss(candles_for_rsi).unwrap();
        assert_eq!(gain_loss, super::GainLoss {
            avg_gain: 0.23857144f32,
            avg_loss: 0.100000106f32,
            curr_gain: 0f32,
            curr_loss: 0f32
        });
    }

    #[test]
    fn rsi_for_interval() {
        let conn = test_setup::setup().unwrap();
        let file_name = "./test/fixtures/rsi_test.csv".to_string();
        test_setup::load_candles_from_csv(&conn, &file_name);

        let rsi = super::rsi_for_interval(&conn, &ref_range).unwrap();
        assert_eq!(rsi, 70.46411);
    }

    #[test]
    fn rsi_for_interval_average() {
        let conn = test_setup::setup().unwrap();
        let file_name = "./test/fixtures/rsi_test.csv".to_string();
        test_setup::load_candles_from_csv(&conn, &file_name).unwrap();

        let rsi = super::rsi_for_interval(&conn, &ref_range).unwrap();

        let mut found_candles = Candle::in_range(
            &conn,
            Period::OneMin,
            1538718120i64,
            1538718180i64
        ).unwrap();
        let candle_to_save = found_candles.first_mut().unwrap();
        candle_to_save.rsi = rsi;
        candle_to_save.update(&conn);

        let next_rsi = super::rsi_for_interval(&conn, &next_ref_range).unwrap();

        assert_eq!(next_rsi, 66.24962);
    }
}
