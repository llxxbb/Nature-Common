use super::Instance;

pub enum ConverterReturned {
    Error(String),
    Delay(u32),
    Instances(Vec<Instance>),
}

pub struct CallOutParameter {
    pub from: Instance,
    pub last_status: Option<Instance>,
    /// This is used for callback
    pub carrier_id: u128,
}

pub trait ConverterTrait {
    fn convert(para: CallOutParameter) -> ConverterReturned;
}