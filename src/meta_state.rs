use std::fmt::Write;
use std::str::FromStr;

use crate::NatureError;

pub type States = Vec<State>;

#[derive(Debug, PartialEq)]
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

    pub fn string_to_states_v2(states: &str) -> Result<(States, usize), NatureError> {
        // check length
        if states.len() < 1 {
            return Err(NatureError::VerifyError("states string should not be empty".to_string()));
        }
        // store temp result
        let mut rtn: States = vec![];
        let mut normal = String::new();
        let mut mutex: States = vec![];
        let mut is_mutex = false;
        let mut parent: Option<State> = None;
        let mut x = 0;
        // main progress
        while x < states.len() {
            let c = &states[x..x + 1];
            dbg!(x, c);
            match c {
                "," => {    // separator
                    if is_mutex {
                        if normal.len() > 0 {
                            mutex.push(State::Normal(normal));
                            normal = String::new();
                        } else if parent.is_some() {
                            mutex.push(parent.unwrap());
                            parent = None;
                        }
                        let mut nm: States = vec![];
                        nm.append(&mut mutex);
                        rtn.push(State::Mutex(nm));
                        is_mutex = false;
                    } else {
                        if normal.len() > 0 {  // the ']' logic will make `normal` be empty.
                            rtn.push(State::Normal(normal));
                            normal = String::new();
                        } else if parent.is_some() {
                            rtn.push(parent.unwrap());
                            parent = None;
                        }
                    }
                }
                "|" => {    // mutex
                    is_mutex = true;
                    if normal.len() > 0 {  // the ']' logic will make `normal` be empty.
                        mutex.push(State::Normal(normal));
                        normal = String::new();
                    } else if parent.is_some() {
                        mutex.push(parent.unwrap());
                        parent = None;
                    }
                }
                "[" => {    // parent begin
                    let r = Self::string_to_states_v2(&states[x + 1..])?;
                    x = x + r.1;
                    parent = Some(State::Parent(normal, r.0));
                    normal = String::new();
                }
                "]" => {    // parent end
                    match is_mutex {
                        false => {
                            if normal.len() > 0 {
                                rtn.push(State::Normal(normal));
                            } else if parent.is_some() {
                                rtn.push(parent.unwrap());
                            }
                        }
                        true => {
                            if normal.len() > 0 {
                                mutex.push(State::Normal(normal))
                            } else if parent.is_some() {
                                mutex.push(parent.unwrap());
                            }
                            rtn.push(State::Mutex(mutex));
                        }
                    }
                    return Ok((rtn, x + 1));
                }
                _ => {      // litera
                    let w = write!(&mut normal, "{}", c);
                    if w.is_err() {
                        return Err(NatureError::SystemError(w.err().unwrap().to_string()));
                    }
                }
            }
            x = x + 1;
        }
        // the last normal unhandled by loop
        if is_mutex {
            if normal.len() > 0 {
                mutex.push(State::Normal(normal));
            } else if parent.is_some() {
                mutex.push(parent.unwrap());
            }
            rtn.push(State::Mutex(mutex));
        } else {
            if normal.len() > 0 {
                rtn.push(State::Normal(normal));
            } else if parent.is_some() {
                rtn.push(parent.unwrap());
            }
        }
        Ok((rtn, states.len()))
    }

    pub fn string_to_states_v1(_states: &str, mut _rtn: Vec<State>) -> Result<States, NatureError> {
        unimplemented!()
    }
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
                State::Parent(s[..index].to_string(), Self::string_to_states_v1(subs, vec![])?)
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
mod string_to_states_virtual_end {
    use super::*;

    // normal, no parent, no mutex : already test in other place
    // normal, no parent, mutex : already test in other place
    // normal, parent, no mutex : does not exist this case
    // normal, parent, mutex : does not exist this case
    // no normal, parent, not mutex : test under here
    #[test]
    fn for_parent_and_no_mutex() {
        let rtn = State::string_to_states_v2("p[b]|a").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
            State::Normal("a".to_string()),
        ]));
    }

    // no normal, parent, mutex : test under here
    #[test]
    fn for_parent_and_mutex() {
        let rtn = State::string_to_states_v2("a|p[b]|a").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
            State::Normal("a".to_string()),
        ]));
    }
}

#[cfg(test)]
mod string_to_states_square_end {
    use super::*;

    // normal, no parent, no mutex : already test in other place
    // normal, no parent, mutex : already test in other place
    #[test]
    fn for_normal_and_mutex() {
        let rtn = State::string_to_states_v2("p[b|a]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Mutex(vec![
                State::Normal("b".to_string()),
                State::Normal("a".to_string()),
            ]),
        ]));
    }

    // normal, parent, no mutex : does not exist this case
    // normal, parent, mutex : does not exist this case
    // no normal, parent, not mutex : test under here
    #[test]
    fn for_parent_and_no_mutex() {
        let rtn = State::string_to_states_v2("p[p[b]]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
        ]));
    }

    // no normal, parent, mutex : test under here
    #[test]
    fn for_parent_and_mutex() {
        let rtn = State::string_to_states_v2("p[a|p[b]]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Mutex(vec![
                State::Normal("a".to_string()),
                State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
            ]),
        ]));
    }
}

