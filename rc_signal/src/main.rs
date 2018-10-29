extern crate chrono;
extern crate curl;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate config as std_config;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate csv;

use self::diesel::prelude::*;

use std_config::{File, FileFormat, Config};
use std::{thread, time};

extern crate rc_schema;
pub mod models;
pub mod config;
pub mod timestamp;
pub mod feed;
pub mod engine;
pub mod analysis_range;
pub mod test_setup;

const CONFIG_FILE_NAME: &str = "config";

fn get_config() -> Result<config::Config, std_config::ConfigError> {
    let mut default_config = Config::default();
    default_config
        .merge(File::new(CONFIG_FILE_NAME, FileFormat::Json))
        .and_then(|merged| merged.clone().try_into::<config::Config>())
}

pub fn connect_postgres(config: &config::Config) -> PgConnection {
    let connection_string = format!("postgres://postgres:{}@{}:{}/{}",
        config.postgres.password,
        config.postgres.host,
        config.postgres.port,
        config.postgres.database
        );

    PgConnection::establish(&connection_string).expect(&format!("Error connecting to {}", &connection_string))
}

fn main() {
    pretty_env_logger::init();
    info!("rc_signal starting");

    let config_result = get_config();
    if let Err(err) = config_result {
        error!("Config not loaded - {}.", err);
        return;
    }
    let config = config_result.unwrap();

    let conn = connect_postgres(&config);

    let mut engine = engine::Engine::new(&conn);
    models::CandlePeriod::init(&conn);
    loop {
        let wait_duration = time::Duration::from_secs(5);
        thread::sleep(wait_duration);
        engine.process();
    }
}
