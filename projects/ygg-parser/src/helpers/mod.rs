use crate::bootstrap::{DecoratorCallNode, EscapedUnicodeNode, ModifierCallNode};

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