use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{NatureError, Result};

#[inline]
pub fn generate_id<T: Hash>(value: &T) -> Result<u64> {
    let mut s = DefaultHasher::new();
    value.hash(&mut s);
    Ok(s.finish())
}

#[inline]
pub fn id_from_hex_str(value: &str) -> Result<u64> {
    match u64::from_str_radix(value, 16) {
        Ok(rtn) => Ok(rtn),
        Err(e) => {
            let msg = format!("can't convert to id from {}, err: {}", value, e);
            warn!("{}", msg);
            Err(NatureError::VerifyError(msg))
        }
    }
}
