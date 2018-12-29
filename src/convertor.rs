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
    pub fun: ExecutorWithOptionWeight,
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
    /// weight in a certain `group`. if `group` is `None` then weight only take effect at the `OneStepFlow` which that `Executor` lived in.
    pub weight: Weight,
}

impl From<(ExecutorWithOptionWeight, String)> for Executor {
    fn from(e: (ExecutorWithOptionWeight, String)) -> Self {
        let group: String;
        let proportion: u32;
        match e.0.weight {
            None => {
                group = e.1;
                proportion = 1;
            }
            Some(se) => {
                match se.group {
                    None => group = e.1,
                    Some(g) => group = g
                }
                proportion = se.proportion;
            }
        }
        Executor {
            protocol: e.0.protocol,
            url: e.0.url,
            weight: Weight {
                group,
                proportion,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct ExecutorWithOptionWeight {
    pub protocol: Protocol,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub weight: Option<WeightWithOptionGroup>,
}

/// used to gray deploy
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct Weight {
    /// The weight will be share at the same `group` between `OneStepFlow`
    pub group: String,
    /// indicate the proportion of the whole stream, the whole will the sum of the participate `Weight::proportion`
    pub proportion: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct WeightWithOptionGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub proportion: u32,
}

fn is_zero(num: &u32) -> bool {
    *num == 0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde_executor_with_option_weight() {
        let mut ewe = ExecutorWithOptionWeight {
            protocol: Protocol::LocalRust,
            url: "".to_string(),
            weight: Some(WeightWithOptionGroup {
                group: None,
                proportion: 0,
            }),
        };
        let ewe_s = serde_json::to_string(&ewe).unwrap();
        assert_eq!(ewe_s, "{\"protocol\":\"LocalRust\",\"weight\":{}}");
        ewe.weight = Some(WeightWithOptionGroup {
            group: None,
            proportion: 5,
        });
        let ewe_s = serde_json::to_string(&ewe).unwrap();
        assert_eq!(ewe_s, "{\"protocol\":\"LocalRust\",\"weight\":{\"proportion\":5}}");
        let ewe_dw: ExecutorWithOptionWeight = serde_json::from_str(&ewe_s).unwrap();
        assert_eq!(ewe, ewe_dw);
        let ewe_s = "{\"protocol\":\"LocalRust\",\"weight\":{}}";
        let ewe_dw: ExecutorWithOptionWeight = serde_json::from_str(&ewe_s).unwrap();
        assert_eq!(ewe_dw.weight.unwrap().proportion, 0);
    }
}
