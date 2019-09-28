use crate::{Meta, META_AND_VERSION_SEPARATOR, MetaType, NatureError, Result};

pub struct MetaString;

impl MetaString {
    pub fn null() -> String {
        "/N".to_string()
    }

    /// version default to 1 and meta_type default to `Business`
    pub fn new(key: &str) -> Result<String> {
        Self::with_version_and_type(key, 1, MetaType::Business)
    }

    /// version default to 1
    pub fn with_type(key: &str, meta_type: MetaType) -> Result<String> {
        Self::with_version_and_type(key, 1, meta_type)
    }

    pub fn with_version_and_type(key: &str, version: i32, meta_type: MetaType) -> Result<String> {
        let prefix = meta_type.get_prefix();
        if meta_type == MetaType::Null {
            return Ok(prefix);
        }
        let key = Meta::key_standardize(key)?;
        let rtn = format!("{}{}:{}", &prefix, key, version);
        Ok(rtn)
    }

    pub fn make_meta_string(full_key: &str, version: i32) -> String {
        format!("{}:{}", full_key, version)
    }

    pub fn make_tuple_from_str(meta_str: &str) -> Result<(String, i32)> {
        let x: Vec<&str> = meta_str.split(META_AND_VERSION_SEPARATOR).collect();
        if x.len() != 2 {
            return Err(NatureError::VerifyError("error meta string format".to_string()));
        }
        Ok((x[0].to_string(), x[1].parse::<i32>()?))
    }

    pub fn full_key(key: &str) -> Result<String> {
        let key = Meta::key_standardize(key)?;
        let rtn = format!("{}{}", MetaType::Business.get_prefix(), key);
        Ok(rtn)
    }
}
