use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use juniper::{FieldResult, EmptyMutation, RootNode };
use analysis_range;
use models;
use ::juniper;
use ::r2d2;

pub struct Context {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Context: Context as "Query" |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field trade(&executor, from: String, to: String) -> FieldResult<Vec<models::Trade>> {
        let context = executor.context();
        let conn = context.pool.get()?;
        let trades = models::Trade::in_timestamp_range(
            &conn,
            from.parse::<i64>()?,
            to.parse::<i64>()?
        )?;

        Ok(trades)
    }

    // Arguments to resolvers can either be simple types or input objects.
    // The executor is a special (optional) argument that allows accessing the context.
    field candle(&executor, from: String, to: String) -> FieldResult<Vec<models::Candle>> {
        let context = executor.context();
        let conn = context.pool.get()?;
        let candle = models::Candle::in_range(
            &conn,
            analysis_range::Period::OneMin,
            from.parse::<i64>()?,
            to.parse::<i64>()?
        )?;

        Ok(candle)
    }
});

pub type Schema = juniper::RootNode<'static, Context, EmptyMutation<Context>>;
