use super::NatureError;
use super::Result;
use super::MetaType;

/// Business Metadata
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Meta {
    /// # Identify a `Thing`.
    ///
    /// A `Thing` may have a lots of `Instance`s, so it's a **Class** for Instance`.
    /// Because there are huge quantity of `Thing`s , so we need a way to organize `Thing`s.
    /// A way is to set name with hierarchical structures,
    ///
    /// The `key` will include `meta_type` prefix.
    ///
    /// # Value Example
    ///
    /// /B/shop/order
    key: String,

    /// key with `ThingType` prefix
    full_key: String,

    /// A `Thing` can be changed in future, the `version` will support this without effect the old ones
    pub version: i32,

    /// A `Thing`'s type
    meta_type: MetaType,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            key: String::new(),
            full_key: MetaType::Business.get_prefix(),
            version: 1,
            meta_type: MetaType::Business,
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
                    meta_type: meta_type,
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
            meta_type: meta_type,
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

    pub fn from_full_key(fk: &str, version: i32) -> Result<Meta> {
        let err_msg = "illegal format for `full_key` : ".to_string() + fk.clone();
        if fk == "/N" {
            return Meta::new_with_type(fk, MetaType::Null);
        }
        if fk.len() < 3 {
            return Err(NatureError::VerifyError(err_msg));
        }
        if &fk[2..3] != "/" {
            return Err(NatureError::VerifyError(err_msg));
        }
        let meta_type = MetaType::from_prefix(&fk[0..2])?;
        Meta::new_with_version_and_type(&fk[3..], version, meta_type)
    }

    pub fn check<T, F>(&self, checker: F) -> Result<()> where F: Fn(&Meta) -> Result<T> {
        checker(&self)?;
        Ok(())
    }
}

/// separator for `Thing`'s key
static PATH_SEPARATOR: char = '/';


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
}
