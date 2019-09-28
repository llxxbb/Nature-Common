use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};

use crate::{generate_id, NatureError, Result, TargetState};
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
        if key.is_empty() {
            return Err(NatureError::VerifyError("key can not be empty".to_string()));
        }
        Ok(Instance {
            id: 0,
            data: InstanceNoID {
                meta: format!("/B/{}:1", key),
                event_time: 0,
                execute_time: 0,
                create_time: 0,
                content: "".to_string(),
                context: HashMap::new(),
                states: HashSet::new(),
                state_version: 0,
                from: None,
            },
        })
    }

    pub fn new_with_type(key: &str, meta: MetaType) -> Result<Self> {
        if key.is_empty() {
            return Err(NatureError::VerifyError("key can not be empty".to_string()));
        }
        Ok(Instance {
            id: 0,
            data: InstanceNoID {
                meta: format!("{}/{}:1", meta.get_prefix(), key),
                event_time: 0,
                execute_time: 0,
                create_time: 0,
                content: "".to_string(),
                context: HashMap::new(),
                states: HashSet::new(),
                state_version: 0,
                from: None,
            },
        })
    }

    pub fn fix_id(&mut self) -> Result<&mut Self> {
        if self.id == 0 {
            self.id = generate_id(&self.data)?;
        }
        Ok(self)
    }

    pub fn check_and_fix_id<T, W>(&mut self, meta_cache_getter: fn(&str, fn(&str) -> Result<T>) -> Result<W>, meta_getter: fn(&str) -> Result<T>) -> Result<&mut Self> {
        let _ = Meta::get(&self.meta, meta_cache_getter, meta_getter)?;
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
    pub meta: String,
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
    pub state_version: i32,
    pub from: Option<FromInstance>,
}

impl InstanceNoID {
    pub fn modify_state(&mut self, add_and_delete: TargetState) {
        // delete first
        if let Some(x) = add_and_delete.remove {
            x.iter().for_each(|one| { self.states.remove(one); });
        }
        // add then
        if let Some(x) = add_and_delete.add {
            x.iter().for_each(|one| { self.states.insert(one.clone()); });
        }
    }
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
    use super::*;

    #[test]
    fn automatic_generate_id() {
        let mut instance = Instance::new("hello").unwrap();
        assert_eq!(instance.id, 0);
        let _ = instance.fix_id();
        assert_eq!(instance.id, 17399718179305179577446015748023824286);
    }

    #[test]
    fn modify_state() {
        let mut ins = Instance::new("hello").unwrap();
        assert_eq!(ins.states.len(), 0);
        ins.modify_state(TargetState {
            add: None,
            remove: None,
        });
        assert_eq!(ins.states.len(), 0);
        ins.modify_state(TargetState {
            add: Some(vec!["a".to_string(), "b".to_string()]),
            remove: None,
        });
        assert_eq!(ins.states.len(), 2);
        assert_eq!(ins.states.contains("a"), true);
        assert_eq!(ins.states.contains("b"), true);
        ins.modify_state(TargetState {
            add: Some(vec!["c".to_string(), "d".to_string()]),
            remove: Some(vec!["a".to_string()]),
        });
        assert_eq!(ins.states.len(), 3);
        assert_eq!(ins.states.contains("b"), true);
        assert_eq!(ins.states.contains("c"), true);
        assert_eq!(ins.states.contains("d"), true);
        ins.modify_state(TargetState {
            add: None,
            remove: Some(vec!["b".to_string(), "c".to_string()]),
        });
        assert_eq!(ins.states.len(), 1);
        assert_eq!(ins.states.contains("d"), true);
        // add same
        ins.modify_state(TargetState {
            add: Some(vec!["d".to_string()]),
            remove: None,
        });
        assert_eq!(ins.states.len(), 1);
        assert_eq!(ins.states.contains("d"), true);
        // remove not exists
        ins.modify_state(TargetState {
            add: None,
            remove: Some(vec!["b".to_string(), "c".to_string()]),
        });
        assert_eq!(ins.states.len(), 1);
        assert_eq!(ins.states.contains("d"), true);
    }


    #[test]
    fn can_not_get_from_cache() {
        let mut instance = Instance::new("/err").unwrap();
        fn cache<T, W>(_: &str, _: fn(&str) -> Result<T>) -> Result<W> {
            Err(NatureError::VerifyError("cache error".to_string()))
        }
        fn getter<T>(_: &str) -> Result<T> {
            Err(NatureError::VerifyError("getter error".to_string()))
        }
        let result = instance.check_and_fix_id::<String, String>(cache, getter);
        assert!(result.is_err());
    }

    #[test]
    fn can_get_from_cache() {
        let mut instance = Instance::new("/ok").unwrap();
        fn cache<T>(_: &str, _: fn(&str) -> Result<T>) -> Result<String> {
            Ok("hello".to_string())
        }
        fn getter<T>(_: &str) -> Result<T> {
            Err(NatureError::VerifyError("getter error".to_string()))
        }
        let result = instance.check_and_fix_id::<String, String>(cache, getter);
        assert!(result.is_ok());
    }
}



