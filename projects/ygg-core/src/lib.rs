#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(trivial_bounds)]
#![feature(async_closure)]


pub use self::{
    manager::{FILE_MANAGER, HINT_MANAGER},
};
pub use self::errors::{Result, Error};

pub mod codegen;
pub mod manager;

mod frontend;
mod errors;

