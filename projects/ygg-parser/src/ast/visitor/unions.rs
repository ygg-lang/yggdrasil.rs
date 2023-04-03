use super::*;

impl<'i> Extractor<Define_unionContext<'i>> for GrammarRule {
    fn take_one(node: &Define_unionContext<'i>) -> Option<Self> {
        let id = YggdrasilIdentifier::take(node.name.clone())?;
        let range = Range { start: node.start().start as usize, end: node.stop().stop as usize };
        let modifiers = YggdrasilModifiers::take(node.modifiers()).unwrap_or_default();
        let anno = YggdrasilAnnotations { macros: vec![], modifiers };
        let body = match node.union_block() {
            Some(s) => YggdrasilExpression::take_many(&s.union_term_all()),
            None => vec![],
        };

        Some(GrammarRule::create_union(id, body, range).with_annotation(&anno))
    }
}

impl<'i> Extractor<Union_termContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Union_termContextAll<'i>) -> Option<Self> {
        let terms = YggdrasilExpression::take_many(&node.union_expression_all());
        let mut expr = YggdrasilExpression::from(ConcatExpression::new(terms)?);
        expr.tag = YggdrasilIdentifier::take(node.tag_branch().and_then(|v| v.identifier()));
        Some(expr)
    }
}

impl<'i> Extractor<Union_expressionContextAll<'i>> for YggdrasilExpression {
    fn take_one(node: &Union_expressionContextAll<'i>) -> Option<Self> {
        match node {
            Union_expressionContextAll::UHardContext(u) => {
                let lhs = Self::take(u.lhs.clone())?;
                let rhs = Self::take(u.rhs.clone())?;
                Some(lhs + rhs)
            }
            Union_expressionContextAll::USoftContext(u) => {
                let lhs = Self::take(u.lhs.clone())?;
                let rhs = Self::take(u.rhs.clone())?;
                Some(lhs & rhs)
            }
            Union_expressionContextAll::UUntagContext(u) => {
                let base = Self::take(u.union_expression())?;
                Some(base.with_remark())
            }
            Union_expressionContextAll::USuffixContext(u) => {
                let base = Self::take(u.union_expression())?;
                let suffix = YggdrasilOperator::take(u.suffix())?;
                Some(Self::unary(base, suffix))
            }
            Union_expressionContextAll::UETagContext(u) => {
                let base = Self::take(u.union_expression())?;
                let id = YggdrasilIdentifier::take(u.identifier())?;
                Some(base.with_tag(id))
            }
            Union_expressionContextAll::UtomContext(v) => Self::take(v.atomic()),
            Union_expressionContextAll::UNotContext(u) => {
                let base = Self::take(u.union_expression())?;
                Some(Self::unary(base, YggdrasilOperator::Negative))
            }
            Union_expressionContextAll::Error(_) => None,
        }
    }
}
