use std::fmt::Write;
use std::str::FromStr;

use crate::NatureError;

pub type InstanceStates = Vec<InstanceState>;

#[derive(PartialEq, Debug)]
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
                let subs = &s[index + 1..s.len() - 1];
                let subs: Vec<&str> = subs.split(",").collect();
                let mut vec1: Vec<InstanceState> = Vec::new();
                let _ = subs.iter().try_for_each(|x| {
                    match x.parse::<InstanceState>() {
                        Ok(s) => {
                            vec1.push(s);
                            Ok(())
                        }
                        Err(e) => Err(e)
                    }
                })?;
                InstanceState {
                    name: s[..index].to_string(),
                    sub_state: vec1,
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
            let _ = write!(&mut rtn, ",{}", states[x].to_string());
        }
        rtn
    }
}

#[cfg(test)]
mod instance_state_to_string {
    use crate::InstanceState;

    #[test]
    fn to_string() {
        let string = InstanceState {
            name: "aaa".to_string(),
            sub_state: vec![],
        }.to_string();
        assert_eq!(string, "aaa");
        let string = InstanceState {
            name: "aaa".to_string(),
            sub_state: vec![
                InstanceState {
                    name: "a".to_string(),
                    sub_state: vec![],
                },
                InstanceState {
                    name: "b".to_string(),
                    sub_state: vec![
                        InstanceState {
                            name: "b-1".to_string(),
                            sub_state: vec![],
                        },
                        InstanceState {
                            name: "b-2".to_string(),
                            sub_state: vec![],
                        },
                        InstanceState {
                            name: "b-3".to_string(),
                            sub_state: vec![],
                        }
                    ],
                },
                InstanceState {
                    name: "c".to_string(),
                    sub_state: vec![],
                }
            ],
        }.to_string();
        assert_eq!(string, "aaa[a,b[b-1,b-2,b-3],c]");
    }
}

#[cfg(test)]
mod str_to_instance_state {
    use crate::InstanceState;

    #[test]
    fn simple_str() {
        let result = "aaa".parse::<InstanceState>().unwrap();
        assert_eq!(result.name, "aaa");
        assert_eq!(result.sub_state.len(), 0);
    }

    #[test]
    fn has_sub() {
        let result = "aaa[a,b,c]".parse::<InstanceState>().unwrap();
        assert_eq!(result.name, "aaa");
        let subs = result.sub_state;
        assert_eq!(subs.len(), 3);
        assert_eq!(subs[0], "a".parse::<InstanceState>().unwrap());
        assert_eq!(subs[1], "b".parse::<InstanceState>().unwrap());
        assert_eq!(subs[2], "c".parse::<InstanceState>().unwrap());
    }
}
