use crate::{is_one, is_zero, one};

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParaForQueryByID {
    pub id: u128,
    pub meta: String,
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub state_version_from: i32,
    #[serde(skip_serializing_if = "is_one")]
    #[serde(default = "one")]
    pub limit: i32,
}

impl ParaForQueryByID {
    pub fn new(id: u128, meta: &str) -> Self {
        ParaForQueryByID {
            id,
            meta: meta.to_string(),
            state_version_from: 0,
            limit: 1,
        }
    }
}

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParaForIDAndFrom {
    pub id: u128,
    pub meta: String,
    pub from_id: u128,
    pub from_meta: String,
    pub from_state_version: i32,

}