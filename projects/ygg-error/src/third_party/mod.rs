use crate::{YggdrasilError, YggdrasilErrorKind};

impl From<std::io::Error> for YggdrasilError {
    fn from(value: std::io::Error) -> Self {
        YggdrasilError { kind: Box::new(YggdrasilErrorKind::Io { error: value.to_string(), file: None }) }
    }
}
