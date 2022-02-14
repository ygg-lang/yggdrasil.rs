// pub const XID_START: TrieSet = TrieSet::from(&[('0', '9'), ('A', 'Z'), ('a', 'z')]);

#[rustfmt::skip]
pub const TXT: ucd_trie::TrieSet = ucd_trie::TrieSet {
    tree1_level1: &[287948901175001088, 576460743847706622, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    tree2_level1: &[],
    tree2_level2: &[],
    tree3_level1: &[],
    tree3_level2: &[],
    tree3_level3: &[],
};

pub fn is_txt(c: char) -> bool {
    TXT.contains_char(c)
}
