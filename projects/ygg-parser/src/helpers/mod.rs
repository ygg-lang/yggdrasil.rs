use crate::bootstrap::{AnnotationCallNode, ModifierCallNode};

mod annotations;

pub struct TakeAnnotations<'i> {
    auto_tag: bool,
    macros: &'i [AnnotationCallNode],
    modifiers: &'i [ModifierCallNode],
}
