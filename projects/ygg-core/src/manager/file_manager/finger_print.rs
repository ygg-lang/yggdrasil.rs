use super::*;

pub struct FileFingerprint {
    pub fingerprint: u128,
    pub text: String,
}

impl FileFingerprint {
    pub fn new(url: &Url) -> Result<Self> {
        let input = fs::read_to_string(url.to_file_path()?)?;
        let mut bytes = input.clone().into_bytes();
        bytes.extend_from_slice(url.as_str().as_bytes());
        Ok(Self { fingerprint: xxh3_128(&bytes), text: input })
    }
}

impl PartialEq<FileFingerprint> for FileStore {
    fn eq(&self, other: &FileFingerprint) -> bool {
        self.fingerprint == other.fingerprint
    }
}
