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
    iterator: Peekable<I>,
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
        Self { iterator: s.peekable(), phantom: Default::default() }
    }
}
