use crate::{Meta, Instance};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskForSerial {
    pub thing: Meta,
    pub context_for_finish: String,
    pub instances: Vec<Instance>,
}
