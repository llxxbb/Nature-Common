use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};

use chrono::prelude::*;

use crate::{generate_id, NatureError, Result, TargetState};
use crate::converter::DynamicConverter;
use crate::meta_type::MetaType;

use super::Meta;

/// A snapshot for a particular `Meta`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Instance {
    /// A unique value used to distinguish other instance
    pub id: u128,
    pub data: BizObject,
    /// The time which plan to flow for this instance
    pub execute_time: i64,
    /// When this instance created in db
    pub create_time: i64,
}

impl Instance {
    pub fn new(key: &str) -> Result<Self> {
        if key.is_empty() {
            return Err(NatureError::VerifyError("key can not be empty".to_string()));
        }
        let key = Meta::key_standardize(key)?;
        Ok(Instance {
            id: 0,
            data: BizObject {
                meta: format!("/B{}:1", key),
                content: "".to_string(),
                context: HashMap::new(),
                states: HashSet::new(),
                state_version: 0,
                from: None,
            },
            execute_time: 0,
            create_time: 0,
        })
    }

    pub fn new_with_type(key: &str, meta: MetaType) -> Result<Self> {
        if key.is_empty() {
            return Err(NatureError::VerifyError("key can not be empty".to_string()));
        }
        Ok(Instance {
            id: 0,
            data: BizObject {
                meta: format!("{}/{}:1", meta.get_prefix(), key),
                content: "".to_string(),
                context: HashMap::new(),
                states: HashSet::new(),
                state_version: 0,
                from: None,
            },
            execute_time: 0,
            create_time: 0,
        })
    }

    pub fn fix_id(&mut self) -> Result<&mut Self> {
        if self.id == 0 {
            self.id = generate_id(&self.data)?;
        }
        Ok(self)
    }

    pub fn revise<T, W>(&mut self, meta_cache_getter: fn(&str, fn(&str) -> Result<T>) -> Result<W>, meta_getter: fn(&str) -> Result<T>) -> Result<&mut Self> {
        let _ = Meta::get(&self.meta, meta_cache_getter, meta_getter)?;
        let now = Local::now().timestamp_millis();
        if self.create_time == 0 {
            self.create_time = now;
        }
        if self.execute_time == 0 {
            self.execute_time = now;
        }
        self.fix_id()
    }

    pub fn meta_must_same(is: &Vec<Self>) -> Result<()> {
        if is.len() < 2 {
            return Ok(());
        }
        let option = is[1..].iter().find(|x| { !x.meta.eq(&is[0].meta) });
        match option {
            Some(_) => Err(NatureError::VerifyError("instances's meta must be same!".to_string())),
            None => Ok(())
        }
    }
}


impl Deref for Instance {
    type Target = BizObject;

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
pub struct BizObject {
    /// This instance's Type
    pub meta: String,
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

impl BizObject {
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
            execute_time: 0,
            create_time: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn revise_test() {
        let mut instance = Instance::new("hello").unwrap();
        assert_eq!(instance.id, 0);
        assert_eq!(instance.execute_time, 0);
        assert_eq!(instance.create_time, 0);
        let _ = instance.revise(meta_cache, meta_getter);
        assert_eq!(instance.id, 110743375152055492399589371372438031015);
        assert_eq!(instance.execute_time > 0, true);
        assert_eq!(instance.create_time > 0, true);
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
        let result = instance.revise::<String, String>(cache, getter);
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
        let result = instance.revise::<String, String>(cache, getter);
        assert!(result.is_ok());
    }

    #[test]
    fn same_meta_test() {
        let vec1 = vec![Instance::new("hello").unwrap(), Instance::new("world").unwrap()];
        assert_eq!(Instance::meta_must_same(&vec1).is_err(), true);
        let vec1 = vec![Instance::new("hello").unwrap(), Instance::new("hello").unwrap()];
        assert_eq!(Instance::meta_must_same(&vec1).is_err(), false);
    }

    #[test]
    fn instance_new_test() {
        let ins = Instance::new("hello").unwrap();
        assert_eq!(ins.meta, "/B/hello:1");
        let ins = Instance::new("/hello").unwrap();
        assert_eq!(ins.meta, "/B/hello:1");
    }

    fn meta_cache(m: &str, _: fn(&str) -> Result<String>) -> Result<Meta> {
        Meta::from_string(m)
    }

    fn meta_getter(_: &str) -> Result<String> {
        Ok("".to_string())
    }
}



