#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct MetaSetting {
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_state: bool,
    /// As a downstream meta, Nature can automatic implement the instance.
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_empty_content: bool,
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
        };
        let result = serde_json::to_string(&setting).unwrap();
        assert_eq!(result, r#"{"is_empty_content":true}"#)
    }
}