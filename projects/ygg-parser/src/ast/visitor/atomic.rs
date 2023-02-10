use super::*;

impl<'i> Extractor<AtomicContextAll<'i>> for ExpressionType {
    fn take_one(node: &AtomicContextAll<'i>) -> Option<Self> {
        let body: ExpressionType = match node {
            AtomicContextAll::ASpecialContext(s) => {
                let this = s.SPECIAL()?;
                let text = this.get_text();
                let span = Range { start: this.symbol.start as u32, end: this.symbol.stop as u32 };
                match text.as_str() {
                    "true" => BooleanNode { value: true, span }.into(),
                    "false" => BooleanNode { value: false, span }.into(),
                    "null" => NullNode { nil: false, span }.into(),
                    "nil" | "∅" => NullNode { nil: true, span }.into(),
                    "∞" => StringTextNode::new("∞", span).into(),
                    _ => unreachable!("Atom: {}", text),
                }
            }
            AtomicContextAll::ALambdaContext(s) => {
                todo!()
            }
            AtomicContextAll::ANumberContext(s) => NumberLiteralNode::take(s.number_literal())?.into(),
            AtomicContextAll::AStringContext(s) => StringLiteralNode::take(s.string_literal())?.into(),
            AtomicContextAll::ANamepathContext(s) => NamePathNode::take(s.namepath())?.into(),
            AtomicContextAll::AOutputContext(s) => OutputNode::take(s.output_name())?.into(),
            AtomicContextAll::Error(_) => {
                todo!()
            }
        };
        Some(body)
    }
}

impl<'i> Extractor<TerminalNode<'i, ValkyrieAntlrParserContextType>> for isize {
    fn take_one(node: &TerminalNode<'i, ValkyrieAntlrParserContextType>) -> Option<Self> {
        isize::from_str(&node.get_text()).ok()
    }
}
impl<'i> Extractor<TerminalNode<'i, ValkyrieAntlrParserContextType>> for BigUint {
    fn take_one(node: &TerminalNode<'i, ValkyrieAntlrParserContextType>) -> Option<Self> {
        BigUint::from_str(&node.get_text()).ok()
    }
}
impl<'i> Extractor<Output_nameContextAll<'i>> for OutputNode {
    fn take_one(node: &Output_nameContextAll<'i>) -> Option<Self> {
        let index = match node {
            Output_nameContextAll::PositiveOutputContext(v) => isize::take(v.INTEGER())?,
            Output_nameContextAll::NegativeOutputContext(v) => -isize::take(v.INTEGER())?,
            Output_nameContextAll::Error(_) => 0,
        };
        let span = Range { start: node.start().get_start() as u32, end: node.stop().get_stop() as u32 };
        Some(Self { count: index, span })
    }
}

impl<'i> Extractor<Number_literalContextAll<'i>> for NumberLiteralNode {
    fn take_one(node: &Number_literalContextAll<'i>) -> Option<Self> {
        let value = node.number()?.get_text();
        let suffix = IdentifierNode::take(node.identifier());
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        Some(Self { value, unit: suffix, span })
    }
}
impl<'i> Extractor<String_literalContextAll<'i>> for StringLiteralNode {
    fn take_one(node: &String_literalContextAll<'i>) -> Option<Self> {
        let handler = IdentifierNode::take(node.identifier());
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        let raw = StringTextNode::take(node.string())?;
        Some(Self { literal: raw.text, handler, span })
    }
}

impl<'i> Extractor<StringContext<'i>> for StringTextNode {
    fn take_one(node: &StringContext<'i>) -> Option<Self> {
        let mut out = String::with_capacity(node.get_text().len());
        let span = Range { start: node.start().start as u32, end: node.stop().stop as u32 };
        // for item in node.STRING_TEXT_all() {
        //     out.push_str(&item.get_text());
        // }
        let all = node.get_children().count();
        for index in 0..all {
            match node.get_token(STRING_TEXT, index) {
                Some(item) => out.push_str(&item.get_text()),
                None => {}
            }
        }
        Some(Self { text: out, span })
    }
}

impl<'i> Extractor<IdentifierContextAll<'i>> for IdentifierNode {
    fn take_one(node: &IdentifierContextAll<'i>) -> Option<Self> {
        if let Some(s) = node.UNICODE_ID() {
            return Some(IdentifierNode {
                name: s.get_text(),
                span: Range { start: s.symbol.start as u32, end: s.symbol.stop as u32 },
            });
        }
        if let Some(s) = node.RAW_ID() {
            return Some(IdentifierNode {
                name: s.symbol.text.trim_matches('`').to_string(),
                span: Range { start: s.symbol.start as u32, end: s.symbol.stop as u32 },
            });
        }
        None
    }
}
