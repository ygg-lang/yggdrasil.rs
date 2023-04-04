use super::*;
use serde::de::{Error, Visitor};
use std::fmt::Formatter;

struct ModeReader {}

#[automatically_derived]
impl<'de> Deserialize<'de> for SupportedMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ModeReader {})
    }
}

impl<'de> Visitor<'de> for ModeReader {
    type Value = SupportedMode;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("accepts, `rs`, `ts`,`vscode`, `kotlin`, `jetbrain`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(match v.to_ascii_lowercase().as_str() {
            "rs" | "rust" => SupportedMode::Rust,
            _ => return Err(E::unknown_variant(v, &["rust", "typescript", "kotlin"])),
        })
    }
}
