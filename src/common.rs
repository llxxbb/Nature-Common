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
    DaoDuplicated,
    R2D2Error(String),
    SystemError(String),
}
