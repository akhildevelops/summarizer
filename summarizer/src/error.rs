use anyhow::Error as anyhowError;
use sqlx::Error as Sqlxerror;
use std::env::VarError;
use std::io::Error as Ioerror;
use std::{error::Error, fmt::Display};
#[derive(Debug)]
pub enum Serror {
    Youtubefetch(String),
    Scheduler(String),
    Database(String),
    Environment(String),
    Other(String),
}

impl Display for Serror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Youtubefetch(x) => write!(f, "Youtubefetch: {}", x),
            Self::Scheduler(x) => write!(f, "Scheduler: {}", x),
            Self::Database(x) => write!(f, "Database: {}", x),
            Self::Environment(x) => write!(f, "Environment {}", x),
            Self::Other(x) => write!(f, "Other: {}", x),
        }
    }
}

impl Error for Serror {}

impl From<Ioerror> for Serror {
    fn from(value: Ioerror) -> Self {
        Self::Scheduler(value.to_string())
    }
}

impl From<Sqlxerror> for Serror {
    fn from(value: Sqlxerror) -> Self {
        Self::Database(value.to_string())
    }
}

impl From<VarError> for Serror {
    fn from(value: VarError) -> Self {
        Self::Environment(value.to_string())
    }
}

impl From<anyhowError> for Serror {
    fn from(value: anyhowError) -> Self {
        Self::Other(value.to_string())
    }
}
