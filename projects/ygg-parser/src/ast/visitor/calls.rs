use super::*;

impl<'i> Extractor<Function_callContextAll<'i>> for ApplyCallNode {
    fn take_one(node: &Function_callContextAll<'i>) -> Option<Self> {
        let monadic = node.OP_AND_THEN().is_some();
        let terms = ApplyCallTerms::take(node.tuple_call_body());
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { base: Default::default(), monadic, caller: Default::default(), arguments: terms, span })
    }
}
impl<'i> Extractor<Closure_callContextAll<'i>> for ClosureCallNode {
    fn take_one(node: &Closure_callContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        match node {
            Closure_callContextAll::NormalClosureContext(o) => {
                let monadic = o.OP_AND_THEN().is_some();
                let arguments = ApplyCallTerms::take(o.tuple_call_body());
                let trailing = FunctionBlock::take(o.function_block());
                Some(Self { base: Default::default(), monadic, arguments, trailing, caller: ClosureCaller::Normal, span })
            }
            Closure_callContextAll::SlotClosureContext(o) => {
                let monadic = o.OP_AND_THEN().is_some();
                let arguments = None;
                let trailing = FunctionBlock::take(o.function_block());
                Some(Self { base: Default::default(), monadic, arguments, trailing, caller: ClosureCaller::Lambda, span })
            }
            Closure_callContextAll::IntegerClosureContext(o) => {
                let monadic = o.OP_AND_THEN().is_some();
                let arguments = ApplyCallTerms::take(o.tuple_call_body());
                let trailing = FunctionBlock::take(o.function_block());
                let int = BigUint::take(o.INTEGER())?;
                Some(Self { base: Default::default(), monadic, arguments, trailing, caller: ClosureCaller::Integer(int), span })
            }
            Closure_callContextAll::InternalClosureContext(o) => {
                let monadic = o.OP_AND_THEN().is_some();
                let arguments = ApplyCallTerms::take(o.tuple_call_body());
                let trailing = FunctionBlock::take(o.function_block());
                let id = IdentifierNode::take(o.identifier())?;
                Some(Self { base: Default::default(), monadic, arguments, trailing, caller: ClosureCaller::Internal(id), span })
            }
            Closure_callContextAll::Error(_) => None,
        }
    }
}

impl<'i> Extractor<Slice_callContextAll<'i>> for SubscriptCallNode {
    fn take_one(node: &Slice_callContextAll<'i>) -> Option<Self> {
        let monadic = node.OP_AND_THEN().is_some();
        let array = ArrayNode::take(node.range_literal())?;
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { kind: array.kind, base: Default::default(), monadic, terms: array.terms, span })
    }
}

impl<'i> Extractor<Tuple_call_bodyContextAll<'i>> for ApplyCallTerms {
    fn take_one(node: &Tuple_call_bodyContextAll<'i>) -> Option<Self> {
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { terms: ApplyCallItem::take_many(&node.tuple_call_item_all()), span })
    }
}

impl<'i> Extractor<Tuple_call_itemContextAll<'i>> for ApplyCallItem {
    fn take_one(node: &Tuple_call_itemContextAll<'i>) -> Option<Self> {
        let modifiers = ModifiersNode { terms: IdentifierNode::take_many(&node.mods) };
        let parameter = IdentifierNode::take(node.field.clone());
        let body = ExpressionType::take(node.expression())?;
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { modifiers, parameter, body, span })
    }
}
