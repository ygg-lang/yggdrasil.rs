#![feature(once_cell)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(trivial_bounds)]
#![feature(async_closure)]

pub use self::{
    errors::{Error, Result},
    manager::{FILE_MANAGER, HINT_MANAGER},
};

pub mod codegen;
pub mod manager;

mod errors;
mod frontend;
