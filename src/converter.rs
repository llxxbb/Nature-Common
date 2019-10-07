use std::str::FromStr;

use crate::{Result, SelfRouteInstance};
use crate::error::NatureError;

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
    /// return instances
    Instances(Vec<Instance>),
    /// return `SelfRouteInstance`
    SelfRoute(Vec<SelfRouteInstance>),
    /// return mixed result
    Mixed((Vec<Instance>, Vec<SelfRouteInstance>)),
}

#[derive(Serialize, Deserialize)]
pub struct ConverterParameter {
    pub from: Instance,
    pub last_state: Option<Instance>,
    /// This is used for callback
    pub carrier_id: Vec<u8>,
}

pub trait ConverterTrait {
    fn convert(para: ConverterParameter) -> ConverterReturned;
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DynamicConverter {
    /// Only `Dynamic` and `Null` target supported for security reason.
    pub to: Option<String>,
    /// REST api for convert to `to`
    pub fun: Executor,
    /// use upstream's id as downstream's id.
    pub use_upstream_id: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Ord, PartialOrd, Eq, Hash)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct Executor {
    pub protocol: Protocol,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub url: String,
    // many different Executor can reside in a group. which control the executor's opportunity to be executed
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub group: String,
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub proportion: u32,
}

impl Executor {
    pub fn for_local(path: &str) -> Self {
        Executor {
            protocol: Protocol::LocalRust,
            url: path.to_string(),
            group: "".to_string(),
            proportion: 0,
        }
    }
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(num: &u32) -> bool {
    *num == 0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde_executor_with_option_weight() {
        let mut ewe = Executor {
            protocol: Protocol::LocalRust,
            url: "".to_string(),
            group: "".to_string(),
            proportion: 0,
        };
        let ewe_s = serde_json::to_string(&ewe).unwrap();
        assert_eq!(ewe_s, "{\"protocol\":\"LocalRust\"}");
        ewe.proportion = 5;
        let ewe_s = serde_json::to_string(&ewe).unwrap();
        assert_eq!(ewe_s, "{\"protocol\":\"LocalRust\",\"proportion\":5}");
        let ewe_dw: Executor = serde_json::from_str(&ewe_s).unwrap();
        assert_eq!(ewe, ewe_dw);
        let ewe_s = "{\"protocol\":\"LocalRust\"}";
        let ewe_dw: Executor = serde_json::from_str(&ewe_s).unwrap();
        assert_eq!(ewe_dw.proportion, 0);
    }
}
