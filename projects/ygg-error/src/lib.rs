#![feature(try_blocks)]

// noinspection DuplicatedCode
mod errors;
pub mod third_party;

pub use crate::errors::{YggdrasilError, YggdrasilErrorKind};
pub use diagnostic::{FileCache, FileID, FileSpan};
pub type Validation<T> = validatus::Validation<T, YggdrasilError>;
pub type Result<T> = core::result::Result<T, YggdrasilError>;

pub use validatus::{
    Validate,
    Validation::{Failure, Success},
};
