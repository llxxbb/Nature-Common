use super::Instance;

pub enum ConverterReturned {
    /// This will break process for ever.
    LogicalError(String),
    /// This can quick finish the process, and retry later.
    EnvError,
    /// No instance would be return.
    None,
    /// Tell `Nature` the task will be processed asynchronously, and it will callback to `Nature` later will result are ready.
    Delay(u32),
    /// Give result to `Nature`
    Instances(Vec<Instance>),
}

pub struct CallOutParameter {
    pub from: Instance,
    pub last_status: Option<Instance>,
    /// This is used for callback
    pub carrier_id: Vec<u8>,
}

pub trait ConverterTrait {
    fn convert(para: CallOutParameter) -> ConverterReturned;
}