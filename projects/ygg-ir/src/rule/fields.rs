use super::*;
use crate::{data::RuleReference, traits::FieldDescriptor};
use std::collections::HashSet;

pub type FieldMap = BTreeMap<String, YggdrasilField>;

#[derive(Debug)]
pub enum FieldCount {
    Optional,
    One,
    Many,
}

pub enum FieldKind {
    Named(YggdrasilIdentifier),
    IgnoreText,
    IgnoreRegex,
    IgnoreComment,
    IgnoreWhitespace,
}

/// ```ygg
/// name: Kind
/// ```
///
/// ```ignore
/// pub struct ANode {
///     a: T,
///     b: Option<T>,
///     c: Vec<T>,
///     span: Range<usize>
/// }
/// ```
pub struct YggdrasilField {
    pub name: YggdrasilIdentifier,
    pub kind: FieldKind,
    pub count: FieldCount,
}

pub struct YggdrasilClass {
    pub fields: BTreeMap<String, YggdrasilField>,
}

impl GrammarRule {
    pub fn get_class_fields(&self) -> YggdrasilClass {
        assert_eq!(self.kind, GrammarRuleKind::Class, "do you filter with class?");

        YggdrasilClass { fields: Default::default() }
    }
}

impl FieldDescriptor for YggdrasilExpression {
    fn visit_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        todo!()
    }

    fn visit_field_count(&self, buffer: &mut HashSet<String>) {
        todo!()
    }
}
//
// impl FieldDescriptor for ExpressionKind {
//     fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
//         match self {
//             ExpressionKind::Choice(e) => e.get_field_names(buffer),
//             ExpressionKind::Concat(e) => e.get_field_names(buffer),
//             ExpressionKind::Unary(e) => e.get_field_names(buffer),
//             ExpressionKind::Rule(e) => e.get_field_names(buffer),
//             ExpressionKind::Function(e) => e.get_field_names(buffer),
//             ExpressionKind::Regex(_) => {
//                 todo!()
//             }
//             _ => {}
//         }
//     }
//
//     fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount2>) {
//         match self {
//             ExpressionKind::Choice(e) => e.get_field_count(buffer),
//             ExpressionKind::Concat(e) => e.get_field_count(buffer),
//             ExpressionKind::Unary(e) => e.get_field_count(buffer),
//             ExpressionKind::Rule(e) => e.get_field_count(buffer),
//             ExpressionKind::Function(e) => e.get_field_count(buffer),
//             ExpressionKind::Regex(_) => {
//                 todo!()
//             }
//             _ => {}
//         }
//     }
// }
//
// impl FieldDescriptor for FunctionExpression {
//     fn get_field_names<'a>(&'a self, _buffer: &mut HashSet<&'a String>) {
//         todo!()
//     }
//
//     fn get_field_count(&self, _buffer: &mut HashSet<String, FieldCount2>) {
//         todo!()
//     }
// }
//
// impl FieldDescriptor for ChoiceExpression {
//     fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
//         self.branches.iter().for_each(|f| f.get_field_names(buffer))
//     }
//
//     fn get_field_count(&self, _buffer: &mut HashSet<String, FieldCount2>) {
//         todo!()
//     }
// }
//
// impl FieldDescriptor for ConcatExpression {
//     fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
//         todo!()
//     }
//
//     fn get_field_count(&self, _buffer: &mut HashSet<String, FieldCount2>) {
//         todo!()
//     }
// }
//
// impl FieldDescriptor for UnaryExpression {
//     fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
//         self.base.get_field_names(buffer)
//     }
//
//     fn get_field_count(&self, _buffer: &mut HashSet<String, FieldCount2>) {
//         todo!()
//     }
// }
//
// impl FieldDescriptor for RuleReference {
//     fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
//         buffer.insert(&self.name.text);
//     }
//
//     fn get_field_count(&self, _: &mut HashSet<String, FieldCount2>) {
//         unreachable!()
//     }
// }
