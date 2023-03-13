use super::*;
use crate::{data::RuleReference, traits::FieldDescriptor};
use std::{collections::HashSet, ops::BitXorAssign};

pub mod counter;

pub enum FieldKind {
    Rule(String),
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
    pub name: String,
    pub kind: FieldKind,
    pub count: FieldCounter,
    pub bind_position: Vec<Range<usize>>,
    pub rule_position: Vec<Range<usize>>,
}

#[derive(Default)]
struct FieldMap {
    wrap: BTreeMap<String, YggdrasilField>,
}

impl FieldMap {
    pub fn register_rule(&mut self, bind: &YggdrasilIdentifier, rule: &RuleReference, count: FieldCounter) {
        match self.wrap.get_mut(&bind.text) {
            Some(old) => {
                old.bind_position.push(bind.range.clone());
                old.count += count;
            }
            None => {
                let name = bind.text.to_string();
                let new = YggdrasilField {
                    name: name.clone(),
                    kind: FieldKind::Rule(rule.name.text.clone()),
                    count,
                    bind_position: vec![bind.range.clone()],
                    rule_position: vec![rule.name.range.clone()],
                };
                self.wrap.insert(name, new);
            }
        }
    }
}

impl GrammarRule {
    pub fn as_class(&self) -> YggdrasilVariant {
        assert_eq!(self.kind, GrammarRuleKind::Class, "do you filter with class?");
        let mut fields = Default::default();
        match &self.body {
            Some(s) => s.visit_class(None, &mut fields),
            None => {}
        }

        YggdrasilVariant { document: self.document.clone(), name: self.name.clone(), fields }
    }
}

impl YggdrasilExpression {
    fn visit_class(&self, candidate: Option<&YggdrasilIdentifier>, map: &mut FieldMap) {
        let tag = self.tag.as_ref().or(candidate);
        match &self.kind {
            // `a:IGNORED`
            ExpressionKind::Ignored => {}
            ExpressionKind::Function(_) => {}
            ExpressionKind::Choice(many) => {
                // a:(x | y), drop tag
                for one in &many.branches {
                    one.visit_class(None, map);
                }
            }
            ExpressionKind::Concat(many) => {
                // a:(x ~ y), drop tag
                for one in &many.sequence {
                    one.visit_class(None, map);
                }
            }
            // a:x+
            // a:(x*)
            // a:(b:x)
            ExpressionKind::Unary(one) => one.base.visit_class(tag, map),
            ExpressionKind::Rule(one) => match &self.tag {
                Some(s) => YggdrasilField { name: s.clone(), kind: FieldKind::Rule(one.name.clone()), count: FieldCounter::OPTIONAL },
                None => {}
            },
            ExpressionKind::Text(_) => {}
            ExpressionKind::CharacterAny => {}
            ExpressionKind::CharacterRange(_) => {}
            ExpressionKind::Integer(_) => {}
            ExpressionKind::Boolean(_) => {}
            ExpressionKind::Regex(_) => {}
            _ => {}
        }
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
