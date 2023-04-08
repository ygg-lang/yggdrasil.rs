use crate::YggdrasilError;
use wax::{BuildError, WalkError};

impl From<BuildError> for YggdrasilError {
    fn from(value: BuildError) -> Self {
        YggdrasilError::runtime_error(value)
    }
}

impl From<WalkError> for YggdrasilError {
    fn from(value: WalkError) -> Self {
        YggdrasilError::runtime_error(value)
    }
}
