mod generated;

use character_set::{CharacterSet, DumpAction};
use generated::*;

#[test]
fn build() {
    let dump = DumpAction {
        name: "CASED_LETTER".to_string(),
        public: "pub".to_string(),
        skip_fmt: true,
        dump_tree: true,
        dump_check: true,
        trie_set: "".to_string(),
    };
    let mut set = CharacterSet::nil();
    for i in CASED_LETTER {
        set.include(*i).unwrap_or_default()
    }
    println!("{:#}", set)
}
