use ::Result;

use super::NatureError;

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
    pub key: String,

    /// A `Thing` can be changed in future, the `version` will support this without effect the old ones
    pub version: i32,

    /// A `Thing`'s type
    pub thing_type: ThingType,
}

impl Default for Thing {
    fn default() -> Self {
        Thing {
            key: ThingType::Business.get_prefix() + &String::default(),
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
        let mut key = key.to_string();
        match Self::key_standardize(&mut key) {
            Err(e) => Err(e),
            Ok(_) => Ok(Thing {
                key: thing_type.get_prefix() + &key,
                version,
                thing_type,
            })
        }
    }

    pub fn new_null() -> Thing {
        Thing {
            key: String::new(),
            version: 0,
            thing_type: ThingType::Business,
        }
    }
}

/// separator for `Thing`'s key
static PATH_SEPARATOR: char = '/';

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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn key_can_not_be_null() {
        println!("----------------- standardize_empty --------------------");
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

    /// also test for removing last separator and Business prefix
    #[test]
    fn standardize_no_separator_at_beginning() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Thing::new(&key);
        assert_eq!(rtn.unwrap().key, "/B/a/b/c");
    }

    #[test]
    fn thing_type_test() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Thing::new_with_type(&key.clone(), ThingType::System);
        assert_eq!(rtn.unwrap().key, "/S/a/b/c");
        let rtn = Thing::new_with_type(&key, ThingType::Dynamic);
        assert_eq!(rtn.unwrap().key, "/D/a/b/c");
        let rtn = Thing::new_with_type(&key, ThingType::Business);
        assert_eq!(rtn.unwrap().key, "/B/a/b/c");
        let rtn = Thing::new_with_type(&key, ThingType::Null);
        assert_eq!(rtn.unwrap().key, "/N/a/b/c");
    }

    #[test]
    fn key_cat_be_null() {
        let rtn = Thing::new(&String::new());
        match rtn.err().unwrap() {
            NatureError::VerifyError(ss) => assert_eq!(ss, "key length can\'t be zero"),
            err => {
                println!("{:?}", err);
                panic!("un match")
            }
        }
    }
}
