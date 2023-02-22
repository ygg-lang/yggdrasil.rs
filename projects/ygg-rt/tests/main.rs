mod json5;

use dense::DFA;
use regex_automata::{
    dfa::{dense, dense::BuildError, regex::Regex},
    Match,
};
use std::{
    hash::{Hash, Hasher},
    ops::Range,
    str::FromStr,
};

pub struct RegexCompiled {
    pub forward_le: &'static [u8],
    pub reverse_le: &'static [u8],
    pub forward_be: &'static [u8],
    pub reverse_be: &'static [u8],
}

impl RegexCompiled {
    pub fn regex(&self) -> Regex<DFA<&[u32]>> {
        unsafe {
            if cfg!(target_endian = "little") {
                let fwd: DFA<&[u32]> = DFA::from_bytes_unchecked(self.forward_le).unwrap().0;
                let rev: DFA<&[u32]> = DFA::from_bytes_unchecked(self.reverse_le).unwrap().0;
                Regex::builder().build_from_dfas(fwd, rev)
            }
            else {
                let fwd: DFA<&[u32]> = DFA::from_bytes_unchecked(self.forward_be).unwrap().0;
                let rev: DFA<&[u32]> = DFA::from_bytes_unchecked(self.forward_be).unwrap().0;
                Regex::builder().build_from_dfas(fwd, rev)
            }
        }
    }
}

pub struct YggdrasilRegex {
    raw: String,
    span: Range<usize>,
    forward_le: Vec<u8>,
    reverse_le: Vec<u8>,
    forward_be: Vec<u8>,
    reverse_be: Vec<u8>,
}

impl FromStr for YggdrasilRegex {
    type Err = BuildError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(s)?;
        Ok(Self { raw: s.to_string(), ..Self::from(re) })
    }
}

impl YggdrasilRegex {
    pub fn new<S>(text: String, span: Range<usize>) -> Self
    where
        S: ToString,
    {
        Self { raw: text.to_string(), span, forward_le: vec![], reverse_le: vec![], forward_be: vec![], reverse_be: vec![] }
    }
    pub fn build(&mut self) -> Result<(), BuildError> {
        let regex = Regex::new(&self.raw)?;
        let (fwd_bytes, fwd_pad) = regex.forward().to_bytes_little_endian();
        let (rev_bytes, rev_pad) = regex.reverse().to_bytes_little_endian();
        self.forward_le = fwd_bytes[fwd_pad..].to_vec();
        self.reverse_le = rev_bytes[rev_pad..].to_vec();
        let (fwd_bytes, fwd_pad) = regex.forward().to_bytes_big_endian();
        let (rev_bytes, rev_pad) = regex.reverse().to_bytes_big_endian();
        self.forward_be = fwd_bytes[fwd_pad..].to_vec();
        self.reverse_be = rev_bytes[rev_pad..].to_vec();
        Ok(())
    }
}

impl Hash for YggdrasilRegex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.forward_le.hash(state);
        self.reverse_le.hash(state);
        self.forward_be.hash(state);
        self.reverse_be.hash(state);
    }
}

#[test]
fn test() {
    let re1 = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    // serialize both the forward and reverse DFAs, see note below
    let (fwd_bytes, fwd_pad) = re1.forward().to_bytes_little_endian();
    let (rev_bytes, rev_pad) = re1.reverse().to_bytes_little_endian();
    let DFA_UAC_FWD = &fwd_bytes[fwd_pad..];
    let DFA_ASDF_REV = &rev_bytes[rev_pad..];

    let fwd: DFA<&[u32]> = DFA::from_bytes(DFA_UAC_FWD).unwrap().0;
    let rev: DFA<&[u32]> = DFA::from_bytes(DFA_ASDF_REV).unwrap().0;
    // finally, reconstruct our regex
    let re2 = Regex::builder().build_from_dfas(fwd, rev);

    // we can use it like normal
    let text = b"2018-12-24 2016-10-08";
    let matches: Vec<Match> = re2.find_iter(text).collect();
    assert_eq!(matches, vec![Match::must(0, 0..10), Match::must(0, 11..21)]);
}
