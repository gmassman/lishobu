use anyhow::Result;
use std::env;

pub struct Conf {
    pub server_address: String,
    pub pg_conn: String,
    pub rust_env: String,
}

pub fn get_config() -> Result<Conf> {
    Ok(Conf {
        server_address: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into()),
        pg_conn: env::var("PG_CONN")?,
        rust_env: env::var("RUST_ENV").unwrap_or_else(|_| "development".into()),
    })
}
