use super::*;
use yggdrasil_ir::{
    data::{RegularExpression, YggdrasilText},
    rule::YggdrasilNamepath,
};

impl<'i> Extractor<AtomicContextAll<'i>> for ExpressionNode {
    fn take_one(node: &AtomicContextAll<'i>) -> Option<Self> {
        match node {
            AtomicContextAll::AIntContext(_) => todo!(),
            AtomicContextAll::AReContext(r) => Some(RegularExpression::take(r.regex())?.into()),
            AtomicContextAll::ACharContext(escape) => {
                let range = Range { start: escape.start().start as usize, end: escape.stop().stop as usize };
                let char = escape.get_text().chars().last()?;
                Some(
                    YggdrasilText {
                        text: match char {
                            'r' => "\r".to_string(),
                            'n' => "\n".to_string(),
                            _ => char.to_string(),
                        },
                        range,
                    }
                    .into(),
                )
            }
            AtomicContextAll::ATupleContext(_) => todo!(),
            AtomicContextAll::ASpecialContext(_) => todo!(),
            AtomicContextAll::AIdContext(s) => Some(YggdrasilIdentifier::take(s.identifier())?.into()),
            AtomicContextAll::AStringContext(s) => Some(YggdrasilText::take(s.string())?.into()),
            AtomicContextAll::Error(_) => None,
        }
    }
}

impl<'i> Extractor<NamepathContextAll<'i>> for YggdrasilNamepath {
    fn take_one(node: &NamepathContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        Some(Self { identifiers: YggdrasilIdentifier::take_many(&node.identifier_all()), range: span })
    }
}
impl<'i> Extractor<IdentifierContextAll<'i>> for YggdrasilIdentifier {
    fn take_one(node: &IdentifierContextAll<'i>) -> Option<Self> {
        if let Some(s) = node.UNICODE_ID() {
            return Some(YggdrasilIdentifier {
                text: s.get_text(),
                span: Range { start: s.symbol.start as usize, end: s.symbol.stop as usize },
            });
        }
        if let Some(s) = node.RAW_ID() {
            return Some(YggdrasilIdentifier {
                text: s.symbol.text.trim_matches('`').to_string(),
                span: Range { start: s.symbol.start as usize, end: s.symbol.stop as usize },
            });
        }
        None
    }
}
impl<'i> Extractor<RegexContextAll<'i>> for RegularExpression {
    fn take_one(node: &RegexContextAll<'i>) -> Option<Self> {
        None
    }
}

impl<'i> Extractor<StringContextAll<'i>> for YggdrasilText {
    fn take_one(node: &StringContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        let mut buffer = String::new();
        if let Some(s) = node.STRING_SINGLE() {
            buffer = s.get_text().trim_matches('\'').to_string()
        }
        if let Some(s) = node.STRING_DOUBLE() {
            buffer = s.get_text().trim_matches('"').to_string()
        }
        Some(Self { text: buffer, range: span })
    }
}
