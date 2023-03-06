use regex_automata::dfa::{dense::DFA, regex::Regex};

pub mod stack;

/// A precompiled regular expression state machine
pub struct RegexCompiled {
    /// FWD
    pub forward_le: &'static [u8],
    /// FWD
    pub reverse_le: &'static [u8],
    /// FWD
    pub forward_be: &'static [u8],
    /// FWD
    pub reverse_be: &'static [u8],
}

impl RegexCompiled {
    /// FWD
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
