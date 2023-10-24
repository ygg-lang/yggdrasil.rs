use diagnostic::{FileID, FileSpan};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
    path::PathBuf,
};

pub mod file_span;

#[derive(Clone, Debug)]
pub struct YggdrasilError {
    pub(crate) kind: Box<YggdrasilErrorKind>,
}

#[derive(Clone, Debug)]
pub enum YggdrasilErrorKind {
    Io { error: String, file: Option<PathBuf> },
    Runtime { message: String },
    Config { message: String },
    Syntax { message: String, span: FileSpan },
}

impl Error for YggdrasilError {}

impl Error for YggdrasilErrorKind {}

impl YggdrasilError {
    pub fn kind(&self) -> &YggdrasilErrorKind {
        &*self.kind
    }
}

impl Display for YggdrasilError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for YggdrasilErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YggdrasilErrorKind::Io { error, file } => match file {
                Some(s) => {
                    write!(f, "{} at {}", error, s.display())
                }
                None => {
                    write!(f, "{}", error)
                }
            },
            YggdrasilErrorKind::Runtime { message } => {
                write!(f, "RuntimeError: {}", message)
            }
            YggdrasilErrorKind::Config { message } => {
                write!(f, "ConfigError: {}", message)
            }
            YggdrasilErrorKind::Syntax { message, span: range } => {
                write!(f, "SyntaxError: {} at {:?}", message, range.get_range())
            }
        }
    }
}

impl YggdrasilError {
    pub fn config_error<S: Display>(message: S) -> Self {
        Self { kind: Box::new(YggdrasilErrorKind::Config { message: message.to_string() }) }
    }
    pub fn runtime_error<S: Display>(message: S) -> Self {
        Self { kind: Box::new(YggdrasilErrorKind::Runtime { message: message.to_string() }) }
    }
    pub fn syntax_error<S: Display>(message: S, range: Range<usize>) -> Self {
        Self {
            kind: Box::new(YggdrasilErrorKind::Syntax {
                message: message.to_string(),
                span: unsafe { FileID::new(0) }.with_range(range),
            }),
        }
    }
}
