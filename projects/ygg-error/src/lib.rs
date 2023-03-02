#![feature(try_blocks)]
#![feature(return_position_impl_trait_in_trait)]

// noinspection DuplicatedCode
mod errors;
mod third_party;

pub use crate::errors::{YggdrasilError, YggdrasilErrorKind};

pub type Validation<T> = validatus::Validation<T, YggdrasilError>;
pub type Result<T> = core::result::Result<T, YggdrasilError>;

pub use validatus::{
    Validate,
    Validation::{Failure, Success},
};
