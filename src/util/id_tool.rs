extern crate bincode;
extern crate itertools;

use serde::Serialize;
use uuid::*;

use ::NatureError;
use ::Result;

use self::bincode::serialize;

#[inline]
pub fn generate_id<T: ?Sized + Serialize>(value: &T) -> Result<u128> {
    let vec = match serialize(value) {
        Err(e) => return Err(NatureError::SerializeError(e.to_string())),
        Ok(rtn) => rtn
    };
    let uuid = Uuid::new_v3(&Uuid::NAMESPACE_DNS, &vec);
    Ok(u128::from_ne_bytes(*uuid.as_bytes()))
}

#[inline]
pub fn u128_to_vec_u8(value: u128) -> Vec<u8> {
    u128::to_ne_bytes(value).to_vec()
}

#[inline]
#[allow(clippy::ptr_arg)]
pub fn vec_to_u128(vec: &Vec<u8>) -> u128 {
    let mut arr = [0u8; 16];
    arr[..16].clone_from_slice(&vec[..16]);
    u128::from_ne_bytes(arr)
}

#[inline]
pub fn vec_to_hex_string(vec: &[u8]) -> String {
    use self::itertools::Itertools;
    vec.iter().format_with("", |e, f| f(&format_args!("{:02x}", e))).to_string()
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
