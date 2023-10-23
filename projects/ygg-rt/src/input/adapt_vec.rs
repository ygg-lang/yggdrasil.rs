use alloc::vec::Vec;
use core::marker::PhantomData;

use super::*;

///
pub struct SequenceBuilder {
    buffer: Vec<Character>,
}

#[derive(Clone)]
pub struct SequenceView<'i, I>
where
    I: Iterator<Item = Character>,
{
    utf8: Peekable<I>,
    phantom: PhantomData<&'i ()>,
}

impl SequenceBuilder {
    pub fn build<'i>(&'i self) -> SequenceView<impl Iterator<Item = Character> + 'i> {
        SequenceView::new(self.buffer.iter().map(|f| *f))
    }
}

impl<'i, I> SequenceView<'i, I>
where
    I: Iterator<Item = Character>,
{
    pub fn new(s: I) -> Self {
        Self { utf8: s.peekable(), phantom: Default::default() }
    }
}

impl<'i, I> InputStream for SequenceView<'i, I>
where
    I: Iterator<Item = Character> + Clone,
{
    fn read<R>(&mut self) -> Result<Character, YggdrasilError<R>> {
        match self.utf8.next() {
            Some(s) => Ok(s),
            None => {
                todo!()
            }
        }
    }

    fn peek<R>(&mut self) -> Result<Character, YggdrasilError<R>> {
        match self.utf8.peek() {
            Some(s) => Ok(*s),
            None => {
                todo!()
            }
        }
    }
}
