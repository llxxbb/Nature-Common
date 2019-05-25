use crate::{Thing, Instance};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskForSerial {
    pub thing: Thing,
    pub context_for_finish: String,
    pub instances: Vec<Instance>,
}
