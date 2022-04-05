use ropey::Error;

use crate::YggdrasilError;

impl From<Error> for YggdrasilError {
    fn from(e: Error) -> Self {
        Self::language_error(e.to_string())
    }
}
