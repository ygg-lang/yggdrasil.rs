use crate::YggdrasilError;
use askama::Error;

impl From<Error> for YggdrasilError {
    fn from(value: Error) -> Self {
        YggdrasilError::runtime_error(value)
    }
}
