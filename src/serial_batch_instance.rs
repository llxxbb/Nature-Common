use crate::{BizMeta, Instance};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskForSerial {
    pub thing: BizMeta,
    pub context_for_finish: String,
    pub instances: Vec<Instance>,
}
