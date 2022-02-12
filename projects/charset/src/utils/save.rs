use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

use crate::CharacterSet;

impl CharacterSet {
    pub fn dump() -> String {
        let mut buffer = String::new();

        return buffer;
    }
}

impl Serialize for CharacterSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.count()))?;
        for element in self.to_ranges() {
            seq.serialize_element(&element)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for CharacterSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        // deserializer.deserialize_seq()?;
        todo!()
    }
}
