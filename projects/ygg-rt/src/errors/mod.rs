pub type YResult<T> = Result<T, YError>;

pub struct YError {
    kind: Box<YErrorKind>,
}

pub enum YErrorKind {
    IoError { message: String, path: String },
    ParseError { message: String },
    OtherError,
}

impl YError {
    pub fn syntax_error<S: ToString>(message: S) -> Self {
        Self { kind: box YErrorKind::ParseError { message: message.to_string() } }
    }
}
