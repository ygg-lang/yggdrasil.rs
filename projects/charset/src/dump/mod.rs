use std::fmt::Write;

use ucd_trie::TrieSetSlice;

use crate::CharacterSet;

#[derive(Debug, Clone)]
pub struct DumpAction {
    pub name: String,
    pub public: String,
    pub skip_fmt: bool,
    pub dump_tree: bool,
    pub dump_check: bool,
    pub dump_range: bool,
    pub trie_set: String,
}

impl Default for DumpAction {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            public: "pub".to_string(),
            skip_fmt: true,
            dump_tree: true,
            dump_check: true,
            dump_range: true,
            trie_set: "ucd_trie::TrieSet".to_string(),
        }
    }
}

impl DumpAction {
    pub fn dump(&self, set: &CharacterSet) -> Result<String, std::fmt::Error> {
        let mut out = String::new();
        if self.dump_range {
            self.write_range(set, &mut out)?;
            return Ok(out);
        }
        if self.dump_tree {
            let tree = set.compress();
            self.write_tree(tree.as_slice(), &mut out)?;
            if self.dump_check {
                out.push_str("\n\n");
                self.write_check(&mut out)?
            }
            return Ok(out);
        }
        return Ok(out);
    }

    fn write_range(&self, set: &CharacterSet, w: &mut impl Write) -> Result<(), std::fmt::Error> {
        let ranges = set.to_ranges();
        self.write_public(w)?;
        writeln!(w, "const {name}: &'static [RangeInclusive<char>] = &[", name = self.name)?;
        for range in ranges {
            writeln!(w, "    {:?}..={:?},", range.start(), range.end())?;
        }
        write!(w, "];")?;
        Ok(())
    }

    fn write_tree(&self, tree: TrieSetSlice, w: &mut impl Write) -> Result<(), std::fmt::Error> {
        self.write_skip_fmt(w)?;
        self.write_public(w)?;
        writeln!(w, "const {name}: {trie_set} = {trie_set} {{", name = self.name, trie_set = self.trie_set)?;
        writeln!(w, "    tree1_level1: &{:?},", tree.tree1_level1)?;
        writeln!(w, "    tree2_level1: &{:?},", tree.tree2_level1)?;
        writeln!(w, "    tree2_level2: &{:?},", tree.tree2_level2)?;
        writeln!(w, "    tree3_level1: &{:?},", tree.tree3_level1)?;
        writeln!(w, "    tree3_level2: &{:?},", tree.tree3_level2)?;
        writeln!(w, "    tree3_level3: &{:?},", tree.tree3_level3)?;
        write!(w, "}};")?;
        Ok(())
    }
    fn write_check(&self, w: &mut impl Write) -> Result<(), std::fmt::Error> {
        self.write_public(w)?;
        writeln!(w, "fn is_{name}(c: char) -> bool {{", name = self.name.to_ascii_lowercase())?;
        writeln!(w, "    {name}.contains_char(c)", name = self.name)?;
        write!(w, "}};")?;
        Ok(())
    }
    fn write_skip_fmt(&self, writer: &mut impl Write) -> Result<(), std::fmt::Error> {
        if self.skip_fmt {
            writer.write_str("#[rustfmt::skip]")?;
            writer.write_char('\n')?;
        }
        Ok(())
    }
    fn write_public(&self, writer: &mut impl Write) -> Result<(), std::fmt::Error> {
        if !self.public.is_empty() {
            writer.write_str(&self.public)?;
            writer.write_char(' ')?;
        }
        Ok(())
    }
}
