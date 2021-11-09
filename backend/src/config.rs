use crate::error::LSBError;
use std::convert::TryInto;
use std::env;

pub type Result<T, E = LSBError> = std::result::Result<T, E>;

pub struct Conf {
    pub server_address: String,
    pub pg_conn: String,
    pub rust_env: String,
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
        cookie_secret,
    })
}
