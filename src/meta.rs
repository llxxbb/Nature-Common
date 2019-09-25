use crate::{MetaSetting, State};
use crate::state::States;

use super::MetaType;
use super::NatureError;
use super::Result;

/// separator for `Meta`'s key
static PATH_SEPARATOR: char = '/';
static META_AND_VERSION_SEPARATOR: &str = ":";

/// Business Metadata
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Meta {
    /// # Identify a `Meta`.
    ///
    /// A `Meta` may have a lots of `Instance`s, so it's a **Class** for Instance`.
    /// Because there are huge quantity of `Meta`s , so we need a way to organize `Meta`s.
    /// A way is to set name with hierarchical structures,
    key: String,

    /// key with `MetaType` prefix
    /// # Value Example
    ///
    /// /B/shop/order
    full_key: String,

    /// A `Meta` can be changed in future, the `version` will support this without effect the old ones
    pub version: i32,

    /// A `Meta`'s type
    meta_type: MetaType,

    pub state: Option<States>,

    pub is_state: bool,

    pub setting: Option<MetaSetting>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            key: String::new(),
            full_key: MetaType::Business.get_prefix(),
            version: 1,
            meta_type: MetaType::Business,
            state: None,
            is_state: false,
            setting: None,
        }
    }
}

impl Meta {
    /// prefix "/B(usiness)" to the head of the string,.to avoid outer use"/S(ystem)" prefix.
    fn key_standardize(biz: &mut String) -> Result<()> {
        if biz.ends_with(PATH_SEPARATOR) {
            let last = biz.len() - 1;
            biz.remove(last);
        }
        if biz.is_empty() {
            return Err(NatureError::VerifyError("key length can't be zero".to_string()));
        }
        if !biz.starts_with(PATH_SEPARATOR) {
            biz.insert(0, PATH_SEPARATOR);
        }
        Ok(())
    }

    /// version default to 1 and meta_type default to `Business`
    pub fn new(key: &str) -> Result<Self> {
        Self::new_with_version_and_type(key, 1, MetaType::Business)
    }

    /// version default to 1
    pub fn new_with_type(key: &str, meta_type: MetaType) -> Result<Self> {
        Self::new_with_version_and_type(key, 1, meta_type)
    }

    pub fn new_with_version_and_type(key: &str, version: i32, meta_type: MetaType) -> Result<Self> {
        if meta_type == MetaType::Null {
            return Ok(Self::new_null());
        }
        let mut key = key.to_string();
        match Self::key_standardize(&mut key) {
            Err(e) => Err(e),
            Ok(_) => Ok({
                Meta {
                    key: key.clone(),
                    full_key: meta_type.get_prefix() + &key,
                    version,
                    meta_type,
                    state: None,
                    is_state: false,
                    setting: None,
                }
            })
        }
    }

