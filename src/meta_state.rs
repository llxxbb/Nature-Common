use std::fmt::Write;
use std::str::FromStr;

use crate::NatureError;

pub type States = Vec<State>;

pub enum State {
    Mutex(Vec<State>),
    Normal(String),
    Parent(String, Vec<State>),
}

enum StateType {
    Mutex,
    Normal,
    Parent,
}


impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::Normal(s) => s.to_string(),
            State::Parent(name, list) =>
                name.to_owned() + "[" + Self::states_to_string(list, ",").as_str() + "]",
            State::Mutex(list) =>
                Self::states_to_string(list, "|"),
        }
    }
}

impl State {
    pub fn states_to_string(states: &States, separator: &str) -> String {
        if states.len() < 1 {
            return "".to_string();
        }
        let mut rtn = states[0].to_string();
        for x in 1..states.len() {
            let _ = write!(&mut rtn, "{}{}", separator, states[x].to_string());
        }
        rtn
    }

    pub fn string_to_states(states: &str, mut rtn: Vec<State>) -> Result<States, NatureError> {
        if states.len() < 1 {
            return Err(NatureError::VerifyError("states string should not be empty".to_string()));
        }
        let mut remain = states;
        let one_state = match states.find(",") {
            None => states.parse::<State>(),
            Some(comma_index) => {
                let one = Self::cut_string(&remain, comma_index, remain.find("["), remain.find("|"));
                if one.len() == remain.len() {
                    // the last one
                    rtn.push(one.parse::<State>()?);
                    return Ok(rtn);
                }
                remain = &remain[one.len()..];
                one.parse::<State>()
            }
        }?;
        rtn.push(one_state);
        Self::string_to_states(remain, rtn)
    }

    fn cut_string(input: &str, comma_index: usize, square_index: Option<usize>, vertical_index: Option<usize>) -> String {
        let s_t = match square_index {
            Some(s_i) => match vertical_index {
                Some(v_i) =>
                    if v_i < s_i {
                        StateType::Mutex
                    } else {
                        StateType::Parent
                    },
                None => StateType::Parent
            }
            None => match vertical_index {
                Some(_) => StateType::Mutex,
                None => StateType::Normal,
            }
        };
        match s_t {
            StateType::Normal => Self::cut_normal(input, comma_index),
            StateType::Mutex => Self::cut_mutex(input),
            StateType::Parent => Self::cut_parent(input),
        }
    }
    fn cut_normal(input: &str, comma_index: usize) -> String {
        input[..comma_index].to_string()
    }

    fn cut_mutex(input: &str) -> String {
        match input.find("[") {
            None => match input.find(",") {
                None => input.to_string(),
                Some(idx) => input[..idx].to_string(),
            },
            Some(idx) => {
                // skip square
            }
        }
        // TODO
        unimplemented!()
    }
    fn cut_parent(input: &str) -> String {
        // TODO
        unimplemented!()
    }

    fn find_square_end(input: &str, begin: usize) -> usize {}
}

impl FromStr for State {
    type Err = NatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(NatureError::VerifyError("state string can't be empty".to_string()));
        }
        let rtn = match s.find("[") {
            Some(index) => {
                let subs = &s[index + 1..s.len() - 1];
                State::Parent(s[..index].to_string(), Self::string_to_states(subs, vec![])?)
            }
            None => match s.find("|") {
                None => State::Normal(s.to_owned()),
                Some(_) => {
                    let x: Vec<&str> = s.split("|").collect();
                    let mut r_v: Vec<State> = vec![];
                    let vec_result: Result<(), NatureError> = x.iter().try_for_each(|one| {
                        let result = one.parse::<State>()?;
                        r_v.push(result);
                        Ok(())
                    });
                    let _ = vec_result?;
                    State::Mutex(r_v)
                }
            }
        };
        Ok(rtn)
    }
}


#[cfg(test)]
mod instance_state_to_string {
    use crate::meta_state::State;

    #[test]
    fn to_string() {
        let string = State::Parent("a".to_string(), vec![
            State::Parent("b".to_string(), vec![
                State::Normal("e".to_string()),
                State::Mutex(vec![
                    State::Normal("f".to_string()),
                    State::Normal("g".to_string()),
                ]),
                State::Normal("i".to_string()),
            ]),
            State::Normal("c".to_string()),
            State::Normal("d".to_string()),
        ]).to_string();
        assert_eq!(string, "a[b[e,f|g,i],c,d]");
    }
}

//
//#[cfg(test)]
//mod str_to_instance_state {
//    use crate::InstanceState;
//
//    #[test]
//    fn simple_str() {
//        let result = "aaa".parse::<InstanceState>().unwrap();
//        assert_eq!(result.name, "aaa");
//        assert_eq!(result.sub_state.len(), 0);
//    }
//
//    #[test]
//    fn has_sub() {
//        let result = "aaa[a,b,c]".parse::<InstanceState>().unwrap();
//        assert_eq!(result.name, "aaa");
//        let subs = result.sub_state;
//        assert_eq!(subs.len(), 3);
//        assert_eq!(subs[0], "a".parse::<InstanceState>().unwrap());
//        assert_eq!(subs[1], "b".parse::<InstanceState>().unwrap());
//        assert_eq!(subs[2], "c".parse::<InstanceState>().unwrap());
//    }
//}
