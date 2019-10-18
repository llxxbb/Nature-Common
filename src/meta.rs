use std::collections::{HashMap, HashSet};

use crate::{CheckType, MetaSetting, State, StatePath};
use crate::meta_string::MetaString;
use crate::NatureError::VerifyError;
use crate::state::States;

use super::MetaType;
use super::NatureError;
use super::Result;

/// separator for `Meta`'s key
static PATH_SEPARATOR: char = '/';
pub static META_AND_VERSION_SEPARATOR: &str = ":";

/// Business Metadata
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    /// # Identify a `Meta`.
    ///
    /// A `Meta` may have a lots of `Instance`s, so it's a **Class** for Instance`.
    /// Because there are huge quantity of `Meta`s , so we need a way to organize `Meta`s.
    /// A way is to set name with hierarchical structures,
    key: String,

    /// key with `MetaType` prefix
    /// # Value Example
    ///
    /// /B/shop/order
    full_key: String,

    /// A `Meta` can be changed in future, the `version` will support this without effect the old ones
    pub version: i32,

    /// A `Meta`'s type
    meta_type: MetaType,

    state: Option<States>,

    is_state: bool,

    setting: Option<MetaSetting>,

    check_list: HashMap<String, StatePath>,
}


impl Default for Meta {
    fn default() -> Self {
        Meta {
            key: String::new(),
            full_key: MetaType::Business.get_prefix(),
            version: 1,
            meta_type: MetaType::Business,
            state: None,
            is_state: false,
            setting: None,
            check_list: Default::default(),
        }
    }
}

impl Meta {
    /// make start with "/" and remove "/" at the end
    pub fn key_standardize(biz: &str) -> Result<String> {
        let mut biz = biz.to_string();
        if biz.ends_with(PATH_SEPARATOR) {
            let last = biz.len() - 1;
            biz.remove(last);
        }
        if biz.is_empty() {
            return Err(NatureError::VerifyError("key length can't be zero".to_string()));
        }
        if !biz.starts_with(PATH_SEPARATOR) {
            biz.insert(0, PATH_SEPARATOR);
        }
        Ok(biz)
    }

