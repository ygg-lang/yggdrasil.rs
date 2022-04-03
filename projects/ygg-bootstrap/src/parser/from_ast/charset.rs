use character_set::CharacterSet;

use yggdrasil_ir::DataKind;

use crate::parser::ast::CharItem;

use super::*;

impl CharsetNode {
    pub(super) fn as_expr(&self, _: &mut GrammarParser) -> Result<ExpressionKind, YggdrasilError> {
        let mut set = CharacterSet::nil();
        for item in &self.items {
            match item {
                CharItem::CharOne(c) => set.include(*c)?,
                CharItem::CharRange(c) => set.include(c.start..=c.end)?,
            }
        }
        Ok(ExpressionKind::Data(Box::new(DataKind::CharacterSet(set))))
    }
}

impl StringLiteral {
    pub(super) fn as_expr(&self, _: &mut GrammarParser) -> Result<ExpressionKind, YggdrasilError> {
        let mut s = String::new();
        for item in &self.body {
            match item {
                StringItem::CharOne(c) => s.push(*c),
                StringItem::StringEscaped(escaped) => match escaped.char {
                    'n' => s.push('\n'),
                    _ => s.push(escaped.char),
                },
            }
        }
        Ok(ExpressionKind::string(s))
    }
}
