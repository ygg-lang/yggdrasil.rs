#[allow(dead_code, unused_variables)]
mod generated;

use character_set::{CharacterSet, DumpAction};
use generated::*;
use std::{fs::OpenOptions, io::Write};

#[test]
fn build_generated() -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open("src/builtin/generated.rs")?;
    writeln!(file, "use std::ops::RangeInclusive;\n",)?;
    writeln!(file, "{}\n", build_item_range("LETTER", LETTER))?;
    writeln!(file, "{}\n", build_item_range("LOWERCASE_LETTER", LOWERCASE_LETTER))?;
    writeln!(file, "{}\n", build_item_range("MODIFIER_LETTER", MODIFIER_LETTER))?;
    writeln!(file, "{}\n", build_item_range("OTHER_LETTER", OTHER_LETTER))?;
    writeln!(file, "{}\n", build_item_range("TITLECASE_LETTER", TITLECASE_LETTER))?;
    writeln!(file, "{}\n", build_item_range("UPPERCASE_LETTER", UPPERCASE_LETTER))?;
    writeln!(file, "{}\n", build_item_range("CASED_LETTER", CASED_LETTER))?;
    Ok(())
}

fn build_item_range(name: &str, item: &[(u32, u32)]) -> String {
    let dump = DumpAction {
        name: name.to_string(),
        public: "pub".to_string(),
        skip_fmt: true,
        dump_tree: true,
        dump_check: true,
        dump_range: true,
        trie_set: "".to_string(),
    };
    let mut set = CharacterSet::nil();
    for i in item {
        set.include(*i).unwrap_or_default()
    }
    return dump.dump(&set).unwrap_or_default();
}
