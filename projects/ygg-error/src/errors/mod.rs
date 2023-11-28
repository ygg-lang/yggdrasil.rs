use diagnostic::{Diagnostic, DiagnosticBuilder, FileID, FileSpan, ReportKind, Url};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
};
#[derive(Clone, Debug)]
pub struct YggdrasilError {
    pub(crate) kind: Box<YggdrasilErrorKind>,
}

#[derive(Clone, Debug)]
pub enum YggdrasilErrorKind {
    Io { error: String, file: Option<Url> },
    Runtime { message: String },
    Config { message: String },
    Syntax { message: String, hint: String, span: FileSpan },
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
                    write!(f, "{} at {}", error, s.as_str())
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
            YggdrasilErrorKind::Syntax { message, hint: _, span: range } => {
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
                hint: "".to_string(),
                span: FileID::default().with_range(range),
            }),
        }
    }
}

impl YggdrasilError {
    pub fn with_file(mut self, file: FileID) -> Self {
        self.kind.set_file(file);
        self
    }
}

impl YggdrasilError {
    pub fn as_report(&self) -> Diagnostic {
        self.kind.as_report(Diagnostic::new(ReportKind::Error))
    }
}

impl From<YggdrasilErrorKind> for YggdrasilError {
    fn from(value: YggdrasilErrorKind) -> Self {
        Self { kind: Box::new(value) }
    }
}

impl YggdrasilErrorKind {
    fn set_file(&mut self, file: FileID) {
        match self {
            Self::Io { .. } => {}
            Self::Runtime { .. } => {}
            Self::Config { .. } => {}
            Self::Syntax { span, .. } => span.set_file(file),
        }
    }
    fn as_report(&self, builder: DiagnosticBuilder) -> Diagnostic {
        match self {
            Self::Io { error: message, file: _ } => builder.with_message(message).finish(),
            Self::Runtime { message } => builder.with_message(message).finish(),
            Self::Config { message } => builder.with_message(message).finish(),
            Self::Syntax { message, hint, span } => builder
                .with_location(span.get_file(), Some(span.get_start()))
                .with_message(message.to_string())
                .with_label(span.as_label(hint.to_string()))
                .finish(),
        }
    }
}
