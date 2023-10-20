use super::*;
use regex_automata::dfa::dense::BuildError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilRegex {
    pub raw: String,
    pub span: Range<usize>,
}
impl Display for YggdrasilRegex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\"^(")?;
        for char in self.raw.chars() {
            match char {
                '\\' => f.write_str("\\\\")?,
                '"' => f.write_str("\\\"")?,
                _ => f.write_char(char)?,
            }
        }
        f.write_str(")\"")
    }
}

impl From<YggdrasilRegex> for YggdrasilExpression {
    fn from(value: YggdrasilRegex) -> Self {
        ExpressionBody::Regex(value).into()
    }
}
impl YggdrasilRegex {
    pub fn new<S>(text: S, span: Range<usize>) -> Self
    where
        S: Display,
    {
        Self { raw: text.to_string(), span }
    }
    pub fn build(&mut self) -> Result<(), BuildError> {
        // let regex = Regex::new(&self.to_string())?;
        // let (fwd_bytes, fwd_pad) = regex.forward().to_bytes_little_endian();
        // let (rev_bytes, rev_pad) = regex.reverse().to_bytes_little_endian();
        // self.forward_le = fwd_bytes[fwd_pad..].to_vec();
        // self.reverse_le = rev_bytes[rev_pad..].to_vec();
        // let (fwd_bytes, fwd_pad) = regex.forward().to_bytes_big_endian();
        // let (rev_bytes, rev_pad) = regex.reverse().to_bytes_big_endian();
        // self.forward_be = fwd_bytes[fwd_pad..].to_vec();
        // self.reverse_be = rev_bytes[rev_pad..].to_vec();
        Ok(())
    }
    pub fn built(&self) -> Result<Self, BuildError> {
        let mut out = self.clone();
        out.build()?;
        Ok(out)
    }
}

// impl Display for YggdrasilRegex {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         writeln!(f, "/// `{}`", self.raw)?;
//         writeln!(f, "#[rustfmt::skip]")?;
//         writeln!(f, "const {}: RegexCompiled = RegexCompiled {{", self.constant_name())?;
//         writeln!(f, "    forward_le: &{:?},", self.forward_le)?;
//         writeln!(f, "    reverse_le: &{:?},", self.reverse_le)?;
//         writeln!(f, "    forward_be: &{:?},", self.forward_be)?;
//         writeln!(f, "    reverse_be: &{:?},", self.reverse_be)?;
//         f.write_str("};")?;
//         Ok(())
//     }
// }

impl YggdrasilRegex {
    pub fn constant_name(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let id = hasher.finish();
        format!("REGEX_{:X}", id)
    }
}
