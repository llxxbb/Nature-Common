use crate::{FromInstance, Instance, is_default, is_one, one, SEPARATOR_INS_KEY};

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyCondition {
    pub id: String,
    pub meta: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub gt_key: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub para: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
    #[serde(skip_serializing_if = "is_one")]
    #[serde(default = "one")]
    pub limit: i32,
}

impl KeyCondition {
    pub fn new(id: u128, meta: &str, para: &str, state_version: i32) -> Self {
        KeyCondition {
            id: format!("{:x}", id),
            meta: meta.to_string(),
            gt_key: "".to_string(),
            para: para.to_string(),
            state_version,
            limit: 1,
        }
    }
    pub fn id_like(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}%", self.meta, sep)
    }
    pub fn para_like(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{}{}%", self.meta, sep, self.id, sep)
    }
    pub fn get_key(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{}{}{}", self.meta, sep, self.id, sep, self.para)
    }
}

impl From<&Instance> for KeyCondition {
    fn from(input: &Instance) -> Self {
        KeyCondition {
            id: format!("{:x}", input.id),
            meta: input.meta.to_string(),
            gt_key: "".to_string(),
            para: input.para.to_string(),
            state_version: input.state_version,
            limit: 1,
        }
    }
}

impl From<&FromInstance> for KeyCondition {
    fn from(input: &FromInstance) -> Self {
        KeyCondition {
            id: format!("{:x}", input.id),
            meta: input.meta.to_string(),
            gt_key: "".to_string(),
            para: input.para.to_string(),
            state_version: input.state_version,
            limit: 1,
        }
    }
}

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IDAndFrom {
    pub id: u128,
    pub meta: String,
    pub from_key: String,
}

impl IDAndFrom {
    pub fn para_like(&self) -> String {
        format!("{}|{:x}|%", self.meta, self.id)
    }
}

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QueryByMeta {
    pub meta: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub para_like: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time_gt: Option<i64>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time_ge: Option<i64>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time_desc: bool,

}