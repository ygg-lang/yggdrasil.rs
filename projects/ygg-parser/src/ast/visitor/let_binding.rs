use super::*;
use antlr_rust::parser_rule_context::RuleContextExt;
use valkyrie_ast::IdentifierPattern;

impl<'i> Extractor<Let_bindingContextAll<'i>> for LetBindNode {
    fn take_one(node: &Let_bindingContextAll<'i>) -> Option<Self> {
        let pat = LetPattern::take(node.let_pattern())?;
        let typ = ExpressionType::take(node.type_hint().and_then(|v| v.type_expression()));
        let rhs = ExpressionNode::take(node.expression_root());
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { pattern: pat, type_hint: typ, body: rhs, span })
    }
}
impl<'i> Extractor<Let_patternContextAll<'i>> for LetPattern {
    fn take_one(node: &Let_patternContextAll<'i>) -> Option<Self> {
        match node.let_pattern_plain() {
            Some(s) => LetPattern::take_one(&*s),
            None => {
                unimplemented!()
            }
        }
    }
}

impl<'i> Extractor<Let_pattern_plainContextAll<'i>> for LetPattern {
    fn take_one(node: &Let_pattern_plainContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        let all = IdentifierPattern::take_many(&node.modified_identifier_all());
        if all.len() == 1 {
            unsafe { Some(all.first().unwrap_unchecked().clone().into()) }
        }
        else {
            Some(TuplePatternNode { bind: None, name: None, terms: all.into_iter().map(Into::into).collect(), span }.into())
        }
    }
}
