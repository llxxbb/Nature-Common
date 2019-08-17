#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub use convertor::*;
pub use error::*;
pub use instance::*;
pub use self::meta::*;
pub use meta_type::*;
pub use parallel_batch_instance::*;
pub use serial_batch_instance::*;
pub use state::*;
pub use util::*;

mod convertor;
mod error;
mod instance;
mod meta;
mod meta_type;
mod parallel_batch_instance;
mod serial_batch_instance;
mod state;

pub mod util;

pub type Result<T> = std::result::Result<T, NatureError>;