use crate::YggdrasilError;
use peginator::ParseError;

impl From<ParseError> for YggdrasilError {
    fn from(_: ParseError) -> Self {
        todo!()
    }
}
