use anyhow::Error as anyhowError;
use async_openai::error::OpenAIError;
use axum::extract::Json;
use axum_core::response::IntoResponse;
use regex::Error as regexError;
use reqwest::Error as reqError;
use serde::Serialize;
use serde_json::Error as serde_jsonError;
use sqlx::Error as Sqlxerror;
use std::env::VarError;
use std::io::Error as Ioerror;
use std::{error::Error, fmt::Display};
#[derive(Debug, Serialize)]
pub enum Serror {
    Youtubefetch(String),
    Scheduler(String),
    Database(String),
    Environment(String),
    Other(String),
    OpenAIError(String),
    Tokenize(String),
    Communication(String),
}

impl IntoResponse for Serror {
    fn into_response(self) -> axum_core::response::Response {
        Json(self).into_response()
    }
}

impl Display for Serror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Youtubefetch(x) => write!(f, "Youtubefetch: {}", x),
            Self::Scheduler(x) => write!(f, "Scheduler: {}", x),
            Self::Database(x) => write!(f, "Database: {}", x),
            Self::Environment(x) => write!(f, "Environment {}", x),
            Self::Other(x) => write!(f, "Other: {}", x),
            Self::OpenAIError(x) => write!(f, "Openai: {}", x),
            Self::Tokenize(x) => writeln!(f, "Tokenize: {}", x),
            Self::Communication(x) => writeln!(f, "Request: {}", x),
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

impl From<OpenAIError> for Serror {
    fn from(value: OpenAIError) -> Self {
        Self::OpenAIError(value.to_string())
    }
}

impl From<serde_jsonError> for Serror {
    fn from(value: serde_jsonError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<reqError> for Serror {
    fn from(value: reqError) -> Self {
        Self::Communication(value.to_string())
    }
}

impl From<regexError> for Serror {
    fn from(value: regexError) -> Self {
        Self::Other(value.to_string())
    }
}
