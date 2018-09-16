use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NatureError {
    // outer input verify errors
    SerializeError(String),
    VerifyError(String),
    ThingNotDefined(String),
    InstanceStatusVersionConflict,
    UuidParseError,

    // out converter errors
    ConverterLogicalError(String),
    ConverterEnvironmentError(String),

    // Business Contract errors
    TargetInstanceNotIncludeStatus(String),
    TargetInstanceContainsExcludeStatus(String),

    // internal errors
    DaoEnvironmentError(String),
    DaoLogicalError(String),
    DaoDuplicated(String),
    R2D2Error(String),
    SystemError(String),
}

impl From<serde_json::error::Error> for NatureError {
    fn from(e: serde_json::error::Error) -> Self {
        NatureError::SerializeError(e.to_string())
    }
}

