use crate::error::LSBError;
use env_logger::{Builder, Env};
use std::env;

pub struct Conf {
    pub server_address: String,
    pub pg_conn: String,
}


pub fn get_config() -> Result<Conf, LSBError> {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    Ok(Conf {
        server_address: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into()),
        pg_conn: env::var("PG_CONN")?
    })
}
