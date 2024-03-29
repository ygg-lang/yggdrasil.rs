// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use alloc::{vec, vec::Vec};
use core::ops::{Index, Range};

/// Implementation of a `Stack` which maintains popped elements and length of previous states
/// in order to rewind the stack to a previous state.
#[derive(Debug)]
pub struct Stack<T: Clone> {
    /// All elements in the stack.
    cache: Vec<T>,
    /// All elements that are in previous snapshots but may not be in the next state.
    /// They will be pushed back to `cache` if the snapshot is restored,
    /// otherwise be dropped if the snapshot is cleared.
    ///
    /// Those elements from a sequence of snapshots are stacked in one [`Vec`], and
    /// `popped.len() == lengths.iter().map(|(len, remained)| len - remained).sum()`
    popped: Vec<T>,
    /// Every element corresponds to a snapshot, and each element has two fields:
    /// - Length of `cache` when corresponding snapshot is taken (AKA `len`).
    /// - Count of elements that come from corresponding snapshot
    ///   and are still in next snapshot or current state (AKA `remained`).
    ///
    /// And `len` is never less than `remained`.
    ///
    /// On restoring, the `cache` can be divided into two parts:
    /// - `0..remained` are untouched since the snapshot is taken.
    ///
    ///   There's nothing to do with those elements. Just let them stay where they are.
    ///
    /// - `remained..cache.len()` are pushed after the snapshot is taken.
    lengths: Vec<(usize, usize)>,
}

impl<T: Clone> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Stack<T> {
    /// Creates a new `Stack`.
    pub fn new() -> Self {
        Stack { cache: vec![], popped: vec![], lengths: vec![] }
    }

    /// Returns `true` if the stack is currently empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Returns the top-most `&T` in the `Stack`.
    pub fn peek(&self) -> Option<&T> {
        self.cache.last()
    }

    /// Pushes a `T` onto the `Stack`.
    pub fn push(&mut self, elem: T) {
        self.cache.push(elem);
    }

    /// Pops the top-most `T` from the `Stack`.
    pub fn pop(&mut self) -> Option<T> {
        let len = self.cache.len();
        let popped = self.cache.pop();
        if let Some(popped) = &popped {
            if let Some((_, remained_count)) = self.lengths.last_mut() {
                // `len >= *unpopped_count`
                if len == *remained_count {
                    *remained_count -= 1;
                    self.popped.push(popped.clone());
                }
            }
        }
        popped
    }

    /// Returns the size of the stack
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Takes a snapshot of the current `Stack`.
    pub fn snapshot(&mut self) {
        self.lengths.push((self.cache.len(), self.cache.len()))
    }

    /// The parsing after the last snapshot was successful so clearing it.
    pub fn clear_snapshot(&mut self) {
        if let Some((len, unpopped)) = self.lengths.pop() {
            // Popped elements from previous state are no longer needed.
            self.popped.truncate(self.popped.len() - (len - unpopped));
        }
    }

    /// Rewinds the `Stack` to the most recent `snapshot()`. If no `snapshot()` has been taken, this
    /// function return the stack to its initial state.
    pub fn restore(&mut self) {
        match self.lengths.pop() {
            Some((len_stack, remained)) => {
                if remained < self.cache.len() {
                    // Remove those elements that are pushed after the snapshot.
                    self.cache.truncate(remained);
                }
                if len_stack > remained {
                    let rewind_count = len_stack - remained;
                    let new_len = self.popped.len() - rewind_count;
                    let recovered_elements = self.popped.drain(new_len..);
                    self.cache.extend(recovered_elements.rev());
                    debug_assert_eq!(self.popped.len(), new_len);
                }
            }
            None => {
                self.cache.clear();
                // As `self.popped` and `self.lengths` should already be empty,
                // there is no need to clear it.
                debug_assert!(self.popped.is_empty());
                debug_assert!(self.lengths.is_empty());
            }
        }
    }
}

impl<T: Clone> Index<Range<usize>> for Stack<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &[T] {
        self.cache.index(range)
    }
}
