use std::{fmt, env, io};

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

impl From<env::VarError> for LSBError {
    fn from(error: env::VarError) -> Self {
        Self {
            kind: String::from("env"),
            message: error.to_string(),
        }
    }
}

impl From<sqlx::Error> for LSBError {
    fn from(error: sqlx::Error) -> Self {
        Self {
            kind: String::from("db"),
            message: error.to_string(),
        }
    }
}

impl From<io::Error> for LSBError {
    fn from(error: io::Error) -> Self {
        Self {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}
