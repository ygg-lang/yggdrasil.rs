#![no_std]
// #![warn(missing_docs)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc = include_str!("../readme.md")]

extern crate alloc;

// mod vm;
mod byte_code;
mod cst_mode;
mod rules;
mod streams;

pub use crate::{
    byte_code::{Bytecode, Instruction},
    rules::{YggdrasilLanguage, YggdrasilRule},
    streams::InputStream,
};
