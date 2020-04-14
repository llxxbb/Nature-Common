use std::collections::HashSet;

/// used for converter setting
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TargetState {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remove: Option<Vec<String>>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub need_all: HashSet<String>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub need_any: HashSet<String>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub need_none: HashSet<String>,
}
