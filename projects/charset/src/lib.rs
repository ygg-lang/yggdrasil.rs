#![doc = include_str!("../Readme.md")]

pub use self::{insert::InsertAction, utils::CharacterSet};

pub mod builtin;
mod dump;
mod insert;
mod utils;
