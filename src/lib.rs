#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use callback::*;
pub use converter::*;
pub use error::*;
pub use from_instance::*;
pub use instance::*;
pub use instance_para::*;
pub use meta_setting::*;
pub use meta_type::*;
pub use query::*;
pub use settings::*;
pub use state::*;
pub use target_state::*;
pub use util::*;

pub use crate::meta::*;

mod converter;
mod error;
mod instance;
mod meta;
mod meta_type;
mod meta_setting;
mod util;
mod state;
mod query;
mod target_state;
mod callback;
mod from_instance;
mod settings;
mod instance_para;


pub type Result<T> = std::result::Result<T, NatureError>;

#[cfg(feature = "id64")]
pub type ID = u64;

#[cfg(feature = "id128")]
pub type ID = u128;