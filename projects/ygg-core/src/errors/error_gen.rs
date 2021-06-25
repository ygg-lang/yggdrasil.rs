use super::Error;

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::ParsingError { error: e.to_string() }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParsingError { error: e.to_string() }
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Self::Unreachable
    }
}
