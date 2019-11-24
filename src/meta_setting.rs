#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct MetaSetting {
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_state: bool,
    /// Only useful for state-meta.
    /// A meta_string, this meta instance's id will use its master instance's id.
    /// As a target meta, if no `executor` appointed. an auto-converter will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub master: Option<String>,
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
            master: Some("hello".to_string()),
        };
        let result = serde_json::to_string(&setting).unwrap();
        assert_eq!(result, r#"{"master":"hello"}"#)
    }
}