use std::fmt::Write;
use std::str::FromStr;

use crate::NatureError;
use std::ops::Try;

pub type InstanceStates = Vec<InstanceState>;

pub struct InstanceState {
    pub name: String,
    pub sub_state: InstanceStates,
}

impl ToString for InstanceState {
    fn to_string(&self) -> String {
        let rtn = self.name.clone();
        if self.sub_state.is_empty() {
            return rtn;
        }
        rtn + "[" + Self::states_to_string(&self.sub_state).as_str() + "]"
    }
}

impl Try for InstanceState{
    type Ok = ();
    type Error = ();

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn from_error(v: Self::Error) -> Self {
        unimplemented!()
    }

    fn from_ok(v: Self::Ok) -> Self {
        unimplemented!()
    }
}

impl FromStr for InstanceState {
    type Err = NatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(NatureError::VerifyError("state string can't be empty".to_string()));
        }
        let rtn = match s.find("[") {
            None => InstanceState {
                name: s.to_string(),
                sub_state: vec![],
            },
            Some(index) => {
                let subs = &s[index + 1..];
                let subs: Vec<&str> = subs.split(",").collect();
                InstanceState {
                    name: s[..index - 1].to_string(),
                    sub_state: subs.iter().map(|x| x.parse::<InstanceState>()?).collect(),
                }
            }
        };
        Ok(rtn)
    }
}

impl InstanceState {
    pub fn states_to_string(states: &InstanceStates) -> String {
        if states.len() < 1 {
            return "".to_string();
        }
        let mut rtn = states.get(0).unwrap().name.clone();
        for x in 1..states.len() {
            let _ = write!(&mut rtn, ",{}", states.get(x).unwrap().name);
        }
        rtn
    }
}
