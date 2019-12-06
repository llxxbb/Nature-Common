use core::fmt;
use std::collections::BTreeSet;

use serde::{de, Deserialize, Deserializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};

use crate::{Instance, is_default_meta, is_false, is_one_u32, Meta, MetaType, NatureError, PATH_SEPARATOR, Result};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
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

#[derive(Debug, Clone, Serialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct MultiMetaSetting {
    /// if does not set, use meta's key property.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prefix: String,
    #[serde(skip_serializing_if = "is_one_u32")]
    pub version: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    #[serde(skip_serializing_if = "is_default_meta")]
    pub meta_type: MetaType,
    #[serde(skip_serializing_if = "String::is_empty")]
    parent_key: String,
    #[serde(skip_serializing)]
    metas: Vec<Meta>,
    #[serde(skip_serializing)]
    meta_set: BTreeSet<String>,
}

impl MultiMetaSetting {
    pub fn new(parent: &str, prefix: &str, version: u32, keys: Vec<String>, meta_type: MetaType) -> Result<Self> {
        let metas = {
            let prefix = if prefix.is_empty() {
                parent.to_string()
            } else {
                prefix.to_string()
            };
            let mut rtn: Vec<Meta> = Vec::new();
            for k in &keys {
                let key = format!("{}{}{}", prefix, PATH_SEPARATOR, k);
                let m = Meta::new(&key, version, meta_type.clone())?;
                rtn.push(m);
            }
            rtn
        };
        let mut meta_set: BTreeSet<String> = BTreeSet::new();
        metas.iter().for_each(|one| { meta_set.insert(one.meta_string()); });
        let rtn = MultiMetaSetting {
            prefix: prefix.to_string(),
            version,
            keys: keys.clone(),
            meta_type: meta_type.clone(),
            metas,
            parent_key: parent.to_string(),
            meta_set,
        };
        Ok(rtn)
    }
    pub fn get_metas(&self) -> Vec<Meta> {
        self.metas.clone()
    }

    pub fn check_metas(&self, instances: &Vec<Instance>) -> Result<()> {
        let option = instances.iter().find(|one| !self.meta_set.contains(&one.meta));
        match option {
            Some(e) => Err(NatureError::VerifyError(format!("undefined meta: {}", e.meta))),
            None => Ok(())
        }
    }
}

