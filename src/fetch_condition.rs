#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct FetchCondition {
    pub full_key: Option<String>,
    pub from: Option<String>,
    pub size: u16,
}