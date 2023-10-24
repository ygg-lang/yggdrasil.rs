use crate::{TextSpan, YggdrasilRule};

use super::*;

#[derive(Clone)]
pub struct Utf8View<'i> {
    utf8: &'i str,
    offset: usize,
}

#[allow(unused_variables)]
impl<'i> InputStream for Utf8View<'i> {
    fn match_start_of_input<R>(&mut self) -> Result<(), YggdrasilError<R>>
    where
        R: YggdrasilRule,
    {
        match self.offset {
            0 => Ok(()),
            _ => unsafe {
                Err(YggdrasilError::custom_error(
                    "Unexpected character",
                    TextSpan::new_unchecked(self.utf8, self.offset, self.offset),
                ))
            },
        }
    }

    fn match_end_of_input<R>(&mut self) -> Result<usize, YggdrasilError<R>>
    where
        R: YggdrasilRule,
    {
        match self.utf8.chars().next() {
            Some(c) => unsafe {
                Err(YggdrasilError::custom_error(
                    "Unexpected character",
                    TextSpan::new_unchecked(self.utf8, self.offset, self.offset + len_utf8(c) as usize),
                ))
            },
            None => Ok(self.offset),
        }
    }

    fn match_char_if<R, F>(&mut self, condition: F) -> Result<char, YggdrasilError<R>>
    where
        R: YggdrasilRule,
        F: FnOnce(char) -> bool,
    {
        match self.utf8.chars().next() {
            Some(c) => {
                let start = self.offset;
                if condition(c) {
                    self.offset += c.len_utf8();
                    Ok(c)
                }
                else {
                    unsafe {
                        Err(YggdrasilError::custom_error(
                            "Unexpected character",
                            TextSpan::new_unchecked(self.utf8, start, self.offset),
                        ))
                    }
                }
            }
            None => unsafe {
                Err(YggdrasilError::custom_error(
                    "Unexpected EOF",
                    TextSpan::new_unchecked(self.utf8, self.offset, self.offset),
                ))
            },
        }
    }

    fn match_str<R>(&mut self, target: &str, case: bool) -> Result<&str, YggdrasilError<R>> {
        todo!()
    }

    fn match_str_group<R>(&mut self, target: AhoCorasick) -> Result<&str, YggdrasilError<R>> {
        todo!()
    }

    fn match_fn<R, F>(&mut self, parser: F) -> Result<&str, YggdrasilError<R>>
    where
        R: YggdrasilRule,
    {
        todo!()
    }

    fn match_optional<R, F>(&mut self, parser: F) -> Result<&str, YggdrasilError<R>>
    where
        R: YggdrasilRule,
    {
        todo!()
    }

    fn match_repeats<R, F>(&mut self, parser: F, repeats: RangeInclusive<u32>) -> Result<&str, YggdrasilError<R>>
    where
        R: YggdrasilRule,
    {
        todo!()
    }
}

const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;

#[inline]
const fn len_utf8(char: char) -> u32 {
    let code = char as u32;
    if code < MAX_ONE_B {
        1
    }
    else if code < MAX_TWO_B {
        2
    }
    else if code < MAX_THREE_B {
        3
    }
    else {
        4
    }
}
