use alloc::vec::Vec;
use core::slice::Iter;

use super::*;

pub struct SequenceBuilder {
    buffer: Vec<Character>,
}

pub struct SequenceView<'i> {
    utf8: Peekable<Iter<'i, Character>>,
}

impl SequenceBuilder {
    pub fn build(&self) -> SequenceView {
        SequenceView { utf8: self.buffer.iter().peekable() }
    }
}

impl<'i> SequenceView<'i> {
    pub fn new(s: &'i [Character]) -> Self {
        Self { utf8: s.iter().peekable() }
    }
}

impl<'i> InputStream for SequenceView<'i> {
    fn read<R>(&mut self) -> Result<Character, YggdrasilError<R>> {
        match self.utf8.next() {
            Some(s) => Ok(*s),
            None => {
                todo!()
            }
        }
    }

    fn peek<R>(&mut self) -> Result<Character, YggdrasilError<R>> {
        match self.utf8.peek() {
            Some(s) => Ok(**s),
            None => {
                todo!()
            }
        }
    }
}