    pub fn new(key: &str, version: i32, meta_type: MetaType) -> Result<Self> {
        let key = match meta_type {
            MetaType::Null => "".to_string(),
            _ => Self::key_standardize(key)?
        };
        Ok(Meta {
            key: key.to_string(),
            full_key: meta_type.get_prefix() + &key,
            version,
            meta_type,
            state: None,
            is_state: false,
            setting: None,
            check_list: Default::default(),
        })
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_full_key(&self) -> String {
        self.full_key.clone()
    }

    pub fn get_meta_type(&self) -> MetaType {
        self.meta_type.clone()
    }
    pub fn set_meta_type(&mut self, meta_type: MetaType) {
        self.meta_type = meta_type.clone();
        self.full_key = meta_type.get_prefix() + &self.key.clone();
    }

    pub fn get_string(&self) -> String {
        self.full_key.clone() + META_AND_VERSION_SEPARATOR + &self.version.to_string()
    }

    /// `full_key`'s format : /[biz type]/[biz key]
    pub fn from_full_key(full_key: &str, version: i32) -> Result<Meta> {
        let err_msg = "illegal format for `full_key` : ".to_string() + full_key.clone();
        if full_key == "/N" {
            return Meta::new(full_key, 1, MetaType::Null);
        }
        if full_key.len() < 3 {
            return Err(NatureError::VerifyError(err_msg));
        }
        if &full_key[2..3] != "/" {
            return Err(NatureError::VerifyError(err_msg));
        }
        let meta_type = MetaType::from_prefix(&full_key[0..2])?;
        Meta::new(&full_key[3..], version, meta_type)
    }

    /// `meta_str`'s format : [full_key]:[version]
    pub fn from_string(meta_str: &str) -> Result<Meta> {
        let (full_key, version) = MetaString::make_tuple_from_str(meta_str)?;
        Self::from_full_key(&full_key, version)
    }

    pub fn get<T, W>(meta_str: &str, meta_cache_getter: fn(&str, fn(&str) -> Result<T>) -> Result<W>, meta_getter: fn(&str) -> Result<T>) -> Result<W> {
        let meta = meta_cache_getter(meta_str, meta_getter)?;
        Ok(meta)
    }

    pub fn has_state_name(&self, name: &str) -> bool {
        let option = self.check_list.get(name);
        option.is_some()
    }

    pub fn meta_string(&self) -> String {
        format!("{}:{}", self.full_key, self.version)
    }

    pub fn set_states(&mut self, states: Option<States>) -> Result<()> {
        match states {
            Some(ss) => {
                Self::avoid_same_name(&ss, &self.get_string())?;
                self.init_check_list(&ss, 0, &mut Default::default());
                self.state = Some(ss);
                self.is_state = true;
            }
            _ => self.state = None
        }
        Ok(())
    }

    fn init_check_list(&mut self, ss: &States, id: u16, path: &mut StatePath) {
        let mut id = id;
        ss.iter().for_each(|s| {
            id += 1;
            match s {
                State::Normal(name) => {
                    let mut new = path.clone();
                    new.desc_seq.insert(0, CheckType::Normal(id));
                    self.check_list.insert(name.to_string(), new);
                }
                State::Parent(_, nss) => {
                    let mut new = path.clone();
                    new.desc_seq.insert(0, CheckType::Parent(id));
                    self.init_check_list(nss, id, &mut new);
                }
                State::Mutex(nss) => {
                    let mut new = path.clone();
                    new.is_mutex = true;
                    new.desc_seq.insert(0, CheckType::Mutex(id));
                    self.init_check_list(nss, id, &mut new);
                }
            }
        })
    }

    pub fn verify_state(&self, input: &HashSet<String>) -> Result<()> {
        if !self.is_state {
            return Err(VerifyError(format!("[{}] is not a state meta", self.get_string())));
        }
        let mut map: HashMap<u16, u16> = HashMap::new();
        for one in input {
            let option = self.check_list.get(one);
            // undefined
            if option.is_none() {
                let msg = format!("[{}] does not defined in meta: {}", one, self.meta_string());
                warn!("{}", &msg);
                return Err(NatureError::VerifyError(msg));
            }
            // not mutex
            let path = option.unwrap();
            if !path.is_mutex {
                continue;
            }
            // mutex
            let mut last: u16 = 0;
            for op in &path.desc_seq {
                match op {
                    CheckType::Normal(id) => { last = *id; }
                    CheckType::Parent(id) => { last = *id; }
                    CheckType::Mutex(id) => {
                        let cached_p = map.get(id);
                        if let Some(e) = cached_p {
                            if *e != last {
                                let msg = format!("[{}] mutex conflict for meta: {}", one, self.meta_string());
                                warn!("{}", &msg);
                                return Err(NatureError::VerifyError(msg));
                            }
                        } else {
                            map.insert(*id, last);
                            last = *id;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_states(&self) -> Option<States> {
        self.state.clone()
    }
    pub fn is_state(&self) -> bool {
        self.is_state
    }

    fn avoid_same_name(s: &States, string_meta: &str) -> Result<()> {
        let mut set: HashSet<String> = HashSet::new();
        for one in s {
            if !set.insert(one.get_name()) {
                return Err(NatureError::VerifyError(format!("repeated state name: {:?}, for `Meta`: {:?}", one.get_name(), string_meta)));
            }
        }
        Ok(())
    }

    pub fn set_setting(&mut self, settings: &str) -> Result<()> {
        if !settings.is_empty() {
            let setting: MetaSetting = serde_json::from_str(settings)?;
            if setting.is_state {
                self.is_state = true;
            }
            self.setting = Some(setting);
        } else {
            self.setting = None;
        }
        Ok(())
    }

    pub fn get_setting(&self) -> Option<MetaSetting> {
        self.setting.clone()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn key_can_not_be_null() {
        let key = String::new();
        let rtn = Meta::new(&key, 1, MetaType::Business);
        if let Err(NatureError::VerifyError(x)) = rtn {
            assert_eq!(x, "key length can't be zero");
        } else {
            panic!("should get error")
        }

        let key = "/".to_string();
        let rtn = Meta::new(&key, 1, MetaType::Business);
        if let Err(NatureError::VerifyError(x)) = rtn {
            assert_eq!(x, "key length can't be zero");
        } else {
            panic!("should get error")
        }
    }

    #[test]
    fn key_can_be_empty_except_for_null_meta_type() {
        // key is empty
        let meta = Meta::new("", 1, MetaType::Null).unwrap();
        assert_eq!(MetaType::Null, meta.get_meta_type());
        assert_eq!("/N", meta.get_full_key());

        // key is not empty
        let meta = Meta::new("not empty", 1, MetaType::Null).unwrap();
        assert_eq!(MetaType::Null, meta.get_meta_type());
        assert_eq!("/N", meta.get_full_key());
    }

    /// also test for removing last separator and Business prefix
    #[test]
    fn standardize_no_separator_at_beginning() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Meta::new(&key, 1, MetaType::Business);
        assert_eq!("/a/b/c", rtn.unwrap().key);
        let rtn = Meta::new(&key, 1, MetaType::Business);
        assert_eq!("/B/a/b/c", rtn.unwrap().get_full_key());
    }

    #[test]
    fn get_full_key() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Meta::new(&key, 1, MetaType::System);
        assert_eq!(rtn.unwrap().get_full_key(), "/S/a/b/c");
        let rtn = Meta::new(&key, 1, MetaType::Dynamic);
        assert_eq!(rtn.unwrap().get_full_key(), "/D/a/b/c");
        let rtn = Meta::new(&key, 1, MetaType::Business);
        assert_eq!(rtn.unwrap().get_full_key(), "/B/a/b/c");
        let rtn = Meta::new(&key, 1, MetaType::Null);
        assert_eq!(rtn.unwrap().get_full_key(), "/N");
    }

    #[test]
    fn from_meta_str() {
        // error full_key
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : ".to_string())), Meta::from_string(":1"));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /s".to_string())), Meta::from_string("/s:1"));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /ss".to_string())), Meta::from_string("/ss:1"));
        assert_eq!(Err(NatureError::VerifyError("unknow prefix : [/s]".to_string())), Meta::from_string("/s/s:1"));
        assert_eq!(Meta::new("/N", 1, MetaType::Null), Meta::from_string("/N:1"));
        assert_eq!(Err(NatureError::VerifyError("illegal format for `full_key` : /Na".to_string())), Meta::from_string("/Na:1"));
        assert_eq!(Meta::new("/a", 1, MetaType::Null), Meta::from_string("/N/a:1"));
        assert_eq!(Meta::new("/hello", 1, MetaType::Dynamic), Meta::from_string("/D/hello:1"));
        assert_eq!(Meta::new("/world", 1, MetaType::System), Meta::from_string("/S/world:1"));
        assert_eq!(Meta::new("/my", 1, MetaType::Business), Meta::from_string("/B/my:1"));
    }

    #[test]
    fn has_state_name_test() {
        let mut m = Meta::new("hello", 1, MetaType::Business).unwrap();
        assert_eq!(m.has_state_name("a"), false);
        m.set_states(Some(vec![State::Normal("a".to_string())]));
        assert_eq!(m.has_state_name("a"), true);
        assert_eq!(m.has_state_name("b"), false);
    }

    #[test]
    fn meta_string_test() {
        let m = Meta::new("hello", 1, MetaType::Business).unwrap();
        assert_eq!(m.meta_string(), "/B/hello:1");
    }
}

#[cfg(test)]
mod verify_test {
    use super::*;

    #[test]
    fn not_a_state_meta() {
        let meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        let mut set: HashSet<String> = HashSet::new();
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn, Err(NatureError::VerifyError("[/B/hello:1] is not a state meta".to_string())))
    }

    #[test]
    fn none_states() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        let setting = serde_json::to_string(&MetaSetting {
            is_state: true,
            is_empty_content: false,
        }).unwrap();
        meta.set_setting(&setting);
        let mut set: HashSet<String> = HashSet::new();
        set.insert("a".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn, Err(NatureError::VerifyError("[a] does not defined in meta: /B/hello:1".to_string())))
    }

    #[test]
    fn simple() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        match State::string_to_states("a") {
            Ok((ss, _)) => meta.set_states(Some(ss)),
            _ => { panic!("should have some") }
        };
        let mut set: HashSet<String> = HashSet::new();
        set.insert("a".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn, Ok(()))
    }

