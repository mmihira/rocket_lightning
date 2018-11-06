use timestamp::{TimeStamp};
use rc_schema::schema::candles;
use rc_schema::schema::candles::dsl::candles as candles_dsl;
use diesel::prelude::{PgConnection};
use diesel::{Identifiable, ExpressionMethods, QueryDsl, RunQueryDsl, BoolExpressionMethods};
use diesel::result::Error as DieselError;
use analysis_range::{ Period, PeriodIdentity, TimePeriod };

#[table_name="candles"]
#[primary_key(period, start_time, end_time)]
#[derive(Debug, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, PartialEq)]
pub struct Candle {
    pub period: i32,
    pub start_time: TimeStamp,
    pub end_time: TimeStamp,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub vol: f32,
    pub rsi_avg_gain: f32,
    pub rsi_avg_loss: f32,
    pub rsi: f32,
    pub rsi_smoothed: f32,
    pub sma_9: f32,
    pub sma_12: f32,
    pub sma_26: f32,
    pub ema_9: f32,
    pub ema_12: f32,
    pub ema_26: f32
}

graphql_object!(Candle: () |&self| {
    field period() -> i32 { self.period }
    field start_time() -> String { self.start_time.to_string() }
    field end_time() -> String { self.end_time.to_string() }
    field open() -> String { self.open.to_string() }
    field close() -> String { self.close.to_string() }
    field high() -> String { self.high.to_string() }
    field low() -> String { self.low.to_string() }
    field vol() -> String { self.vol.to_string() }
    field rsi_avg_gain() -> String { self.rsi_avg_gain.to_string() }
    field rsi_avg_loss() -> String { self.rsi_avg_loss.to_string() }
    field rsi() -> String { self.rsi.to_string() }
    field rsi_smoothed() -> String { self.rsi_smoothed.to_string() }
    field sma_9() -> String { self.sma_9.to_string() }
    field sma_12() -> String { self.sma_12.to_string() }
    field sma_26() -> String { self.sma_26.to_string() }
    field ema_9() -> String { self.ema_9.to_string() }
    field ema_12() -> String { self.ema_12.to_string() }
    field ema_26() -> String { self.ema_26.to_string() }
});

impl Candle {
    pub fn sma_for_interval(&self, interval: i64) -> f32 {
        match interval {
            9 => self.sma_9,
            12 => self.sma_12,
            26 => self.sma_26,
            _ => panic!("Not a supported inerval!")
        }
    }

    pub fn ema_for_interval(&self, interval: i64) -> f32 {
        match interval {
            9 => self.ema_9,
            12 => self.ema_12,
            26 => self.ema_26,
            _ => panic!("Not a supported inerval!")
        }
    }

    pub fn save_as_new(&self, conn: &PgConnection) ->  Result<Self, DieselError> {
        ::diesel::insert_into(candles::table)
            .values(self)
            .get_result(conn)
    }

    pub fn update(&self, conn: &PgConnection) -> Result<Self, DieselError> {
        ::diesel::update(self)
            .set(self)
            .get_result(conn)
    }

    pub fn deleteAllRecords(conn: &PgConnection) {
        ::diesel::delete(candles::table).execute(conn).unwrap();
    }

    pub fn save_or_update(&self, conn: &PgConnection) -> Result<Self, DieselError> {
        let exists = candles_dsl.find(self.id()).get_result::<Self>(conn);

        match exists {
            Ok(existing) => self.update(conn),
            Err(_) => self.save_as_new(conn)
        }
    }

    /**
     * Order by first being the latest candle
     */
    pub fn in_range(conn: &PgConnection, period: Period, start: TimeStamp, end: TimeStamp) -> Result<Vec<Self>, DieselError> {
        candles_dsl.filter(
            candles::start_time.ge(start)
                .and(candles::end_time.le(end))
                .and(candles::period.eq(period as i32))
            )
            .order_by(candles::start_time.asc())
            .get_results::<Self>(conn)
    }

    pub fn candle_for_range<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<Self, DieselError> {
        candles_dsl.filter(
            candles::start_time.eq(range.range_start())
                .and(candles::end_time.eq(range.range_end()))
                .and(candles::period.eq(range.period() as i32)))
            .get_result::<Self>(conn)
    }

    pub fn prev_candle(&self, conn: &PgConnection) -> Result<Self, DieselError> {
        let range = self.period.period().range_from(self.start_time);
        candles_dsl.filter(
            candles::start_time.eq(range.range_start())
                .and(candles::end_time.eq(range.range_end()))
                .and(candles::period.eq(self.period)))
            .get_result::<Self>(conn)
    }

    pub fn prev_candle_of_range<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<Self, DieselError> {
        let prev_range = range.previous_range();
        candles_dsl.filter(
            candles::start_time.eq(prev_range.range_start())
                .and(candles::end_time.eq(prev_range.range_end()))
                .and(candles::period.eq(prev_range.period() as i32)))
            .get_result::<Self>(conn)
    }
}
