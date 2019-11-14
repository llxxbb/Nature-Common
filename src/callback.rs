use crate::{ConverterReturned};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DelayedInstances {
    pub carrier_id: Vec<u8>,
    pub result: ConverterReturned,
}
