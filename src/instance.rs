use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};

use crate::{generate_id, NatureError, Result};
use crate::converter::DynamicConverter;
use crate::meta_type::MetaType;

use super::Meta;

/// A snapshot for a particular `Meta`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Instance {
    /// A unique value used to distinguish other instance
    pub id: u128,
    pub data: InstanceNoID,
}

impl Instance {
    pub fn new(key: &str) -> Result<Self> {
        Ok(Instance {
            id: 0,
            data: InstanceNoID {
                meta: Meta::new(key)?,
                event_time: 0,
                execute_time: 0,
                create_time: 0,
                content: "".to_string(),
                context: HashMap::new(),
                states: HashSet::new(),
                status_version: 0,
                from: None,
            },
        })
    }

    pub fn new_with_type(key: &str, meta: MetaType) -> Result<Self> {
        Ok(Instance {
            id: 0,
            data: InstanceNoID {
                meta: Meta::new_with_type(key, meta)?,
                event_time: 0,
                execute_time: 0,
                create_time: 0,
                content: "".to_string(),
                context: HashMap::new(),
                states: HashSet::new(),
                status_version: 0,
                from: None,
            },
        })
    }

    pub fn change_meta_type(&mut self, meta_type: MetaType) {
        self.data.meta.set_meta_type(meta_type);
    }

    pub fn fix_id(&mut self) -> Result<&mut Self> {
        if self.id == 0 {
            self.id = generate_id(&self.data)?;
        }
        Ok(self)
    }

    pub fn check_and_fix_id<T, F>(&mut self, meta_getter: F) -> Result<&mut Self>
        where F: Fn(&Meta) -> Result<T> {
        let _ = self.meta.get(meta_getter)?;
        self.fix_id()
    }
}

impl Deref for Instance {
    type Target = InstanceNoID;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.data
    }
}

impl DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Iterator for Instance {
    type Item = Instance;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.clone())
    }
}

/// A snapshot for a particular `Meta`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct InstanceNoID {
    /// This instance's Type
    pub meta: Meta,
    /// The time that this instance exists
    pub event_time: i64,
    /// The time which plan to flow for this instance
    pub execute_time: i64,
    /// When this instance created in db
    pub create_time: i64,
    /// What contend in this instance for the `Meta`
    pub content: String,
    /// Is a json for a `Map[key, value]` which contents other instance for other `Meta`'s.
    /// `Nature` can transform those to `Instance`'s by flowing.
    ///
    /// # Key
    ///
    /// context name
    ///
    /// # Value
    ///
    /// json data for a `Instance`.
    pub context: HashMap<String, String>,
    pub states: HashSet<String>,
    pub status_version: i32,
    pub from: Option<FromInstance>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct FromInstance {
    pub meta: Meta,
    pub status_version: i32,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SelfRouteInstance {
    pub instance: Instance,
    pub converter: Vec<DynamicConverter>,
}

impl SelfRouteInstance {
    pub fn verify(&self) -> Result<()> {
        if self.converter.is_empty() {
            return Err(NatureError::VerifyError("converter must not empty for dynamic convert!".to_string()));
        }
        Ok(())
    }
    pub fn to_instance(&self) -> Instance {
        Instance {
            id: 0,
            data: self.instance.data.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn id_generate() {
        // TODO
//        let mocks = MyMocks::new();
//        let service = InstanceServiceImpl {
//            define_cache: mocks.c_meta.clone()
//        };
//        let mut instance = Instance::new("hello").unwrap();
//        service.id_generate_if_not_set(&mut instance).unwrap();
//        println!("{:?}", instance.id);
//        assert_eq!(instance.id, 336556392135652841283170827290494770821);
    }
}

