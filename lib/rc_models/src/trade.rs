use rc_schema::schema::trades;
use diesel::prelude::{PgConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::result::Error as DieselError;
use analysis_range::{ TimePeriod };

use rc_schema::schema::trades::dsl::trades as trades_dsl;
use timestamp::TimeStamp;

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize)]
pub struct Trade {
    pub id: i32,
    pub tid: i32,
    pub timestamp: TimeStamp,
    pub vol: f32,
    pub price: f32
}

#[table_name="trades"]
#[derive(Debug, Insertable, Deserialize)]
pub struct NewTrade {
    pub tid: i32,
    pub timestamp: i64,
    pub vol: f32,
    pub price: f32
}

graphql_object!(Trade: () |&self| {
    field timestamp() -> i32 { self.timestamp as i32 }
    field vol() -> String { self.vol.to_string() }
    field price() -> String { self.price.to_string() }
});

impl Trade {
    pub fn in_timestamp_range(conn: &PgConnection, start: TimeStamp, end: TimeStamp) -> Result<Vec<Self>, ::diesel::result::Error> {
        trades_dsl.filter(trades::timestamp.between(start, end))
            .order_by(trades::timestamp.asc())
            .get_results::<Self>(conn)
    }

    pub fn trades_in_range<T: TimePeriod>(conn: &PgConnection, range: &T) -> Result<Vec<Self>, ::diesel::result::Error> {
        trades_dsl.filter(trades::timestamp.between(range.range_start(), range.range_end()))
            .order_by(trades::timestamp.asc())
            .get_results::<Self>(conn)
    }

    pub fn deleteAllRecords(conn: &PgConnection) {
        ::diesel::delete(trades::table).execute(conn).unwrap();
    }
}

impl NewTrade {
    pub fn save_as_new(&self, conn: &PgConnection) ->  Result<Trade, ::diesel::result::Error> {
        ::diesel::insert_into(trades::table)
            .values(self)
            .get_result(conn)
    }
}
