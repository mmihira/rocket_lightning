#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub user_name: String,
    pub password: String,
    pub port: i32,
    pub host: String,
    pub no_attempt_reconnect_startup: i32
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub postgres: Postgres
}

