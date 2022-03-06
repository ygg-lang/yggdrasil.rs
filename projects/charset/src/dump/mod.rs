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
    pub trie_set: String,
}

impl Default for DumpAction {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            public: "".to_string(),
            skip_fmt: true,
            dump_tree: true,
            dump_check: true,
            trie_set: "ucd_trie::TrieSet".to_string(),
        }
    }
}

impl DumpAction {
    pub fn dump(&self, set: &CharacterSet) -> String {
        let mut out = String::new();
        if self.dump_tree {
            let tree = set.compress();
            self.dump_tree(tree.as_slice(), &mut out).unwrap_or_default();
            if self.dump_check {}
        }
        return out;
    }

    // fn write_range(&self) -> Result<String, std::fmt::Error> {
    //     let ranges = self.to_ranges();
    //     let mut w = String::new();
    //     let public = if public { "pub " } else { "" };
    //     writeln!(w, "const {}: &'static [(char, char); {}] = &[", public, name, ranges.len())?;
    //     for range in ranges {
    //         writeln!(w, "    ({:?}, {:?}),", range.start, range.end)?;
    //     }
    //     write!(w, "];")?;
    //     Ok(w)
    // }
    fn dump_tree(&self, tree: TrieSetSlice, w: &mut impl Write) -> Result<(), std::fmt::Error> {
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
