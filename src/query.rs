/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParaForQueryByID {
    pub id: u128,
    pub meta: String,
}