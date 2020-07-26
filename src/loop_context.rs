#[derive(Serialize, Deserialize, Debug)]
pub struct LoopContext {
    pub from: String,
    pub to: String,
    pub len: usize,
    pub page: usize,
}