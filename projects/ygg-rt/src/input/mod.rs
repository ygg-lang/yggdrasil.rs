#![doc = include_str!("readme.md")]

use core::{
    fmt::{Display, Formatter, Write},
    iter::Peekable,
    ops::RangeInclusive,
};

use crate::{AhoCorasick, YggdrasilError, YggdrasilRule};

pub use self::{
    adapt_utf8::Utf8View,
    adapt_vec::{SequenceBuilder, SequenceView},
};

mod adapt_utf8;
mod adapt_vec;

mod adapt_reader;

pub trait InputStream {
    fn match_start_of_input<R>(&mut self) -> Result<(), YggdrasilError<R>>
    where
        R: YggdrasilRule;
    fn match_end_of_input<R>(&mut self) -> Result<usize, YggdrasilError<R>>
    where
        R: YggdrasilRule;

    fn match_char<R>(&mut self, target: char) -> Result<char, YggdrasilError<R>>
    where
        R: YggdrasilRule,
    {
        self.match_char_if(|c| c.eq(&target))
    }

    fn match_char_if<R, F>(&mut self, condition: F) -> Result<char, YggdrasilError<R>>
    where
        R: YggdrasilRule,
        F: FnOnce(char) -> bool;
    fn match_char_until<R, F>(&mut self, condition: F) -> Result<char, YggdrasilError<R>>
    where
        R: YggdrasilRule,
        F: FnOnce(char) -> bool,
    {
        self.match_char_if(|c| !condition(c))
    }
    fn match_char_any<R>(&mut self) -> Result<char, YggdrasilError<R>>
    where
        R: YggdrasilRule,
    {
        self.match_char_if(|_| true)
    }
    fn match_char_range<R>(&mut self, range: RangeInclusive<char>) -> Result<char, YggdrasilError<R>>
    where
        R: YggdrasilRule,
    {
        self.match_char_if(|c| range.contains(&c))
    }
    fn match_str<R>(&mut self, target: &str, case: bool) -> Result<&str, YggdrasilError<R>>;

    /// Match a string group
    ///
    /// [`AhoCorasick`] is equivalent to `&[&str]`, but much faster and support precompiled.
    fn match_str_group<R>(&mut self, target: AhoCorasick) -> Result<&str, YggdrasilError<R>>;

    fn match_fn<R, F>(&mut self, parser: F) -> Result<&str, YggdrasilError<R>>
    where
        R: YggdrasilRule;

    fn match_optional<R, F>(&mut self, parser: F) -> Result<&str, YggdrasilError<R>>
    where
        R: YggdrasilRule;

    fn match_repeats<R, F>(&mut self, parser: F, repeats: RangeInclusive<u32>) -> Result<&str, YggdrasilError<R>>
    where
        R: YggdrasilRule;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Character {
    /// The character id
    pub unicode: char,
    /// start offset of character
    pub offset: u32,
    /// In different encodings, the space occupied by one byte is also different.
    pub length: u32,
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char(self.unicode)
    }
}
