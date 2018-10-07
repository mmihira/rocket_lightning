pub mod moving_avg;
pub mod rsi;

use feed;
use feed::PollTrades;
use timestamp::{TimeStamp, Conversions};
use diesel::prelude::{PgConnection};
use diesel::{Identifiable};
use diesel::result::Error as DieselError;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use models::{Trade, Candle};
use analysis_range::{TimePeriod, Period};
use analysis_range;
use std;

pub struct Engine<'a> {
    from: TimeStamp,
    conn: &'a PgConnection
}

struct UpdateTradeResults {
    trades_retrieved: i32,
    trades_saved: i32,
    trades_conflicting: i32,
    trades: Vec<Trade>
}

impl<'a> Engine<'a> {
    pub fn new(conn: &'a PgConnection) -> Engine {
        Engine {
            from: 1524985613,
            conn: conn
        }
    }

    pub fn process(&mut self) {
        let update_state = self.update_latest_trades();

        info!("\n------------------------------------------------------");
        info!("Engine feed from   : {} - {}", self.from, self.from.as_date_string());
        info!("Trades retrieved   : {:?}", update_state.trades_retrieved);
        info!("Trades saved       : {:?}", update_state.trades_saved);
        info!("Trades conflicting : {:?}", update_state.trades_conflicting);

        if update_state.trades_saved > 0 {
            let new_timestamp = update_state.trades.first().unwrap().timestamp;
            self.from = new_timestamp - 20;
        }

        self.do_range(Period::OneMin);
    }

    fn do_range(&self, period: Period) {
        let range = period.analysis_range();
        self.calculate_candle_for_range(&range);
        self.calculate_candle_for_range(&range.previous_range());
    }

    /**
     * Create a candle for the range and calculates all values.
     */
    fn calculate_candle_for_range<T: std::fmt::Debug + analysis_range::TimePeriod>(&self, range: &T) {
        self.init_candle_for_range(range)
            .map_err(|err: DieselError| {
                format!(
                    "When creating candle for period {:?}, got error = {:?}",
                    range,
                    err
                    );
            })
            .map(|mut candle| {
                candle.sma_9  = moving_avg::period_9(self.conn, range)
                    .map_err(|err| { error!("sma_9 error: {} .. continuing with default set", err); ()})
                    .unwrap_or(0f32);
                Box::new(candle) // Box it up so we only have to propagate the box
            })
            .map(|mut candle| {
                candle.sma_12 = moving_avg::period_12(self.conn, range)
                    .map_err(|err| { error!("sma_12 error: {} .. continuing with default set", err); ()})
                    .unwrap_or(0f32);
                candle
            })
            .map(|mut candle| {
                candle.sma_26 = moving_avg::period_26(self.conn, range)
                    .map_err(|err| { error!("sma_26 error: {} .. continuing with default set", err); ()})
                    .unwrap_or(0f32);
                candle
            })
            .map(|mut candle| {
                candle.ema_9 = moving_avg::ema_period_9(self.conn, range)
                    .map_err(|err| { error!("ema_9 error: {} .. continuing with default set", err); ()})
                    .unwrap_or(0f32);
                candle
            })
            .map(|mut candle| {
                candle.rsi = rsi::rsi_for_interval(self.conn, range)
                    .map_err(|err| { error!("rsi error: {} .. continuing with default set", err); ()})
                    .unwrap_or(0f32);
                candle
            })
            .and_then(|ref candle| {
                candle
                    .update(self.conn)
                    .map_err(|err: DieselError| {
                        format!(
                            "When updating candle for period {:?}, got error = {:?}",
                            range,
                            err
                            );
                    })
            })
            .map(|ref candle| {
                info!("Candle created and updated")
            })
            .map_err(|err| {
                error!("{:?}", err)
            });
    }

    /**
     * Create a candle for the range.
     * Initialising all the calculated indicators to their default values.
     */
    fn init_candle_for_range<T: std::fmt::Debug + analysis_range::TimePeriod>(&self, range: &T) -> Result<Candle, DieselError> {
        let t_in_range = Trade::in_timestamp_range(
            self.conn,
            range.range_start(),
            range.range_end()
            );

        let mut vol = 0f32;
        let mut high: f32 = 0f32;
        let mut low = 10000000000000f32;
        let mut open = 0f32;
        let mut close = 0f32;

        for trade in t_in_range.iter() {
            vol += trade.vol;
            high = match trade.price > high {
                true => trade.price,
                false => high
            };
            low = match trade.price < low {
                true => trade.price,
                false => low
            };
        }

        // If therer are no trades we should be copying values from the period before
        // Add a test to replicate this functionality
        open = match t_in_range.first() {
            Some(trade) => trade.price,
            None => 0f32
        };

        close = match t_in_range.last() {
            Some(trade) => trade.price,
            None => 0f32
        };

        let nc = Candle {
            period: range.period() as i32,
            start_time: range.range_start(),
            end_time: range.range_end(),
            open: open,
            close: close,
            high: high,
            low: low,
            vol: vol,
            rsi: 0f32,
            sma_9: 0f32,
            sma_12: 0f32,
            sma_26: 0f32,
            ema_9: 0f32,
            ema_12: 0f32,
            ema_26: 0f32
        };

        info!("Creating candle with id {:?}", nc.id());
        nc.save_or_update(self.conn)
    }

    fn update_latest_trades(&mut self) -> UpdateTradeResults {
        let new_trades_result = feed::bitfinex::Public::get_trades(self.from);
        match new_trades_result {
            Ok(new_trades) => {
                let trade_results: Vec<Result<Trade, DieselError>> = new_trades
                    .iter()
                    .map(|trade| trade.save_as_new(self.conn))
                    .collect();

                let duplicate_no: Vec<bool> = trade_results
                    .iter()
                    .filter_map(|save_result| {
                        match save_result {
                            Err(DieselError::DatabaseError(UniqueViolation, _)) => Some(true),
                            _ => None
                        }
                    })
                    .collect();

                let saved: Vec<Trade> = trade_results
                    .into_iter()
                    .filter_map(|save_result| save_result.ok())
                    .collect();

                UpdateTradeResults {
                    trades_retrieved: new_trades.len() as i32,
                    trades_saved: saved.len() as i32,
                    trades_conflicting: duplicate_no.len() as i32,
                    trades: saved
                }
            },
            Err(e) => {
                error!("Could not retrieve new trades {}", e);
                UpdateTradeResults {
                    trades_retrieved: 0,
                    trades_saved: 0,
                    trades_conflicting: 0,
                    trades: vec![]
                }
            }
        }
    }
}
