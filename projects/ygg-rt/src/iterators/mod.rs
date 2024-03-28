use crate::{
    span::TextSpan, InvalidTag, TokenPair, TokenQueue, TokenStream, TokenTree, Tokens, YggdrasilError, YggdrasilNode,
    YggdrasilRule,
};
use alloc::{borrow::ToOwned, format, rc::Rc, string::String, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    iter::Filter,
    ptr,
};

pub mod token_pair;
pub mod token_queue;
pub mod token_stream;
pub mod token_tree;
pub mod tokens;

#[cfg(not(feature = "dynamic"))]
pub type Tag = &'static str;
#[cfg(feature = "dynamic")]
pub type Tag = alloc::borrow::Cow<'static, str>;
