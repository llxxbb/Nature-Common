use std::str::FromStr;

use error::NatureError;
use Result;

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

pub struct DynamicConverter {
    /// Only `Dynamic` target support for security reason.
    pub to: Option<String>,
    /// REST api for convert to `to`
    pub fun: Executor,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Ord, PartialOrd, Eq)]
pub enum Protocol {
    LocalRust,
    Http,
    Https,
}

impl FromStr for Protocol {
    type Err = NatureError;

    fn from_str(s: &str) -> Result<Self> {
        let cmp = &*s.to_uppercase();
        match cmp {
            "LOCALRUST" => Ok(Protocol::LocalRust),
            "HTTP" => Ok(Protocol::Http),
            "HTTPS" => Ok(Protocol::Https),
            _ => {
                let msg = format!("unknown protocol : {}", s);
                Err(NatureError::VerifyError(msg))
            }
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::LocalRust
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct Executor {
    pub protocol: Protocol,
    /// url do not contain's protocol define
    pub url: String,
}
