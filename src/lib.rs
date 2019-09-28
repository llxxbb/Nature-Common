#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub use converter::*;
pub use error::*;
pub use fetch_condition::*;
pub use instance::*;
pub use meta_setting::*;
pub use meta_string::*;
pub use meta_type::*;
pub use parallel_batch_instance::*;
pub use query::*;
pub use serial_batch_instance::*;
pub use state::*;
pub use target_state::*;
pub use util::*;

pub use self::meta::*;

mod converter;
mod error;
mod instance;
mod meta;
mod meta_type;
mod meta_setting;
mod meta_string;
mod parallel_batch_instance;
mod serial_batch_instance;
mod util;
mod state;
mod fetch_condition;
mod query;
mod target_state;

pub type Result<T> = std::result::Result<T, NatureError>;