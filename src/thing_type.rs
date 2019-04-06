use crate::error::NatureError;
use crate::Result;

/// Every `Thing` must have a type
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub enum ThingType {
    Business,
    System,
    Dynamic,
    Null,
}

impl ThingType {
    pub fn get_prefix(&self) -> String {
        match self {
            ThingType::Business => "/B".to_string(),
            ThingType::System => "/S".to_string(),
            ThingType::Dynamic => "/D".to_string(),
            ThingType::Null => "/N".to_string(),
        }
    }

    pub fn from_prefix(prefix: &str) -> Result<Self> {
        match prefix {
            "/B" => Ok(ThingType::Business),
            "/S" => Ok(ThingType::System),
            "/D" => Ok(ThingType::Dynamic),
            "/N" => Ok(ThingType::Null),
            _ => Err(NatureError::VerifyError("unknow prefix : [".to_string() + prefix + "]"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_profix() {
        assert_eq!("/N", ThingType::Null.get_prefix());
        assert_eq!("/S", ThingType::System.get_prefix());
        assert_eq!("/D", ThingType::Dynamic.get_prefix());
        assert_eq!("/B", ThingType::Business.get_prefix());
    }

    #[test]
    fn from_profix() {
        assert_eq!(ThingType::Null, ThingType::from_prefix("/N").unwrap());
        assert_eq!(ThingType::Business, ThingType::from_prefix("/B").unwrap());
        assert_eq!(ThingType::System, ThingType::from_prefix("/S").unwrap());
        assert_eq!(ThingType::Dynamic, ThingType::from_prefix("/D").unwrap());
        assert_eq!(Err(NatureError::VerifyError("unknow prefix : [/d]".to_string())), ThingType::from_prefix("/d"));
    }
}