#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct MetaSetting {
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_state: bool,
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(input: &bool) -> bool {
    !*input
}