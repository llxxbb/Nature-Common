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
pub use parallel_batch_instance::*;
pub use serial_batch_instance::*;
pub use thing::*;
pub use util::*;
pub use thing_type::*;

mod convertor;
mod error;
mod instance;
mod thing;
mod thing_type;
mod parallel_batch_instance;
mod serial_batch_instance;

pub mod util;

pub type Result<T> = std::result::Result<T, NatureError>;