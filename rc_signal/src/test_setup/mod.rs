use models::{Candle};
use diesel::PgConnection;
use diesel::Connection;
use pretty_env_logger;

use std_config;
use std_config::{File, FileFormat, Config};
use std::{thread, time};
use super::CONFIG_FILE_NAME;
use models;

use config;

fn get_config() -> Result<config::Config, std_config::ConfigError> {
    let mut default_config = Config::default();
    default_config
        .merge(File::new(CONFIG_FILE_NAME, FileFormat::Json))
        .and_then(|merged| merged.clone().try_into::<config::Config>())
}

pub fn connect_postgres(config: &config::Config) -> PgConnection {
    let connection_string = format!("postgres://postgres:{}@{}:{}/{}",
        config.postgres_test.password,
        config.postgres_test.host,
        config.postgres_test.port,
        config.postgres_test.database
        );

    PgConnection::establish(&connection_string).expect(&format!("Error connecting to {}", &connection_string))
}

pub fn setup() -> Result<PgConnection, String> {
    use schema::trades::dsl::*;
    pretty_env_logger::init();
    info!("setting up test");

    let config_result = get_config();
    match config_result {
        Err(err) => Err(err.to_string()),
        Ok(config) => {
            let conn = connect_postgres(&config);
            models::CandlePeriod::init(&conn);
            Ok(conn)
        }
    }
}

