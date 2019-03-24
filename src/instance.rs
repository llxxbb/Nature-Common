use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::Deref;

use convertor::DynamicConverter;

use super::Thing;

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Instance {
    /// A unique value used to distinguish other instance
    pub id: u128,
    pub data: InstanceNoID,
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
    pub thing: Thing,
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
    pub thing: Thing,
    pub status_version: i32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ParallelBatchInstance {
    pub thing: Thing,
    pub instances: Vec<Instance>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SerialBatchInstance {
    pub thing: Thing,
    pub context_for_finish: String,
    pub instances: Vec<Instance>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SelfRouteInstance {
    pub instance: Instance,
    pub converter: Vec<DynamicConverter>,
}


