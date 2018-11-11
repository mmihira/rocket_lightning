#![feature(plugin, decl_macro, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate juniper;
#[macro_use] extern crate juniper_codegen;
#[macro_use] extern crate rocket;
#[macro_use] extern crate log;
extern crate juniper_rocket;
extern crate r2d2;
extern crate config as std_config;
extern crate rc_models as models;
extern crate analysis_range;

mod routes;
pub mod config;
mod graphql_schema;

use std_config::{File, FileFormat, Config, Environment};
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use juniper::{ EmptyMutation};

const CONFIG_FILE_NAME: &str = "config";

fn get_config() -> Result<config::Config, std_config::ConfigError> {
    let mut default_config = Config::default();
    default_config
        .merge(File::new(CONFIG_FILE_NAME, FileFormat::Json))?
        .merge(Environment::with_prefix("rc"))
        .and_then(|merged| merged.clone().try_into::<config::Config>())
}

fn get_db_pool(config: &config::Config ) -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let connection_string = format!("postgres://postgres:{}@{}:{}/{}",
        config.postgres.password,
        config.postgres.host,
        config.postgres.port,
        config.postgres.database
        );

    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(connection_string);
    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap()
}

fn main() {
    let config_result = get_config();
    if let Err(err) = config_result {
        error!("Config not loaded - {}.", err);
        return;
    }
    let config = config_result.unwrap();

    let pg_pool = get_db_pool(&config);
    rocket::ignite()
        .mount("/", routes![
            routes::graphiql,
            routes::get_graphql_handler,
            routes::post_graphql_handler,
            routes::hello
        ])
        .manage(graphql_schema::Context { pool: pg_pool.clone() })
        .manage(graphql_schema::Schema::new(
            graphql_schema::Context {pool: pg_pool.clone()},
            EmptyMutation::<graphql_schema::Context>::new(),
        ))
        .launch();
}
