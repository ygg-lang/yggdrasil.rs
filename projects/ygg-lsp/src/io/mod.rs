use std::fs;
use lspower::lsp::Url;

mod global;
mod parser;
pub use global::{initialize_global_storages, FileStateUpdate, FILE_STORAGE};
use yggdrasil_bootstript::Result;

pub fn read_url(url: &Url) -> Result<String> {
    Ok(fs::read_to_string(url.to_file_path()?)?)
}
