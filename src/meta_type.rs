use crate::error::NatureError;
use crate::Result;

/// Every `Meta` must have a type
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub enum MetaType {
    Business,
    System,
    Dynamic,
    Null,
    Parallel,
    Serial,
}

impl Default for MetaType {
    fn default() -> Self {
        MetaType::Business
    }
}

impl MetaType {
    pub fn get_prefix(&self) -> String {
        match self {
            MetaType::Business => "/B".to_string(),
            MetaType::System => "/S".to_string(),
            MetaType::Dynamic => "/D".to_string(),
            MetaType::Null => "/N".to_string(),
            MetaType::Parallel => "/P".to_string(),
            MetaType::Serial => "/R".to_string(),
        }
    }

    pub fn from_prefix(prefix: &str) -> Result<Self> {
        match prefix {
            "/B" => Ok(MetaType::Business),
            "/S" => Ok(MetaType::System),
            "/D" => Ok(MetaType::Dynamic),
            "/N" => Ok(MetaType::Null),
            "/P" => Ok(MetaType::Parallel),
            "/R" => Ok(MetaType::Serial),
            _ => Err(NatureError::VerifyError("unknow prefix : [".to_string() + prefix + "]"))
        }
    }

    pub fn check_type(meta: &str, m_type: MetaType) -> Result<()> {
        let prefix = m_type.get_prefix();
        let err = format!("meta string must begin with {}/", &prefix);
        let err = Err(NatureError::VerifyError(err));
        if meta.len() < 3 {
            return err;
        }
        let x = &meta[0..2];
        if x != format!("{}", &prefix) {
            return err;
        }
        Ok(())
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
        assert_eq!("/P", MetaType::Parallel.get_prefix());
        assert_eq!("/R", MetaType::Serial.get_prefix());
    }

    #[test]
    fn from_profix() {
        assert_eq!(MetaType::Null, MetaType::from_prefix("/N").unwrap());
        assert_eq!(MetaType::Business, MetaType::from_prefix("/B").unwrap());
        assert_eq!(MetaType::System, MetaType::from_prefix("/S").unwrap());
        assert_eq!(MetaType::Dynamic, MetaType::from_prefix("/D").unwrap());
        assert_eq!(MetaType::Parallel, MetaType::from_prefix("/P").unwrap());
        assert_eq!(MetaType::Serial, MetaType::from_prefix("/R").unwrap());
        assert_eq!(Err(NatureError::VerifyError("unknow prefix : [/d]".to_string())), MetaType::from_prefix("/d"));
    }

    #[test]
    fn check_type_test() {
        assert_eq!(Ok(()), MetaType::check_type("/D//dynamic/target/is/null:1", MetaType::Dynamic));
    }
}