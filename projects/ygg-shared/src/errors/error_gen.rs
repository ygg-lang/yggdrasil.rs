use super::Error;

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::AstError { error: e.to_string() }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::AstError { error: e.to_string() }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Self::AstError { error: e.to_string() }
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Self::Unreachable
    }
}
