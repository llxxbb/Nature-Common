use crate::{Instance, Meta};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskForParallel {
    pub meta: Meta,
    pub instances: Vec<Instance>,
}
