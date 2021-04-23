use lspower::lsp::Url;
use std::fs;

mod global;
mod parser;
pub use global::{FileStateUpdate};
use yggdrasil_bootstript::Result;

pub fn read_url(url: &Url) -> Result<String> {
    Ok(fs::read_to_string(url.to_file_path()?)?)
}
