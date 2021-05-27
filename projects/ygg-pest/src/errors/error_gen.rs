use super::YGGError as ThisError;

impl From<std::str::Utf8Error> for ThisError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::FormatError { error: e.to_string() }
    }
}

impl From<std::num::ParseIntError> for ThisError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::FormatError { error: e.to_string() }
    }
}

impl From<()> for ThisError {
    fn from(_: ()) -> Self {
        Self::UnknownError
    }
}
