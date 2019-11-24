/// This is only used for serialize
pub fn is_one(num: &i32) -> bool {
    *num == 1
}

pub fn one() -> i32 { 1 }

/// This is only used for serialize
pub fn is_zero(num: &i32) -> bool {
    *num == 0
}

pub fn is_false(val: &bool) -> bool {
    !val.clone()
}