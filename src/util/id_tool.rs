extern crate itertools;

use ::Result;
use serde::Serialize;
use serde_json;
use uuid::*;

#[inline]
pub fn generate_id<T: ?Sized + Serialize>(value: &T) -> Result<u128> {
    let json = serde_json::to_string(value)?;
    let uuid = Uuid::new_v3(&NAMESPACE_DNS, &json);
    Ok(u128::from_ne_bytes(*uuid.as_bytes()))
}

#[inline]
pub fn u128_to_vec_u8(value: u128) -> Vec<u8> {
    u128::to_ne_bytes(value).to_vec()
}

#[inline]
pub fn vec_to_u128(vec: &Vec<u8>) -> u128 {
    let mut arr = [0u8; 16];
    for i in 0..16 {
        arr[i] = vec[i];
    }
    u128::from_ne_bytes(arr)
}

#[inline]
pub fn vec_to_hex_string(vec: &Vec<u8>) -> String {
    use self::itertools::Itertools;
    vec.iter().format("", |e, f| f(&format_args!("{:02x}", e))).to_string()
}

#[cfg(test)]
mod test {
    use util::id_tool::vec_to_hex_string;

    #[test]
    fn vec_to_hex_string_test() {
        let string = vec_to_hex_string(&vec!(1, 2, 3, 16));
        assert_eq!(string, "01020310");
        let string = vec_to_hex_string(&vec!(1, 2, 3, 15));
        assert_eq!(string, "0102030f");
    }
}
