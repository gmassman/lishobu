use std::{fmt, env};

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