impl<'de> Deserialize<'de> for MultiMetaSetting {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        enum Field { Prefix, Version, Keys, MetaType, ParentKey };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`prefix` or `version` or `keys` or `meta_type` or `metas` or `parent_key`")
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<Field, E>
                        where
                            E: de::Error,
                    {
                        match value {
                            "prefix" => Ok(Field::Prefix),
                            "version" => Ok(Field::Version),
                            "keys" => Ok(Field::Keys),
                            "meta_type" => Ok(Field::MetaType),
                            "parent_key" => Ok(Field::ParentKey),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = MultiMetaSetting;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MultiMetaSetting")
            }

            fn visit_seq<V>(self, mut seq: V) -> std::result::Result<MultiMetaSetting, V::Error>
                where
                    V: SeqAccess<'de>,
            {
                let prefix = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut version = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                if version == 0 {
                    version = 1;
                }
                let keys = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let meta_type = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let parent_key: String = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                match MultiMetaSetting::new(&parent_key, prefix, version, keys, meta_type) {
                    Ok(x) => Ok(x),
                    Err(err) => Err(Error::custom(err.to_string()))
                }
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MultiMetaSetting, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut prefix = None;
                let mut version = None;
                let mut keys = None;
                let mut meta_type = None;
                let mut parent_key = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Prefix => {
                            if prefix.is_some() {
                                return Err(de::Error::duplicate_field("prefix"));
                            }
                            prefix = Some(map.next_value()?);
                        }
                        Field::Version => {
                            if version.is_some() {
                                return Err(de::Error::duplicate_field("version"));
                            }
                            version = Some(map.next_value()?);
                        }
                        Field::Keys => {
                            if keys.is_some() {
                                return Err(de::Error::duplicate_field("keys"));
                            }
                            keys = Some(map.next_value()?);
                        }
                        Field::MetaType => {
                            if meta_type.is_some() {
                                return Err(de::Error::duplicate_field("meta_type"));
                            }
                            meta_type = Some(map.next_value()?);
                        }
                        Field::ParentKey => {
                            if parent_key.is_some() {
                                return Err(de::Error::duplicate_field("parent_key"));
                            }
                            parent_key = Some(map.next_value()?);
                        }
                    }
                }
                let prefix = match prefix {
                    Some(x) => x,
                    None => ""
                };
                let version = match version {
                    Some(x) => x,
                    None => 1
                };
                let keys = match keys {
                    Some(x) => x,
                    None => vec![]
                };
                let meta_type = match meta_type {
                    Some(x) => x,
                    None => MetaType::default()
                };
                let parent_key = match parent_key {
                    Some(x) => x,
                    None => "".to_string()
                };
                match MultiMetaSetting::new(&parent_key, prefix, version, keys, meta_type) {
                    Ok(x) => Ok(x),
                    Err(err) => Err(Error::custom(err.to_string()))
                }
            }
        }

        const FIELDS: &'static [&'static str] = &["prefix", "version", "keys", "meta_type", "parent_type"];
        deserializer.deserialize_struct("MultiMetaSetting", FIELDS, DurationVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_meta_test() {
        let setting = MultiMetaSetting::new("parent", "", 2, vec!["a".to_string(), "b".to_string()], Default::default()).unwrap();
        let mut ins = Instance::new("parent").unwrap();
        let instances: Vec<Instance> = vec![ins.clone()];
        assert_eq!(setting.check_metas(&instances).is_err(), true);
        ins.meta = "/B/parent/a:2".to_string();
        let instances: Vec<Instance> = vec![ins.clone()];
        let result = setting.check_metas(&instances);
        assert_eq!(result.is_ok(), true);
        ins.meta = "/B/parent/b:2".to_string();
        let instances: Vec<Instance> = vec![ins.clone()];
        assert_eq!(setting.check_metas(&instances).is_ok(), true);
        ins.meta = "/B/parent/c:2".to_string();
        let instances: Vec<Instance> = vec![ins.clone()];
        assert_eq!(setting.check_metas(&instances).is_err(), true);
    }

    #[test]
    fn get_metas_test() {
        let setting = MultiMetaSetting::new("parent", "", 2, vec!["a".to_string(), "b".to_string()], Default::default()).unwrap();
        let rtn = setting.get_metas();
        assert_eq!(rtn.len(), 2);
        assert_eq!(rtn[0], Meta::from_full_key("/B/parent/a", 2).unwrap());
        assert_eq!(rtn[1], Meta::from_full_key("/B/parent/b", 2).unwrap());
    }

    #[test]
    fn get_metas_with_prefix_test() {
        let setting = MultiMetaSetting::new("/P/parent", "p", 2, vec!["a".to_string(), "b".to_string()], Default::default()).unwrap();
        let rtn = setting.get_metas();
        assert_eq!(rtn.len(), 2);
        assert_eq!(rtn[0], Meta::from_full_key("/B/p/a", 2).unwrap());
        assert_eq!(rtn[1], Meta::from_full_key("/B/p/b", 2).unwrap());
    }

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
        let multi_meta = MultiMetaSetting::new("p", "", 1, vec!["one".to_string(), "two".to_string()], Default::default());

        let setting = MetaSetting {
            is_state: false,
            master: None,
            multi_meta: Some(multi_meta.unwrap()),
            conflict_avoid: false,
        };
        let result = serde_json::to_string(&setting).unwrap();
        let json = r#"{"multi_meta":{"keys":["one","two"],"parent_key":"p"}}"#;
        assert_eq!(result, json);
        let rtn: MetaSetting = serde_json::from_str(json).unwrap();
        assert_eq!(rtn, setting);
    }
}
