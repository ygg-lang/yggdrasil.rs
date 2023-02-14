use super::*;

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
