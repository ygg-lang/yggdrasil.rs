use crate::{YggdrasilError, YggdrasilErrorKind};
use std::sync::PoisonError;

#[cfg(feature = "antlr-rust")]
mod for_antlr;
#[cfg(feature = "askama")]
mod for_askama;

#[cfg(feature = "json5")]
mod for_json5;

#[cfg(feature = "num")]
mod for_num;
#[cfg(feature = "yggdrasil-rt")]
mod for_runtime;
#[cfg(feature = "wax")]
mod for_wax;
impl From<std::io::Error> for YggdrasilError {
    fn from(value: std::io::Error) -> Self {
        YggdrasilError { kind: Box::new(YggdrasilErrorKind::Io { error: value.to_string(), file: None }) }
    }
}

impl<T> From<PoisonError<T>> for YggdrasilError {
    fn from(value: PoisonError<T>) -> Self {
        YggdrasilError::runtime_error(value)
    }
}
