use super::*;

impl CharacterSet {
    pub fn dump(&self, name: &str, public: bool) -> Result<String, std::fmt::Error> {
        let ranges = self.to_ranges();
        let mut w = String::new();
        let public = if public { "pub " } else { "" };
        writeln!(w, "{}const {}: &'static [(char, char); {}] = &[", public, name, ranges.len())?;
        for range in ranges {
            writeln!(w, "    ({:?}, {:?}),", range.start, range.end)?;
        }
        write!(w, "];")?;
        Ok(w)
    }
    pub fn dump_tree(&self, name: &str, public: bool) -> Result<String, std::fmt::Error> {
        let binding = self.compress();
        let slice = binding.as_slice();
        let mut w = String::new();
        let public = if public { "pub " } else { "" };
        writeln!(w, "{}const {}: TrieSet = TrieSet {{", public, name)?;
        writeln!(w, "    tree1_level1: &{:?},", slice.tree1_level1)?;
        writeln!(w, "    tree2_level1: &{:?},", slice.tree2_level1)?;
        writeln!(w, "    tree2_level2: &{:?},", slice.tree2_level2)?;
        writeln!(w, "    tree3_level1: &{:?},", slice.tree3_level1)?;
        writeln!(w, "    tree3_level2: &{:?},", slice.tree3_level2)?;
        writeln!(w, "    tree3_level3: &{:?},", slice.tree3_level3)?;
        write!(w, "}};")?;
        Ok(w)
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
