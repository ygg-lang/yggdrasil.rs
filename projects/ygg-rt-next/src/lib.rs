// #![no_std]
// #![warn(missing_docs)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc = include_str!("../readme.md")]

// mod vm;
mod byte_code;
mod cst_mode;

mod streams;

pub use crate::streams::InputStream;
