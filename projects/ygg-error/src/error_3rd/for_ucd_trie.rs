use crate::YggdrasilError;
use ucd_trie::Error;

impl From<Error> for YggdrasilError {
    fn from(e: Error) -> Self {
        YggdrasilError::language_error(e.to_string())
    }
}
