use super::NatureError;
use super::Result;

/// `Thing`'s basic information
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Thing {
    /// # Identify a `Thing`.
    ///
    /// A `Thing` may have a lots of `Instance`s, so it's a **Class** for Instance`.
    /// Because there are huge quantity of `Thing`s , so we need a way to organize `Thing`s.
    /// A way is to set name with hierarchical structures,
    ///
    /// # Value Example
    ///
    /// /shop/order
    pub key: String,

    /// A `Thing` can be changed in future, the `version` will support this without effect the old ones
    pub version: i32,

    /// A `Thing`'s type
    pub thing_type: ThingType,
}

impl Default for Thing {
    fn default() -> Self {
        Thing {
            key: String::default(),
            version: 0,
            thing_type: ThingType::Business,
        }
    }
}

impl Thing {
    /// prefix "/B(usiness)" to the head of the string,.to avoid outer use"/S(ystem)" prefix.
    pub fn key_standardize(biz: &mut String) -> Result<()> {
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
}

/// separator for `Thing`'s key
pub static PATH_SEPARATOR: char = '/';

/// the root for system `Thing`
pub static SYS_ROOT: &'static str = "/S";

/// the root for business `Thing`
pub static BIZ_ROOT: &'static str = "/B";


/// Every `Thing` must have a type
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub enum ThingType {
    Business,
    System,
}

