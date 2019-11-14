use crate::{ConverterReturned};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DelayedInstances {
    pub task_id: Vec<u8>,
    pub result: ConverterReturned,
}
