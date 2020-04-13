use std::env;

lazy_static! {
    pub static ref DEFAULT_PARA_SEPARATOR:String={
    env::var("DEFAULT_PARA_SEPARATOR").unwrap_or_else(|_| "/".to_string())
    };
}

/// This is only used for deserialize
pub fn default_para_separator() -> String { DEFAULT_PARA_SEPARATOR.to_string() }

/// This is only used for serialize
pub fn is_default_para_separator(sep: &str) -> bool {
    sep.eq(&DEFAULT_PARA_SEPARATOR.to_string())
}
