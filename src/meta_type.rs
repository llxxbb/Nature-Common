use crate::error::NatureError;
use crate::Result;

/// Every `Thing` must have a type
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub enum MetaType {
    Business,
    System,
    Dynamic,
    Null,
}

impl MetaType {
    pub fn get_prefix(&self) -> String {
        match self {
            MetaType::Business => "/B".to_string(),
            MetaType::System => "/S".to_string(),
            MetaType::Dynamic => "/D".to_string(),
            MetaType::Null => "/N".to_string(),
        }
    }

    pub fn from_prefix(prefix: &str) -> Result<Self> {
        match prefix {
            "/B" => Ok(MetaType::Business),
            "/S" => Ok(MetaType::System),
            "/D" => Ok(MetaType::Dynamic),
            "/N" => Ok(MetaType::Null),
            _ => Err(NatureError::VerifyError("unknow prefix : [".to_string() + prefix + "]"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_profix() {
        assert_eq!("/N", MetaType::Null.get_prefix());
        assert_eq!("/S", MetaType::System.get_prefix());
        assert_eq!("/D", MetaType::Dynamic.get_prefix());
        assert_eq!("/B", MetaType::Business.get_prefix());
    }

    #[test]
    fn from_profix() {
        assert_eq!(MetaType::Null, MetaType::from_prefix("/N").unwrap());
        assert_eq!(MetaType::Business, MetaType::from_prefix("/B").unwrap());
        assert_eq!(MetaType::System, MetaType::from_prefix("/S").unwrap());
        assert_eq!(MetaType::Dynamic, MetaType::from_prefix("/D").unwrap());
        assert_eq!(Err(NatureError::VerifyError("unknow prefix : [/d]".to_string())), MetaType::from_prefix("/d"));
    }
}