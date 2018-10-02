#![feature(int_to_from_bytes)] // this used to convert uuid to u128

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub use convertor::*;
pub use error::*;
pub use instance::*;
pub use thing::*;

mod convertor;
mod error;
mod instance;
mod thing;

pub mod util;

pub type Result<T> = std::result::Result<T, NatureError>;