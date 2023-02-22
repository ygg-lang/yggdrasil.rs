use regex_automata::dfa::{dense::DFA, regex::Regex};

/// A precompiled regular expression state machine
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
