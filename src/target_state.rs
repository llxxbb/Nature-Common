/// used for converter setting
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TargetState {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub remove: Option<Vec<String>>,
}