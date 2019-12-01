use crate::{is_default_meta, is_one_u32, MetaType, one_u32};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multi_meta: Option<MultiMetaSetting>,
    /// Nature will cache the saved instance for a while, and check before saving the following same instances.
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub conflict_avoid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct MultiMetaSetting {
    /// if does not set, use meta's key property.
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub prefix: String,
    #[serde(skip_serializing_if = "is_one_u32")]
    #[serde(default = "one_u32")]
    pub version: u32,
    pub keys: Vec<String>,
    #[serde(skip_serializing_if = "is_default_meta")]
    #[serde(default)]
    pub meta_type: MetaType,
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
    fn json_master_test() {
        let setting = MetaSetting {
            is_state: false,
            master: Some("hello".to_string()),
            multi_meta: None,
            conflict_avoid: false,
        };
        let result = serde_json::to_string(&setting).unwrap();
        assert_eq!(result, r#"{"master":"hello"}"#)
    }

    #[test]
    fn json_multi_meta_test() {
        let setting = MetaSetting {
            is_state: false,
            master: None,
            multi_meta: Some(MultiMetaSetting {
                prefix: "".to_string(),
                version: 1,
                keys: vec!["one".to_string(), "two".to_string()],
                meta_type: Default::default(),
            }),
            conflict_avoid: false,
        };
        let result = serde_json::to_string(&setting).unwrap();
        assert_eq!(result, r#"{"multi_meta":{"keys":["one","two"]}}"#)
    }
}