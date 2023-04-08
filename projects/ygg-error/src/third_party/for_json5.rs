use crate::YggdrasilError;

impl From<json5::Error> for crate::YggdrasilError {
    fn from(value: json5::Error) -> Self {
        Self::config_error(value)
    }
}