    pub fn new_null() -> Meta {
        let meta_type = MetaType::Null;
        Meta {
            key: String::new(),
            full_key: meta_type.get_prefix(),
            version: 1,
            meta_type,
            state: None,
            is_state: false,
            setting: None,
        }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_full_key(&self) -> String {
        self.full_key.clone()
    }

    pub fn get_meta_type(&self) -> MetaType {
        self.meta_type.clone()
    }
    pub fn set_meta_type(&mut self, meta_type: MetaType) {
        self.meta_type = meta_type.clone();
        self.full_key = meta_type.get_prefix() + &self.key.clone();
    }

    pub fn get_string(&self) -> String {
        self.full_key.clone() + META_AND_VERSION_SEPARATOR + &self.version.to_string()
    }

    /// `meta_str`'s format : [full_key]:[version]
    pub fn from_string(meta_str: &str) -> Result<Meta> {
        let x: Vec<&str> = meta_str.split(META_AND_VERSION_SEPARATOR).collect();
        if x.len() != 2 {
            return Err(NatureError::VerifyError("error meta string format".to_string()));
        }
        Self::from_full_key(x[0], x[1].parse()?)
    }

    /// `full_key`'s format : /[biz type]/[biz key]
    pub fn from_full_key(full_key: &str, version: i32) -> Result<Meta> {
        let err_msg = "illegal format for `full_key` : ".to_string() + full_key.clone();
        if full_key == "/N" {
            return Meta::new_with_type(full_key, MetaType::Null);
        }
        if full_key.len() < 3 {
            return Err(NatureError::VerifyError(err_msg));
        }
        if &full_key[2..3] != "/" {
            return Err(NatureError::VerifyError(err_msg));
        }
        let meta_type = MetaType::from_prefix(&full_key[0..2])?;
        Meta::new_with_version_and_type(&full_key[3..], version, meta_type)
    }

    pub fn get<T, W>(&mut self, meta_cache_getter: fn(&mut Meta, fn(&Meta) -> Result<T>) -> Result<W>, meta_getter: fn(&Meta) -> Result<T>) -> Result<()> {
        meta_cache_getter(self, meta_getter)?;
        Ok(())
    }

    pub fn has_state(&self, state: &State) -> bool {
        match &self.state {
            None => false,
            Some(x) => x.iter().find(|one| { one.include(state) }).is_some()
        }
    }

    pub fn has_state_name(&self, name: &str) -> bool {
        match &self.state {
            None => false,
            Some(x) => x.iter().find(|one| { one.has_name(name) }).is_some()
        }
    }

    pub fn meta_string(&self) -> String {
        format!("{}:{}", self.full_key, self.version)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn key_can_not_be_null() {
        let key = String::new();
        let rtn = Meta::new(&key);
        if let Err(NatureError::VerifyError(x)) = rtn {
            assert_eq!(x, "key length can't be zero");
        } else {
            panic!("should get error")
        }

        let key = "/".to_string();
        let rtn = Meta::new(&key);
        if let Err(NatureError::VerifyError(x)) = rtn {
            assert_eq!(x, "key length can't be zero");
        } else {
            panic!("should get error")
        }
    }

    #[test]
    fn key_can_be_empty_except_for_null_meta_type() {
        // key is empty
        let meta = Meta::new_with_type("", MetaType::Null).unwrap();
        assert_eq!(MetaType::Null, meta.get_meta_type());
        assert_eq!("/N", meta.get_full_key());

        // key is not empty
        let meta = Meta::new_with_type("not empty", MetaType::Null).unwrap();
        assert_eq!(MetaType::Null, meta.get_meta_type());
        assert_eq!("/N", meta.get_full_key());

        // call `new_null` directly
        let meta = Meta::new_null();
        assert_eq!(MetaType::Null, meta.get_meta_type());
        assert_eq!("/N", meta.get_full_key());
    }

    /// also test for removing last separator and Business prefix
    #[test]
    fn standardize_no_separator_at_beginning() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Meta::new(&key);
        assert_eq!("/a/b/c", rtn.unwrap().key);
        let rtn = Meta::new(&key);
        assert_eq!("/B/a/b/c", rtn.unwrap().get_full_key());
    }

    #[test]
    fn get_full_key() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Meta::new_with_type(&key.clone(), MetaType::System);
        assert_eq!(rtn.unwrap().get_full_key(), "/S/a/b/c");
        let rtn = Meta::new_with_type(&key, MetaType::Dynamic);
        assert_eq!(rtn.unwrap().get_full_key(), "/D/a/b/c");
        let rtn = Meta::new_with_type(&key, MetaType::Business);
        assert_eq!(rtn.unwrap().get_full_key(), "/B/a/b/c");
        let rtn = Meta::new_with_type(&key, MetaType::Null);
        assert_eq!(rtn.unwrap().get_full_key(), "/N");
    }

    #[test]
    fn from_full_key() {
        // error full_key
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : ".to_string())), Meta::from_full_key("", 1));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /s".to_string())), Meta::from_full_key("/s", 1));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /ss".to_string())), Meta::from_full_key("/ss", 1));
        assert_eq!(Err(NatureError::VerifyError("unknow prefix : [/s]".to_string())), Meta::from_full_key("/s/s", 1));
        assert_eq!(Meta::new_with_type("/N", MetaType::Null), Meta::from_full_key("/N", 1));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /Na".to_string())), Meta::from_full_key("/Na", 1));
        assert_eq!(Meta::new_with_type("/a", MetaType::Null), Meta::from_full_key("/N/a", 1));
        assert_eq!(Meta::new_with_type("/hello", MetaType::Dynamic), Meta::from_full_key("/D/hello", 1));
        assert_eq!(Meta::new_with_type("/world", MetaType::System), Meta::from_full_key("/S/world", 1));
        assert_eq!(Meta::new_with_type("/my", MetaType::Business), Meta::from_full_key("/B/my", 1));
    }

    #[test]
    fn has_state_test() {
        let mut m = Meta::new("hello").unwrap();
        assert_eq!(m.has_state(&State::Normal("a".to_string())), false);
        m.state = Some(vec![State::Normal("a".to_string())]);
        assert_eq!(m.has_state(&State::Normal("a".to_string())), true);
        assert_eq!(m.has_state(&State::Normal("b".to_string())), false);
    }

    #[test]
    fn has_state_name_test() {
        let mut m = Meta::new("hello").unwrap();
        assert_eq!(m.has_state_name("a"), false);
        m.state = Some(vec![State::Normal("a".to_string())]);
        assert_eq!(m.has_state_name("a"), true);
        assert_eq!(m.has_state_name("b"), false);
    }

    #[test]
    fn meta_string_test(){
        let m = Meta::new("hello").unwrap();
        assert_eq!(m.meta_string(), "/B/hello:1");
    }
}
