use crate::{FromInstance, Instance, is_default, is_one, one};

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParaForQueryByID {
    pub id: u128,
    pub meta: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub para: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
    #[serde(skip_serializing_if = "is_one")]
    #[serde(default = "one")]
    pub limit: i32,
}

impl ParaForQueryByID {
    pub fn new(id: u128, meta: &str, para: &str, state_version: i32) -> Self {
        ParaForQueryByID {
            id,
            meta: meta.to_string(),
            para: para.to_string(),
            state_version,
            limit: 1,
        }
    }
}

impl From<&Instance> for ParaForQueryByID {
    fn from(input: &Instance) -> Self {
        ParaForQueryByID {
            id: input.id,
            meta: input.meta.to_string(),
            para: input.para.to_string(),
            state_version: input.state_version,
            limit: 1,
        }
    }
}

impl From<&FromInstance> for ParaForQueryByID {
    fn from(input: &FromInstance) -> Self {
        ParaForQueryByID {
            id: input.id,
            meta: input.meta.to_string(),
            para: input.para.to_string(),
            state_version: input.state_version,
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
    pub from_para: String,

}