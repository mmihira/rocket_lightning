use models::{Candle};
use diesel::PgConnection;
use diesel::Connection;
use diesel::result::Error as DieselError;

use std_config;
use std_config::{File, FileFormat, Config};
use std;
use serde_json;
use std::io::Read;
use std::{thread, time};
use super::CONFIG_FILE_NAME;
use models;
use config;
use csv;

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

pub fn load_from_file(con: &PgConnection) {
    let z: Result<Vec<Result<Candle, DieselError>>, String> = std::fs::File::open("./test/fixtures/rsi_candles.json")
        .map_err(|err| err.to_string())
        .map(|mut f| {
            let mut contents = String::new();
            f.read_to_string(&mut contents);
            contents
        })
        .and_then(|contents: String| {
            serde_json::from_str(&contents)
                .map(|x: Vec<Candle>| x)
                .map_err(|err| err.to_string())
        })
        .map(|candles: Vec<Candle>| {
            candles
                .into_iter()
                .map(|candle| candle.save_as_new(&con))
                .collect()
        })
        .map_err(|err| {
            println!("Error {}", err);
            err
        });
}

pub fn load_candles_from_csv(con: &PgConnection, file_name: &str) -> Result<Vec<Candle>, String> {
    std::fs::File::open(file_name)
        .map_err(|err| err.to_string())
        .map(|f| {
            csv::Reader::from_reader(f)
                .deserialize()
                .map(|x: Result<Candle, _>| x.unwrap())
                .collect()
        })
        .map(|candles: Vec<Candle>| {
            con.transaction::<_, DieselError, _>(|| {
                Candle::deleteAllRecords(&con);
                candles
                    .into_iter()
                    .map(|candle| candle.save_as_new(&con))
                    .collect()
            }).unwrap()
        })
        .map_err(|err| {
            println!("Error {}", err);
            err
        })
}

pub fn candles_from_csv(con: &PgConnection, file_name: &str) -> Result<Vec<Candle>, String> {
    std::fs::File::open(file_name)
        .map_err(|err| err.to_string())
        .map(|f| {
            csv::Reader::from_reader(f)
                .deserialize()
                .map(|x: Result<Candle, _>| x.unwrap())
                .collect()
        })
        .map_err(|err| {
            println!("Error {}", err);
            err
        })
}
