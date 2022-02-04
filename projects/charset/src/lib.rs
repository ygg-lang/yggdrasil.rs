#![doc = include_str!("../Readme.md")]

pub use self::{insert::CharacterInsert, utils::CharacterSet};

pub mod builtin;
mod insert;
mod utils;
