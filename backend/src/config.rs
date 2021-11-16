use crate::error::{LSBError, Result};
use std::convert::TryInto;
use std::env;

pub struct Conf {
    pub server_address: String,
    pub pg_conn: String,
    pub rust_env: String,
    pub frontend_path: String,
    pub cookie_expiry: i64,
    pub cookie_secret: Box<[u8; 32]>,
}

pub fn get_config() -> Result<Conf> {
    let cookie_secret = env::var("COOKIE_SECRET")?.into_bytes().into_boxed_slice();
    let cookie_secret: Box<[u8; 32]> = match cookie_secret.try_into() {
        Ok(array) => array,
        Err(o) => {
            return Err(LSBError::new(
                String::from("config"),
                format!(
                    "Failed to load 32 byte COOKIE_SECRET from env, got {} bytes",
                    o.len()
                ),
            ))
        }
    };

    Ok(Conf {
        server_address: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into()),
        rust_env: env::var("RUST_ENV").unwrap_or_else(|_| "development".into()),
        pg_conn: env::var("PG_CONN")?,
        frontend_path: env::var("FRONTEND_PATH")?,
        cookie_expiry: 60 * 60 * 24 * 30, // 30 days defalt
        cookie_secret,
    })
}
