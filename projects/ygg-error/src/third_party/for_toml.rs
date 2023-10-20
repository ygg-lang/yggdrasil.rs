use crate::YggdrasilError;

impl From<toml::de::Error> for YggdrasilError {
    fn from(value: toml::de::Error) -> Self {
        YggdrasilError::config_error(value)
    }
}
