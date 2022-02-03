use url::ParseError;

use crate::{YggdrasilError, YggdrasilErrorKind};

impl From<ParseError> for YggdrasilError {
    fn from(_: ParseError) -> Self {
        Self { kind: Box::new(YggdrasilErrorKind::IOError(std::io::Error::from_raw_os_error(10022))), file: None, range: None }
    }
}
