use super::*;

impl<'i> Extractor<Tuple_literalContextAll<'i>> for TupleNode {
    fn take_one(node: &Tuple_literalContextAll<'i>) -> Option<Self> {
        let mut terms = vec![];
        for pair in node.collection_pair_all() {
            match TupleTermNode::take_one(&pair) {
                Some(s) => terms.push(s),
                None => tracing::warn!(""),
            }
        }
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { kind: TupleKind::Tuple, terms, span })
    }
}
impl<'i> Extractor<Range_literalContextAll<'i>> for ArrayNode {
    fn take_one(node: &Range_literalContextAll<'i>) -> Option<Self> {
        let mut terms = vec![];
        for pair in node.range_axis_all() {
            match ArrayTermNode::take_one(&pair) {
                Some(s) => terms.push(s),
                None => tracing::warn!(""),
            }
        }
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { kind: ArrayKind::Ordinal, terms, span })
    }
}

impl<'i> Extractor<Collection_pairContextAll<'i>> for TupleTermNode {
    fn take_one(node: &Collection_pairContextAll<'i>) -> Option<Self> {
        let key = TupleKeyType::take(node.collection_key()).unwrap_or_default();
        let expr = ExpressionType::take(node.expression())?;
        Some(Self { key, value: expr })
    }
}
impl<'i> Extractor<Collection_keyContextAll<'i>> for TupleKeyType {
    fn take_one(node: &Collection_keyContextAll<'i>) -> Option<Self> {
        match node {
            Collection_keyContextAll::CK1Context(v) => Some(Self::Identifier(IdentifierNode::take(v.identifier())?)),
            Collection_keyContextAll::CK3Context(v) => {
                Some(Self::Identifier(StringTextNode::take(v.string())?.as_identifier()))
            }
            Collection_keyContextAll::CK2Context(v) => Some(Self::Number(BigUint::take(v.INTEGER())?)),
            Collection_keyContextAll::Error(_) => None,
        }
    }
}
impl<'i> Extractor<Range_axisContextAll<'i>> for ArrayTermNode {
    fn take_one(node: &Range_axisContextAll<'i>) -> Option<Self> {
        if let Some(s) = &node.index {
            return Some(Self::Index { index: ExpressionType::take_one(&**s)? });
        }
        let head = ExpressionType::take(node.head.clone());
        let tail = ExpressionType::take(node.tail.clone());
        let step = ExpressionType::take(node.step.clone());
        Some(Self::Range { head, tail, step })
    }
}
