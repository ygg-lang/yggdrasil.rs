use crate::bootstrap::{DecoratorCallNode, ModifierCallNode};

mod annotations;
mod expressions;

pub struct TakeAnnotations<'i> {
    auto_tag: bool,
    macros: &'i [DecoratorCallNode],
    modifiers: &'i [ModifierCallNode],
}
