use super::*;

#[derive(Clone, Debug, Serialize)]
pub struct YccPatterns {
    patterns: Vec<String>,
}

impl Default for YccPatterns {
    fn default() -> Self {
        Self { patterns: vec!["grammars/**.ygg".to_string()] }
    }
}

impl YccPatterns {
    pub fn empty() -> Self {
        Self { patterns: vec![] }
    }
    pub fn as_glob(&self) -> Result<Any, YggdrasilError> {
        Ok(wax::any(self.patterns.iter().map(|s| s.as_str()))?)
    }
}

struct MaybeString<'i> {
    wrap: &'i mut YccPatterns,
}

impl<'de> Deserialize<'de> for YccPatterns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = YccPatterns::empty();
        deserializer.deserialize_any(MaybeString { wrap: &mut out })?;
        Ok(out)
    }
}

impl<'i, 'de> Visitor<'de> for MaybeString<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except glob patterns")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        for line in v.lines().map(|s| s.trim()) {
            if !line.is_empty() {
                self.wrap.patterns.push(line.to_string())
            }
        }
        Ok(())
    }
    // fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    // where
    //     A: SeqAccess<'de>,
    // {
    //     todo!()
    // }
}
