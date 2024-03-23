use crate::bootstrap::{
    BranchTagNode, DecoratorCallNode, EscapedUnicodeNode, IdentifierNode, ModifierCallNode, RegexEmbedNode, RegexItemNode,
    UnionBranchNode,
};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Write},
    ops::Range,
};

mod annotations;
mod expressions;

impl UnionBranchNode {
    pub fn is_single(&self) -> bool {
        self.expression_hard.as_identifier().is_some()
    }
    pub fn branch_name(&self, name: &str, index: usize) -> (Cow<str>, Range<usize>) {
        match self.try_branch_name() {
            Some(s) => s,
            None => match self.try_expression_name() {
                Some(s) => s,
                None => (Cow::Owned(format!("{}{}", name, index)), self.expression_hard.span.clone()),
            },
        }
    }

    fn try_branch_name(&self) -> Option<(Cow<str>, Range<usize>)> {
        let branch = self.branch_tag.as_ref()?;
        Some((Cow::Borrowed(&branch.identifier.text), branch.span.clone()))
    }
    fn try_expression_name(&self) -> Option<(Cow<str>, Range<usize>)> {
        let id = self.expression_hard.as_atom()?.as_identifier()?;
        Some((Cow::Borrowed(&id.text), id.span.clone()))
    }
}

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
