/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParaForQueryByID {
    pub id: u128,
    pub meta: String,
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