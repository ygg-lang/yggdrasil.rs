use character_set::CharacterSet;
use yggdrasil_ir::DataKind;

use crate::parser::ast::CharItem;

use super::*;

impl CharsetNode {
    pub(super) fn as_expr(&self, _: &mut GrammarParser) -> Result<ExpressionKind, YggdrasilError> {
        let mut set: CharacterSet;
        if self.neg.is_some() {
            set = CharacterSet::all();
            for item in &self.items {
                match item {
                    CharItem::CharOne(c) => set.exclude(c.string.chars().next().unwrap())?,
                    CharItem::CharRange(c) => set.exclude(c.start..=c.end)?,
                }
            }
            Ok(ExpressionKind::Data(Box::new(DataKind::CharacterFused(set))))
        }
        else {
            set = CharacterSet::nil();
            for item in &self.items {
                match item {
                    CharItem::CharOne(c) => set.include(c.string.chars().next().unwrap())?,
                    CharItem::CharRange(c) => set.include(c.start..=c.end)?,
                }
            }
            Ok(ExpressionKind::Data(Box::new(DataKind::CharacterFused(set))))
        }
    }
}

impl StringLiteral {
    pub(super) fn as_expr(&self, _: &mut GrammarParser) -> Result<ExpressionKind, YggdrasilError> {
        let mut s = String::new();
        for item in &self.body {
            match item {
                StringItem::CharOne(c) => s.push(c.string.chars().next().unwrap()),
                StringItem::StringEscaped(escaped) => match escaped.char {
                    'n' => s.push('\n'),
                    _ => s.push(escaped.char),
                },
            }
        }
        Ok(ExpressionKind::string(s))
    }
}
