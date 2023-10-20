use crate::YggdrasilError;
use num::bigint::ParseBigIntError;

impl From<ParseBigIntError> for YggdrasilError {
    fn from(value: ParseBigIntError) -> Self {
        YggdrasilError::syntax_error(value, 0..0)
    }
}
