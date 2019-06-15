use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::Deref;

use crate::{generate_id, NatureError, Result};
use crate::convertor::DynamicConverter;
use crate::thing_type::ThingType;

use super::BizMeta;

/// A snapshot for a particular `Thing`
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
                thing: BizMeta::new(key)?,
                event_time: 0,
                execute_time: 0,
                create_time: 0,
                content: "".to_string(),
                context: HashMap::new(),
                status: HashSet::new(),
                status_version: 0,
                from: None,
            },
        })
    }

    pub fn new_with_type(key: &str, thing_type: ThingType) -> Result<Self> {
        Ok(Instance {
            id: 0,
            data: InstanceNoID {
                thing: BizMeta::new_with_type(key, thing_type)?,
                event_time: 0,
                execute_time: 0,
                create_time: 0,
                content: "".to_string(),
                context: HashMap::new(),
                status: HashSet::new(),
                status_version: 0,
                from: None,
            },
        })
    }

    pub fn change_thing_type(&mut self, thing_type: ThingType) {
        self.data.thing.set_thing_type(thing_type);
    }

    pub fn fix_id(&mut self) -> Result<&mut Self> {
        if self.id == 0 {
            self.id = generate_id(&self.data)?;
        }
        Ok(self)
    }

    pub fn check_and_fix_id<T, F>(&mut self, checker: F) -> Result<&mut Self>
        where F: Fn(&BizMeta) -> Result<T> {
        let _ = self.thing.check(checker)?;
        self.fix_id()
    }

    pub fn save<F>(&self, dao: F) -> Result<usize>
        where F: Fn(&Instance) -> Result<usize>
    {
        match dao(self) {
            Ok(num) => Ok(num),
            Err(NatureError::DaoDuplicated(_)) => Ok(0),
            Err(e) => Err(e)
        }
    }
}

impl Deref for Instance {
    type Target = InstanceNoID;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.data
    }
}

impl Iterator for Instance {
    type Item = Instance;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.clone())
    }
}

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct InstanceNoID {
    /// This instance's Type
    pub thing: BizMeta,
    /// The time that this instance exists
    pub event_time: i64,
    /// The time which plan to flow for this instance
    pub execute_time: i64,
    /// When this instance created in db
    pub create_time: i64,
    /// What contend in this instance for the `Thing`
    pub content: String,
    /// Is a json for a `Map[key, value]` which contents other instance for other `Thing`'s.
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
    pub status: HashSet<String>,
    pub status_version: i32,
    pub from: Option<FromInstance>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct FromInstance {
    pub thing: BizMeta,
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
//            define_cache: mocks.c_thing_define.clone()
//        };
//        let mut instance = Instance::new("hello").unwrap();
//        service.id_generate_if_not_set(&mut instance).unwrap();
//        println!("{:?}", instance.id);
//        assert_eq!(instance.id, 336556392135652841283170827290494770821);
    }
}