    #[test]
    fn pure_parent() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        match State::string_to_states("a1,a2,p1[a3,p2[p3[a,b,c]]]") {
            Ok((ss, _)) => meta.set_states(Some(ss)),
            _ => { panic!("should have some") }
        };
        dbg!(&meta);
        let mut set: HashSet<String> = HashSet::new();
        set.insert("d".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.is_err(), true);
        set.clear();
        set.insert("b".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.is_ok(), true);
    }

    #[test]
    fn simple_mutex() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        match State::string_to_states("a|b") {
            Ok((ss, _)) => meta.set_states(Some(ss)),
            _ => { panic!("should have some") }
        };
        let mut set: HashSet<String> = HashSet::new();
        set.insert("b".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.is_ok(), true);
        set.insert("a".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.err().unwrap().to_string().contains("mutex conflict"), true);
    }

    #[test]
    fn parent_in_mutex() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        match State::string_to_states("a|b[c|d,e]]") {
            Ok((ss, _)) => meta.set_states(Some(ss)),
            _ => { panic!("should have some") }
        };
        let mut set: HashSet<String> = HashSet::new();
        set.insert("a".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.is_ok(), true);

        set.insert("c".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.err().unwrap().to_string().contains("mutex conflict"), true);

        set.clear();
        set.insert("a".to_string());
        set.insert("d".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.err().unwrap().to_string().contains("mutex conflict"), true);

        set.clear();
        set.insert("c".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.is_ok(), true);

        set.insert("d".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.err().unwrap().to_string().contains("mutex conflict"), true);

        set.clear();
        set.insert("c".to_string());
        set.insert("e".to_string());
        let rtn = meta.verify_state(&set);
        assert_eq!(rtn.is_ok(), true);
    }
}
