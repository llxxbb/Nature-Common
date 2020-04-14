use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use chrono::prelude::*;

use crate::{FromInstance, generate_id, is_default, MetaType, NatureError, ParaForQueryByID, PART_SEPARATOR, Result, TargetState};
use crate::converter::DynamicConverter;

use super::Meta;

// sys context define
pub static CONTEXT_TARGET_INSTANCE_ID: &str = "target.id";

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
                meta: format!("{}{}{}:1", MetaType::default().get_prefix(), PART_SEPARATOR, key),
                content: "".to_string(),
                context: HashMap::new(),
                sys_context: HashMap::new(),
                states: HashSet::new(),
                state_version: 0,
                from: None,
                para: String::new(),
            },
            execute_time: 0,
            create_time: 0,
        })
    }

    pub fn check_and_revise<T, W>(&mut self, meta_cache_getter: fn(&str, &fn(&str) -> Result<T>) -> Result<W>, meta_getter: &fn(&str) -> Result<T>) -> Result<&mut Self> {
        let _ = Meta::get(&self.meta, meta_cache_getter, meta_getter)?;
        self.revise()
    }

    pub fn revise(&mut self) -> Result<&mut Self> {
        let now = Local::now().timestamp_millis();
        if self.create_time == 0 {
            self.create_time = now;
        }
        if self.execute_time == 0 {
            self.execute_time = now;
        }
        if self.para.is_empty() && self.id == 0 {
            self.id = generate_id(&self.data)?;
        }
        Ok(self)
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

    pub fn get_last_taget<DAO>(&self, target_meta: &str, dao: DAO) -> Result<Option<Instance>>
        where DAO: Fn(&ParaForQueryByID) -> Result<Option<Instance>>
    {
        match self.sys_context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
            // context have target id
            Some(state_id) => {
                debug!("get last state for meta {}", target_meta);
                let state_id = u128::from_str(state_id)?;
                dao(&ParaForQueryByID::new(state_id, &target_meta, &String::default(), 0))
            }
            None => Ok(None),
        }
    }

    pub fn get_master<ID>(&self, self_meta: &Meta, dao: ID) -> Result<Option<Instance>>
        where ID: Fn(&ParaForQueryByID) -> Result<Option<Instance>>
    {
        match self_meta.get_setting() {
            None => Ok(None),
            Some(setting) => match setting.master {
                None => Ok(None),
                Some(master) => Ok(dao(&ParaForQueryByID::new(self.id, &master, &self.para, 0))?)
            },
        }
    }

    pub fn get_unique(&self) -> String {
        format!("{}{}{}", &self.meta, &self.id, &self.para)
    }

    pub fn get_key(&self) -> String {
        format!("{}|{}|{}|{}", self.meta, self.id, self.para, self.state_version)
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
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub content: String,
    /// Is a json for a `Map[key, value]` which maybe used for next `Relation`
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub context: HashMap<String, String>,
    /// like `context` but is specified by Nature
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub sys_context: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub states: HashSet<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub from: Option<FromInstance>,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub para: String,
}

impl BizObject {
    pub fn modify_state(&mut self, add_and_delete: &TargetState, meta: &Meta) {
        // delete first
        if let Some(x) = &add_and_delete.remove {
            x.iter().for_each(|one| { self.states.remove(one); });
        }
        let mut append: Vec<String> = self.states.clone().into_iter().collect();
        match &add_and_delete.add {
            Some(ss) => {
                append.append(&mut ss.clone());
                let (remained, _) = meta.check_state(&append).unwrap();
                self.states = remained.into_iter().collect();
            }
            None => ()
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SelfRouteInstance {
    pub instance: Instance,
    pub converter: Vec<DynamicConverter>,
}

impl SelfRouteInstance {
    pub fn verify(&self) -> Result<()> {
        if self.converter.is_empty() {
            return Err(NatureError::VerifyError("executor must not empty for dynamic convert!".to_string()));
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

    pub fn get_key(&self) -> String {
        self.instance.get_key()
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
        let m_g: fn(&str) -> Result<String> = meta_getter;
        let _ = instance.check_and_revise(meta_cache, &m_g);
        assert_eq!(instance.execute_time > 0, true);
        assert_eq!(instance.create_time > 0, true);
    }

    #[test]
    fn can_not_get_from_cache() {
        let mut instance = Instance::new("/err").unwrap();
        fn cache<T, W>(_: &str, _: &fn(&str) -> Result<T>) -> Result<W> {
            Err(NatureError::VerifyError("cache error".to_string()))
        }
        fn getter<T>(_: &str) -> Result<T> {
            Err(NatureError::VerifyError("getter error".to_string()))
        }
        let m_g: fn(&str) -> Result<String> = getter;
        let result = instance.check_and_revise::<String, String>(cache, &m_g);
        assert!(result.is_err());
    }

    #[test]
    fn can_get_from_cache() {
        let mut instance = Instance::new("/ok").unwrap();
        fn cache<T>(_: &str, _: &fn(&str) -> Result<T>) -> Result<String> {
            Ok("hello".to_string())
        }
        fn getter<T>(_: &str) -> Result<T> {
            Err(NatureError::VerifyError("getter error".to_string()))
        }
        let m_g: fn(&str) -> Result<String> = getter;

        let result = instance.check_and_revise::<String, String>(cache, &m_g);
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
        assert_eq!(ins.meta, "B:hello:1");
        let ins = Instance::new("/hello").unwrap();
        assert_eq!(ins.meta, "B:hello:1");
    }

    #[test]
    fn unique_id_test() {
        let ins = Instance::new("hello").unwrap();
        assert_eq!(ins.get_unique(), "B:hello:10");
    }

    fn meta_cache(m: &str, _: &fn(&str) -> Result<String>) -> Result<Meta> {
        Meta::from_string(m)
    }

    fn meta_getter(_: &str) -> Result<String> {
        Ok("".to_string())
    }
}



