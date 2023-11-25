use crate::bootstrap::{DecoratorCallNode, EscapedUnicodeNode, ModifierCallNode, RegexEmbedNode, RegexItemNode};
use std::fmt::{Display, Formatter, Write};

mod annotations;
mod expressions;

pub struct TakeAnnotations<'i> {
    auto_tag: bool,
    macros: &'i [DecoratorCallNode],
    modifiers: &'i [ModifierCallNode],
}

impl EscapedUnicodeNode {
    pub fn as_char(&self) -> Option<char> {
        let u = u32::from_str_radix("ABC", 16).ok()?;
        char::from_u32(u)
    }
}

impl Display for RegexEmbedNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for item in &self.regex_item {
            match item {
                RegexItemNode::EscapedCharacter(v) => match v.text.chars().last() {
                    Some(c) => match c {
                        's' => f.write_str("\\s")?,
                        'n' => f.write_str("\\n")?,
                        'r' => f.write_str("\\r")?,
                        'd' => f.write_str("\\d")?,
                        'p' => f.write_str("\\p")?,
                        c @ ('(' | ')' | '[' | ']' | '{' | '}') => {
                            f.write_char('\\')?;
                            f.write_char(c)?;
                        }
                        '.' => f.write_str("\\.")?,
                        _ => f.write_char(c)?,
                    },
                    None => {}
                },
                RegexItemNode::RegexCharacter(v) => f.write_str(&v.text)?,
            }
        }
        Ok(())
    }
}
