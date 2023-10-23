#![doc = include_str!("readme.md")]

use crate::input::InputStream;
use alloc::vec::Vec;

use crate::YggdrasilError;

#[derive(Clone)]
pub struct ParseInput<I> {
    stream: I,
}

#[derive(Clone)]
pub enum ParseOutput<I, R> {
    /// The current status can continue
    Fine {
        state: ParseInput<I>,
        /// Some recoverable errors
        error: Vec<YggdrasilError<R>>,
    },
    /// Current status cannot be continued
    Fail {
        /// Unrecoverable fatal error
        fatal: YggdrasilError<R>,
        /// Some recoverable errors
        error: Vec<YggdrasilError<R>>,
    },
}

impl<I> ParseInput<I>
where
    I: InputStream,
{
    pub fn match_char<R>(mut self, target: char) -> ParseOutput<I, R> {
        // Once a char is read, it can never be read again
        match self.stream.read::<R>() {
            Ok(c) => {
                if c.unicode == target {
                    todo!()
                }
                else {
                    todo!()
                }
            }
            Err(_) => {
                todo!()
            }
        }
    }
}
