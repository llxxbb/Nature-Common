use crate::{Instance, is_default};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct FromInstance {
    pub id: u128,
    pub meta: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub para: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
}

impl FromInstance {
    pub fn get_upstream(&self) -> String {
        format!("{}:{}:{}", self.meta, self.id, self.state_version)
    }
}

impl From<&Instance> for FromInstance {
    fn from(from: &Instance) -> Self {
        FromInstance {
            id: from.id,
            meta: from.meta.to_string(),
            para: from.para.clone(),
            state_version: from.state_version,
        }
    }
}