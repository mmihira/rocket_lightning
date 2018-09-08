use ::schema::candle_period;
// use ::schema::candle_period::dsl::candle_period as candle_period_dsl;
use diesel::prelude::{PgConnection};
use diesel::{ExpressionMethods, RunQueryDsl};

use analysis_range::{Period, PeriodIdentity };

#[table_name="candle_period"]
#[derive(Debug, Queryable, Insertable)]
pub struct CandlePeriod {
    pub id: i32,
    pub period_name: String
}

impl CandlePeriod {
    pub fn init(conn: &PgConnection) {
        let periods = vec![
            Self {id: Period::OneMin as i32, period_name: String::from("one_min")},
            Self {id: Period::FiveMin as i32, period_name: String::from("five_min")},
            Self {id: Period::FifteenMin as i32, period_name: String::from("fifteen_min")},
        ];

        periods
            .iter()
            .for_each(|period| { period.save_as_new(conn); });
    }

    pub fn save_as_new(&self, conn: &PgConnection) ->  Result<Self, ::diesel::result::Error> {
        ::diesel::insert_into(candle_period::table)
            .values(self)
            .get_result::<Self>(conn)
    }

    pub fn get_period(&self) -> Period {
        self.id.period()
    }
}

