use crate::Instance;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskForSerial {
    pub context_for_finish: String,
    pub instances: Vec<Instance>,
}
