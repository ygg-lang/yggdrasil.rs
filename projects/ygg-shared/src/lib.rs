extern crate thiserror;
// pub mod records;

pub mod macros;
pub mod traits;
pub mod records;

mod errors;

pub use self::errors::{Error,Result};