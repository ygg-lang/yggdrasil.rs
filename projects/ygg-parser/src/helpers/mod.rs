use crate::bootstrap::{
    DecoratorCallNode, EscapedUnicodeNode, ModifierCallNode, RegexEmbedNode, RegexItemNode, UnionBranchNode,
};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Write},
    ops::Range,
};
use yggdrasil_rt::YggdrasilNode;

mod annotations;
mod expressions;

impl<'i> UnionBranchNode<'i> {
    pub fn is_single(&self) -> bool {
        self.expression_hard().to_identifier().is_some()
    }
    pub fn branch_name(&self, name: &str, index: usize) -> (Cow<str>, Range<usize>) {
        match self.try_branch_name() {
            Some(s) => s,
            None => match self.try_expression_name() {
                Some(s) => s,
                None => (Cow::Owned(format!("{}{}", name, index)), self.expression_hard().get_range()),
            },
        }
    }

    fn try_branch_name(&self) -> Option<(Cow<str>, Range<usize>)> {
        let branch = self.branch_tag()?;
        Some((Cow::Borrowed(branch.identifier().get_str()), branch.get_range()))
    }
    fn try_expression_name(&self) -> Option<(Cow<str>, Range<usize>)> {
        let atom = self.expression_hard().to_atom()?;
        let id = atom.to_identifier()?;
        Some((Cow::Borrowed(id.get_str()), id.get_range()))
    }
}

pub struct TakeAnnotations<'i> {
    auto_tag: bool,
    macros: Vec<DecoratorCallNode<'i>>,
    modifiers: Vec<ModifierCallNode<'i>>,
}

impl<'i> EscapedUnicodeNode<'i> {
    pub fn as_char(&self) -> Option<char> {
        let u = u32::from_str_radix("ABC", 16).ok()?;
        char::from_u32(u)
    }
}

impl<'i> Display for RegexEmbedNode<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for item in &self.regex_item() {
            match item {
                RegexItemNode::EscapedCharacter(v) => match v.get_chars().last() {
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
                RegexItemNode::RegexCharacter(v) => f.write_str(v.get_str())?,
            }
        }
        Ok(())
    }
}
