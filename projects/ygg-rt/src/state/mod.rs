#![doc = include_str!("readme.md")]

use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

use crate::YggdrasilError;

#[derive(Clone)]
pub struct ParseInput<I, P> {
    pub stream: Rc<RefCell<I>>,
    pub parser: P,
}

#[derive(Clone)]
pub enum ParseOutput<R, I, P> {
    /// The current status can continue
    Fine {
        state: ParseInput<I, P>,
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
