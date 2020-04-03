use crate::{Instance, is_false, NatureError, Result};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct MetaSetting {
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
}

#[cfg(test)]
mod test {
    use crate::{Instance, MetaSetting};

    #[test]
    fn check_multi_meta() {
        let ms = MetaSetting {
            is_state: false,
            master: None,
            multi_meta: vec!["B:a:1".to_string(), "B:b:1".to_string()],
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