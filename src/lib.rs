#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
#[cfg(test)] extern crate mockers;
#[cfg(test)] extern crate mockers_derive;

pub use convertor::*;
pub use error::*;
pub use instance::*;
pub use thing::*;
pub use util::*;
pub use thing_type::*;

mod convertor;
mod error;
mod instance;
mod thing;
mod thing_type;

pub mod util;

pub type Result<T> = std::result::Result<T, NatureError>;