#![doc = include_str!("../Readme.md")]

pub use self::{dump::DumpAction, insert::InsertAction};

pub mod builtin;
mod dump;
mod insert;
mod utils;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct CharacterSet {
    all: Box<[bool; 0x110000]>,
}
