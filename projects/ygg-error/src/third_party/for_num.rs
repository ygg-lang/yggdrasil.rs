use crate::YggdrasilError;
use std::num::ParseIntError;

#[cfg(feature = "num")]
use num::bigint::ParseBigIntError;

#[cfg(feature = "num")]
impl From<ParseBigIntError> for YggdrasilError {
    fn from(value: ParseBigIntError) -> Self {
        YggdrasilError::syntax_error(value, 0..0)
    }
}

impl From<ParseIntError> for YggdrasilError {
    fn from(value: ParseIntError) -> Self {
        YggdrasilError::syntax_error(value, 0..0)
    }
}
