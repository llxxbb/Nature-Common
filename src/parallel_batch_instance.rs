use crate::{Instance, Thing};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ParallelBatchInstance {
    pub thing: Thing,
    pub instances: Vec<Instance>,
}
