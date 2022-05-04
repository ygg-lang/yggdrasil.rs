use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use diagnostic::DiagnosticLevel;

use self::YggdrasilErrorKind::*;

pub mod error_std;

pub type YggdrasilResult<T = ()> = Result<T, YggdrasilError>;

pub type Validation<T> = diagnostic::Validation<T, YggdrasilError>;

#[derive(Debug)]
pub struct YggdrasilError {
    pub error: Box<YggdrasilErrorKind>,
    pub level: DiagnosticLevel,
}

#[derive(Debug)]
pub enum YggdrasilErrorKind {
    IOError(std::io::Error),
    FormatError(std::fmt::Error),
    LanguageError(String),
    StructureError(String),
    UnexpectedToken(String),
    InfoMissing(String),
    /// Some nodes failed to resolve and are being rolled back
    Unwinding,
    /// A forbidden cst_node encountered
    Unreachable,
}

impl YggdrasilError {}

impl YggdrasilError {}

impl Error for YggdrasilError {}

impl Display for YggdrasilError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for YggdrasilErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
