use crate::YggdrasilError;
use antlr_rust::errors::ANTLRError;

impl From<ANTLRError> for YggdrasilError {
    fn from(value: ANTLRError) -> Self {
        YggdrasilError::runtime_error(value)
    }
}
