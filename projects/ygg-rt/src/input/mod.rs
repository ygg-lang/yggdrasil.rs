#![doc = include_str!("readme.md")]

use core::{
    fmt::{Display, Formatter, Write},
    iter::Peekable,
    str::Chars,
};

use crate::YggdrasilError;

pub use self::{
    adapt_utf8::Utf8View,
    adapt_vec::{SequenceBuilder, SequenceView},
};

mod adapt_utf8;
mod adapt_vec;

mod adapt_reader;

pub trait InputStream {
    fn read<R>(&mut self) -> Result<Character, YggdrasilError<R>>;
    fn peek<R>(&mut self) -> Result<Character, YggdrasilError<R>>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Character {
    pub unicode: char,
    pub offset: u32,
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char(self.unicode)
    }
}
