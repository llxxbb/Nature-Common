use std::collections::btree_set::BTreeSet;
use std::str::FromStr;

use crate::{Instance, is_false, NatureError, Result};

#[derive(Debug, Clone, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct MetaSetting {
    pub is_state: bool,
    /// Only useful for state-meta.
    /// A meta_string, this meta instance's id will use its master instance's id.
    /// As a target meta, if no `executor` appointed. an auto-converter will be created.
    pub master: Option<String>,
    /// `MetaSettingTemp#multi_meta` can't use BTreeSet type,
    /// so make this struct for it,
    /// it would be good performance for multi_meta verify.
    /// each of the item's format is meta-type:business-key:version
    pub multi_meta: BTreeSet<String>,
    /// Nature will cache the saved instance for a while, and check before saving the following same instances.
    pub conflict_avoid: bool,
}

impl From<MetaSettingTemp> for MetaSetting {
    fn from(input: MetaSettingTemp) -> Self {
        MetaSetting {
            is_state: input.is_state,
            master: input.master,
            multi_meta: {
                let mut rtn = BTreeSet::<String>::new();
                input.multi_meta.into_iter().for_each(|one| { rtn.insert(one); });
                rtn
            },
            conflict_avoid: input.conflict_avoid,
        }
    }
}

impl FromStr for MetaSetting {
    type Err = NatureError;

    fn from_str(s: &str) -> Result<Self> {
        let tmp: MetaSettingTemp = serde_json::from_str(s)?;
        Ok(tmp.into())
    }
}

impl From<MetaSetting> for MetaSettingTemp {
    fn from(input: MetaSetting) -> Self {
        MetaSettingTemp {
            is_state: input.is_state,
            master: input.master,
            multi_meta: {
                let mut rtn: Vec<String> = vec![];
                input.multi_meta.into_iter().for_each(|one| { rtn.push(one); });
                rtn
            },
            conflict_avoid: input.conflict_avoid,
        }
    }
}

impl MetaSetting {
    pub fn check_multi_meta(&self, instances: &Vec<Instance>) -> Result<()> {
        for instance in instances {
            if !self.multi_meta.contains(&instance.meta) {
                let msg = format!("undefined meta:{} ", instance.meta);
                return Err(NatureError::VerifyError(msg));
            }
        }
        Ok(())
    }

    pub fn to_json(&self) -> Result<String> {
        let temp = MetaSettingTemp::from(self.clone());
        let rtn = serde_json::to_string(&temp)?;
        Ok(rtn)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
struct MetaSettingTemp {
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_state: bool,
    /// Only useful for state-meta.
    /// A meta_string, this meta instance's id will use its master instance's id.
    /// As a target meta, if no `executor` appointed. an auto-converter will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub master: Option<String>,
    /// each of the item's format is meta-type:business-key:version
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub multi_meta: Vec<String>,
    /// Nature will cache the saved instance for a while, and check before saving the following same instances.
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub conflict_avoid: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn master_test(){
        let mut  set = MetaSettingTemp::default();
        set.master = Some("B:from:1".to_string());
        let result = serde_json::to_string(&set).unwrap();
        assert_eq!(result,r#"{"master":"B:from:1"}"#)
    }

    #[test]
    fn check_multi_meta() {
        let mut set: BTreeSet<String> = BTreeSet::new();
        set.insert("B:a:1".to_string());
        set.insert("B:b:1".to_string());

        let ms = MetaSetting {
            is_state: false,
            master: None,
            multi_meta: set,
            conflict_avoid: false,
        };
        let a = Instance::new("a").unwrap();
        let b = Instance::new("b").unwrap();
        let c = Instance::new("d").unwrap();
        assert_eq!(ms.check_multi_meta(&vec![a.clone()]).is_ok(), true);
        assert_eq!(ms.check_multi_meta(&vec![b.clone()]).is_ok(), true);
        assert_eq!(ms.check_multi_meta(&vec![a.clone(), b.clone()]).is_ok(), true);
        assert_eq!(ms.check_multi_meta(&vec![c.clone()]).is_err(), true);
        assert_eq!(ms.check_multi_meta(&vec![c.clone(), a.clone()]).is_err(), true);
        assert_eq!(ms.check_multi_meta(&vec![a.clone(), c.clone()]).is_err(), true);
        assert_eq!(ms.check_multi_meta(&vec![b.clone(), c.clone()]).is_err(), true);
        assert_eq!(ms.check_multi_meta(&vec![c.clone(), b.clone()]).is_err(), true);
        assert_eq!(ms.check_multi_meta(&vec![a, b, c]).is_err(), true);
    }
}