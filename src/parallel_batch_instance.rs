use crate::{Instance, BizMeta};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskForParallel {
    pub thing: BizMeta,
    pub instances: Vec<Instance>,
}
