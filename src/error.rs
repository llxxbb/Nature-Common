use std;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use actix::prelude::SendError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NatureError {
    VerifyError(String),
    LogicalError(String),
    DaoDuplicated(String),
    SystemError(String),
    EnvironmentError(String),
}

impl Error for NatureError {}

impl Display for NatureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<serde_json::error::Error> for NatureError {
    fn from(e: serde_json::error::Error) -> Self {
        NatureError::VerifyError(e.to_string())
    }
}

impl From<std::num::ParseIntError> for NatureError {
    fn from(e: std::num::ParseIntError) -> Self {
        NatureError::VerifyError(e.to_string())
    }
}

impl<T> From<SendError<T>> for NatureError {
    fn from(err: SendError<T>) -> Self {
        NatureError::EnvironmentError(err.to_string())
    }
}

impl From<reqwest::Error> for NatureError {
    fn from(err: reqwest::Error) -> Self {
        NatureError::EnvironmentError(err.to_string())
    }
}

impl From<std::io::Error> for NatureError {
    fn from(err: std::io::Error) -> Self {
        NatureError::EnvironmentError(err.to_string())
    }
}

impl From<r2d2::Error> for NatureError {
    fn from(err: r2d2::Error) -> Self {
        NatureError::EnvironmentError(err.to_string())
    }
}
