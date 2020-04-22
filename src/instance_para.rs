use crate::{DEFAULT_PARA_SEPARATOR, NatureError, Result};

pub fn get_para_and_key_from_para(para: &str, part: &Vec<u8>) -> Result<(String, String)> {
    let sep: &str = &DEFAULT_PARA_SEPARATOR;
    let keys: Vec<&str> = para.split(&sep).collect();
    make_key_and_para(&keys, part, &sep)
}

/// key for instance'content, para for instance's para
pub fn make_key_and_para(keys: &Vec<&str>, k_index: &Vec<u8>, sep: &str) -> Result<(String, String)> {
    // make instance's para
    let mut p: Vec<&str> = vec![];
    for index in k_index {
        let index = *index as usize;
        if index >= keys.len() {
            return Err(NatureError::VerifyError("outbound index".to_string()));
        }
        p.push(keys[index]);
        p.push(sep);
    }
    let p = p[..p.len() - 1].concat();

    // make key
    let mut k: Vec<&str> = vec![];
    for i in 0..keys.len() {
        if k_index.contains(&(i as u8)) {
            continue;
        }
        k.push(keys[i]);
        k.push(sep);
    }
    let k = match k.len() {
        0 => "".to_string(),
        _ => k[..k.len() - 1].concat()
    };
    Ok((p, k))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn key_para_make() {
        let keys = vec!["a", "b", "c", "d", "e"];
        let idx = vec![3, 1];
        let result = make_key_and_para(&keys, &idx, "-").unwrap();
        assert_eq!(result.0, "d-b");
        assert_eq!(result.1, "a-c-e");
    }
}