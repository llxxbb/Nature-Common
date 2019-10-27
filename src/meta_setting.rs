#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct MetaSetting {
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_state: bool,
    /// As a downstream meta, Nature can automatic implement the instance.
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_empty_content: bool,
    /// A meta_string, this meta instance's id must equal protagonist instance's id.
    /// only useful for state-meta.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub belong_to: Option<String>,
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(input: &bool) -> bool {
    !*input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_test() {
        let setting = MetaSetting {
            is_state: false,
            is_empty_content: true,
            belong_to: None
        };
        let result = serde_json::to_string(&setting).unwrap();
        assert_eq!(result, r#"{"is_empty_content":true}"#)
    }
}