#[cfg(test)]
mod string_to_states_comma_end {
    use super::*;

    // normal, no parent, no mutex : already test in other place
    // normal, no parent, mutex : already test in other place
    // normal, parent, no mutex : does not exist this case
    // normal, parent, mutex : does not exist this case
    // no normal, parent, not mutex : already test in other place
    // no normal, parent, mutex : test under here
    #[test]
    fn for_parent_and_mutex() {
        let rtn = State::string_to_states_v2("a|p[b],a").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
        ]));
        assert_eq!(rtn.0[1], State::Normal("a".to_string()));
    }
}

#[cfg(test)]
mod string_to_states_for_mixed {
    use super::*;

    #[test]
    fn normal_parent() {
        let rtn = State::string_to_states_v2("a,p[c]").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Parent("p".to_string(), vec![State::Normal("c".to_string())]));
    }

    #[test]
    fn normal_mutex() {
        let rtn = State::string_to_states_v2("a,c|d").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Mutex(vec![
            State::Normal("c".to_string()),
            State::Normal("d".to_string()),
        ]));
    }

    #[test]
    fn parent_mutex() {
        let rtn = State::string_to_states_v2("p[a],c|d").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));
        assert_eq!(rtn.0[1], State::Mutex(vec![
            State::Normal("c".to_string()),
            State::Normal("d".to_string()),
        ]));
    }


    #[test]
    fn complex() {
        let rtn = State::string_to_states_v2("a,p[a],m|n,p[m|m,p[a,b]|p[c,c]]").unwrap();
        assert_eq!(rtn.0.len(), 4);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));
        assert_eq!(rtn.0[2], State::Mutex(vec![
            State::Normal("m".to_string()),
            State::Normal("n".to_string()),
        ]));
//        发现的问题： [ 逻辑不能立即放到 range 中去， 要看后面是，才可以，如果是 | 则需要放到 mutex 中去。
        assert_eq!(rtn.0[3], State::Parent("p".to_string(), vec![
            State::Mutex(vec![
                State::Normal("m".to_string()),
                State::Normal("m".to_string()),
            ]),
            State::Mutex(vec![
                State::Parent("p".to_string(), vec![
                    State::Normal("a".to_string()),
                    State::Normal("b".to_string()),
                ]),
                State::Parent("p".to_string(), vec![
                    State::Normal("c".to_string()),
                    State::Normal("c".to_string()),
                ]),
            ])]));
    }
}

#[cfg(test)]
mod string_to_states_for_mutex {
    use super::*;

    #[test]
    fn single() {
        let rtn = State::string_to_states_v2("a|b").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string()),
        ]));

        let rtn = State::string_to_states_v2("a|b|c").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string()),
            State::Normal("c".to_string()),
        ]));
    }

    #[test]
    fn multi() {
        let rtn = State::string_to_states_v2("a|b,c|d").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string()),
        ]));
        assert_eq!(rtn.0[1], State::Mutex(vec![
            State::Normal("c".to_string()),
            State::Normal("d".to_string()),
        ]));
    }
}

#[cfg(test)]
mod string_to_states_for_parent {
    use super::*;

    #[test]
    fn one_child() {
        let rtn = State::string_to_states_v2("p[a]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));
    }

    #[test]
    fn three_children() {
        let rtn = State::string_to_states_v2("p[a,b,c]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string()),
            State::Normal("c".to_string())]));
    }

    #[test]
    fn three_parent() {
        let rtn = State::string_to_states_v2("p1[a],p2[b],p3[c]").unwrap();
        assert_eq!(rtn.0.len(), 3);
        assert_eq!(rtn.0[0], State::Parent("p1".to_string(), vec![State::Normal("a".to_string())]));
        assert_eq!(rtn.0[1], State::Parent("p2".to_string(), vec![State::Normal("b".to_string())]));
        assert_eq!(rtn.0[2], State::Parent("p3".to_string(), vec![State::Normal("c".to_string())]));
    }

    #[test]
    fn comma_end() {
        let rtn = State::string_to_states_v2("p[a,").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));

        let rtn = State::string_to_states_v2("p[a,b],").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())]));
    }

    #[test]
    fn right_square_missed() {
        let rtn = State::string_to_states_v2("p[a").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));

        let rtn = State::string_to_states_v2("p[a,b").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())]));
    }
}

#[cfg(test)]
mod string_to_states_for_normal {
    use super::*;

    #[test]
    fn only_one() {
        let rtn = State::string_to_states_v2("test").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Normal("test".to_string()));
    }

    #[test]
    fn three() {
        let rtn = State::string_to_states_v2("a,b,c").unwrap();
        assert_eq!(rtn.0.len(), 3);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Normal("b".to_string()));
        assert_eq!(rtn.0[2], State::Normal("c".to_string()));
    }

    #[test]
    fn comma_end() {
        let rtn = State::string_to_states_v2("a,b,").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Normal("b".to_string()));
    }
}

#[cfg(test)]
mod states_to_string {
    use super::*;

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
