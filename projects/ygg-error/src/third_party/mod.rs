use crate::{YggdrasilError, YggdrasilErrorKind};

#[cfg(feature = "antlr-rust")]
mod for_antlr;
#[cfg(feature = "askama")]
mod for_askama;

impl From<std::io::Error> for YggdrasilError {
    fn from(value: std::io::Error) -> Self {
        YggdrasilError { kind: Box::new(YggdrasilErrorKind::Io { error: value.to_string(), file: None }) }
    }
}
