use std::error::Error;
use std::{env, fmt, io};

pub type Result<T, E = LSBError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct LSBError {
    kind: String,
    message: String,
}

impl fmt::Display for LSBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LSBError {}

impl LSBError {
    pub fn new(kind: String, message: String) -> Self {
        Self { kind, message }
    }
}

macro_rules! lsberror_from {
    ($base_err:ty, $kind:literal) => {
        impl From<$base_err> for LSBError {
            fn from(error: $base_err) -> Self {
                Self {
                    kind: String::from($kind),
                    message: error.to_string(),
                }
            }
        }
    };
}

lsberror_from!(env::VarError, "env");
lsberror_from!(sqlx::Error, "db");
lsberror_from!(io::Error, "io");
lsberror_from!(actix_web::Error, "actix");
