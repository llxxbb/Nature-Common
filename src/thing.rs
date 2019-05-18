use super::NatureError;
use super::Result;
use super::ThingType;

/// `Thing`'s basic information
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Thing {
    /// # Identify a `Thing`.
    ///
    /// A `Thing` may have a lots of `Instance`s, so it's a **Class** for Instance`.
    /// Because there are huge quantity of `Thing`s , so we need a way to organize `Thing`s.
    /// A way is to set name with hierarchical structures,
    ///
    /// The `key` will include `thing_type` prefix.
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
    thing_type: ThingType,
}

impl Default for Thing {
    fn default() -> Self {
        Thing {
            key: String::new(),
            full_key: ThingType::Business.get_prefix(),
            version: 1,
            thing_type: ThingType::Business,
        }
    }
}

impl Thing {
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

    /// version default to 1 and thing_type default to `Business`
    pub fn new(key: &str) -> Result<Self> {
        Self::new_with_version_and_type(key, 1, ThingType::Business)
    }

    /// version default to 1
    pub fn new_with_type(key: &str, thing_type: ThingType) -> Result<Self> {
        Self::new_with_version_and_type(key, 1, thing_type)
    }

    pub fn new_with_version_and_type(key: &str, version: i32, thing_type: ThingType) -> Result<Self> {
        if thing_type == ThingType::Null {
            return Ok(Self::new_null());
        }
        let mut key = key.to_string();
        match Self::key_standardize(&mut key) {
            Err(e) => Err(e),
            Ok(_) => Ok({
                Thing {
                    key: key.clone(),
                    full_key: thing_type.get_prefix() + &key,
                    version,
                    thing_type,
                }
            })
        }
    }

    pub fn new_null() -> Thing {
        let thing_type = ThingType::Null;
        Thing {
            key: String::new(),
            full_key: thing_type.get_prefix(),
            version: 1,
            thing_type,
        }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_full_key(&self) -> String {
        self.full_key.clone()
    }

    pub fn get_thing_type(&self) -> ThingType {
        self.thing_type.clone()
    }
    pub fn set_thing_type(&mut self, thing_type: ThingType) {
        self.thing_type = thing_type.clone();
        self.full_key = thing_type.get_prefix() + &self.key.clone();
    }

    pub fn from_full_key(fk: &str, version: i32) -> Result<Thing> {
        let err_msg = "illegal format for `full_key` : ".to_string() + fk.clone();
        if fk == "/N" {
            return Thing::new_with_type(fk, ThingType::Null);
        }
        if fk.len() < 3 {
            return Err(NatureError::VerifyError(err_msg));
        }
        if &fk[2..3] != "/" {
            return Err(NatureError::VerifyError(err_msg));
        }
        let thing_type = ThingType::from_prefix(&fk[0..2])?;
        Thing::new_with_version_and_type(&fk[3..], version, thing_type)
    }

    pub fn check<T, F>(&self, checker: F) -> Result<()> where F: Fn(&Thing) -> Result<T> {
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
        let rtn = Thing::new(&key);
        if let Err(NatureError::VerifyError(x)) = rtn {
            assert_eq!(x, "key length can't be zero");
        } else {
            panic!("should get error")
        }

        let key = "/".to_string();
        let rtn = Thing::new(&key);
        if let Err(NatureError::VerifyError(x)) = rtn {
            assert_eq!(x, "key length can't be zero");
        } else {
            panic!("should get error")
        }
    }

    #[test]
    fn key_can_be_empty_except_for_null_thing_type() {
        // key is empty
        let thing = Thing::new_with_type("", ThingType::Null).unwrap();
        assert_eq!(ThingType::Null, thing.get_thing_type());
        assert_eq!("/N", thing.get_full_key());

        // key is not empty
        let thing = Thing::new_with_type("not empty", ThingType::Null).unwrap();
        assert_eq!(ThingType::Null, thing.get_thing_type());
        assert_eq!("/N", thing.get_full_key());

        // call `new_null` directly
        let thing = Thing::new_null();
        assert_eq!(ThingType::Null, thing.get_thing_type());
        assert_eq!("/N", thing.get_full_key());
    }

    /// also test for removing last separator and Business prefix
    #[test]
    fn standardize_no_separator_at_beginning() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Thing::new(&key);
        assert_eq!("/a/b/c", rtn.unwrap().key);
        let rtn = Thing::new(&key);
        assert_eq!("/B/a/b/c", rtn.unwrap().get_full_key());
    }

    #[test]
    fn get_full_key() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Thing::new_with_type(&key.clone(), ThingType::System);
        assert_eq!(rtn.unwrap().get_full_key(), "/S/a/b/c");
        let rtn = Thing::new_with_type(&key, ThingType::Dynamic);
        assert_eq!(rtn.unwrap().get_full_key(), "/D/a/b/c");
        let rtn = Thing::new_with_type(&key, ThingType::Business);
        assert_eq!(rtn.unwrap().get_full_key(), "/B/a/b/c");
        let rtn = Thing::new_with_type(&key, ThingType::Null);
        assert_eq!(rtn.unwrap().get_full_key(), "/N");
    }

    #[test]
    fn from_full_key() {
        // error full_key
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : ".to_string())), Thing::from_full_key("", 1));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /s".to_string())), Thing::from_full_key("/s", 1));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /ss".to_string())), Thing::from_full_key("/ss", 1));
        assert_eq!(Err(NatureError::VerifyError("unknow prefix : [/s]".to_string())), Thing::from_full_key("/s/s", 1));
        assert_eq!(Thing::new_with_type("/N", ThingType::Null), Thing::from_full_key("/N", 1));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /Na".to_string())), Thing::from_full_key("/Na", 1));
        assert_eq!(Thing::new_with_type("/a", ThingType::Null), Thing::from_full_key("/N/a", 1));
        assert_eq!(Thing::new_with_type("/hello", ThingType::Dynamic), Thing::from_full_key("/D/hello", 1));
        assert_eq!(Thing::new_with_type("/world", ThingType::System), Thing::from_full_key("/S/world", 1));
        assert_eq!(Thing::new_with_type("/my", ThingType::Business), Thing::from_full_key("/B/my", 1));
    }
}
