#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
extern crate reqwest;
#[macro_use]
extern crate lazy_static;

pub use callback::*;
pub use converter::*;
pub use error::*;
pub use fetch_condition::*;
pub use from_instance::*;
pub use instance::*;
pub use self::meta::*;
pub use meta_setting::*;
pub use meta_type::*;
pub use query::*;
pub use state::*;
pub use target_state::*;
pub use util::*;
pub use settings::*;

mod converter;
mod error;
mod instance;
mod meta;
mod meta_type;
mod meta_setting;
mod util;
mod state;
mod fetch_condition;
mod query;
mod target_state;
mod callback;
mod from_instance;
mod settings;


pub type Result<T> = std::result::Result<T, NatureError>;