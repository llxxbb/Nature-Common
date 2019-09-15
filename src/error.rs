use std;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use actix::prelude::SendError;
use uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NatureError {
    // outer input verify errors
    SerializeError(String),
    VerifyError(String),
    MetaNotDefined(String),
    InstanceStatusVersionConflict,
    UuidParseError,

    // out converter errors
    ConverterLogicalError(String),
    ConverterEnvironmentError(String),
    ConverterProtocalError(String),

    // Business Contract errors
    TargetInstanceNotIncludeStatus(String),
    TargetInstanceContainsExcludeStatus(String),

    // internal errors
    DaoEnvironmentError(String),
    DaoLogicalError(String),
    DaoDuplicated(String),
    R2D2Error(String),
    SystemError(String),
    Break,

}

impl Error for NatureError {}

impl Display for NatureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl From<serde_json::error::Error> for NatureError {
    fn from(e: serde_json::error::Error) -> Self {
        NatureError::SerializeError(e.to_string())
    }
}

impl From<uuid::parser::ParseError> for NatureError {
    fn from(_e: uuid::parser::ParseError) -> Self {
        NatureError::UuidParseError
    }
}

impl From<std::num::ParseIntError> for NatureError {
    fn from(_: std::num::ParseIntError) -> Self {
        NatureError::UuidParseError
    }
}

impl<T> From<SendError<T>> for NatureError {
    fn from(err: SendError<T>) -> Self {
        NatureError::SystemError(err.to_string())
    }
}

