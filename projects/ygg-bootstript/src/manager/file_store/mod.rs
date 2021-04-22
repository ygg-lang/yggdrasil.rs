use crate::codegen::GrammarType;
use super::*;
pub use self::finger_print::FileFingerprint;

mod finger_print;

#[derive(Clone, Debug)]
pub struct FileStore {
    pub fingerprint: u128,
    pub data: FileType,
}



#[derive(Clone, Debug)]
pub enum FileType {
    Grammar(GrammarState),
    Type(GrammarType),
    GrammarString(String),
    TypeString(String),
}

impl FileStore {
    pub fn load_url(url: &Url, f: FileFingerprint) -> Result<Self> {
        let FileFingerprint { fingerprint, text } = f;
        let path = url.to_file_path()?;
        let data = match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => Self::parse_toml(text),
            Some("ygg") | Some("yg") => Self::parse_ygg(text),
            _ => Err(YGGError::io_error("Unsupported file extension")),
        }?;
        Ok(Self { fingerprint, data })
    }
    pub fn parse_toml(_input: String) -> Result<FileType> {
        unimplemented!()
    }
    pub fn parse_ygg(_input: String) -> Result<FileType> {
        unimplemented!()
    }
}

