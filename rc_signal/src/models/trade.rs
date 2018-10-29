use rc_schema::schema::trades;
use diesel::prelude::{PgConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::result::Error as DieselError;

use rc_schema::schema::trades::dsl::trades as trades_dsl;
use timestamp::TimeStamp;

#[derive(Debug, Identifiable, Queryable, Deserialize)]
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

impl Trade {
    pub fn in_timestamp_range(conn: &PgConnection, start: TimeStamp, end: TimeStamp) -> Vec<Self> {
        trades_dsl.filter(trades::timestamp.between(start, end))
            .order_by(trades::timestamp.asc())
            .get_results::<Self>(conn)
            .unwrap()
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


