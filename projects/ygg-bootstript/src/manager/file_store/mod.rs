use super::*;


pub struct FileStore {
   pub fingerprint: u128,
    pub data: FileType,
}

pub struct FileFingerprint {
    pub fingerprint: u128,
    pub  text: String
}

pub enum FileType {
    Grammar(),
    Fragment(),
    TypeDefine(),
}


impl FileStore {
    pub fn load_url(url: &Url, f: FileFingerprint) -> Result<Self> {
        let FileFingerprint { fingerprint, text } = f;
        let path = url.to_file_path()?;
        let data = match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => Self::parse_toml(text),
            Some("ygg") | Some("yg") => Self::parse_ygg(text),
            _ => Err(YGGError::IOError { error: String::from("Unsupported file extension") }),
        }?;
        Ok(Self { fingerprint , data })
    }
    pub fn parse_toml(_input: String) -> Result<FileType> {
        unimplemented!()
    }
    pub fn parse_ygg(_input: String) -> Result<FileType> {
        unimplemented!()
    }
}

impl FileFingerprint {
    pub fn new(url:&Url) -> Result<Self> {
        let input = fs::read_to_string(url.to_file_path()?)?;
        let mut bytes = input.clone().into_bytes();
        bytes.extend_from_slice(url.as_str().as_bytes());
        Ok(Self {
            fingerprint: xxh3_128(&bytes),
            text: input
        })
    }
}

impl PartialEq<FileFingerprint> for FileStore {
    fn eq(&self, other: &FileFingerprint) -> bool {
        self.fingerprint == other.fingerprint
    }
}