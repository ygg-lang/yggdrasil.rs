use super::*;

impl GrammarState {
    pub fn report_meta(&self) -> HintItems {
        let mut document_symbol = vec![];
        document_symbol.push(self.top_area());
        for rule in self.rules() {
            document_symbol.push(rule.symbol_item())
        }
        document_symbol.push(test_node());
        HintItems { diagnostic: vec![], code_lens: vec![], document_symbol }
    }
    fn top_area(&self) -> DocumentSymbol {
        let name = match self.is_grammar {
            true => "Grammar",
            false => "Fragment",
        };
        let range = self.name.range;
        let mut terms = vec![];
        if self.is_grammar {
            terms.push(self.extension())
        }
        terms.push(self.ignores());
        DocumentSymbol {
            name: name.to_string(),
            detail: Some(self.name.data.to_owned()),
            kind: SymbolKind::Namespace,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: Some(terms),
        }
    }
    fn extension(&self) -> DocumentSymbol {
        let detail;
        let children;
        let range = self.name.range;
        let n = self.extensions.len();
        match n {
            0 => {
                detail = 0.to_string();
                children = None;
            }
            _ => {
                detail = n.to_string();
                children = Some(
                    self.extensions
                        .iter()
                        .enumerate()
                        .map(|(i, r)| self.extension_item(i, r.data.to_owned(), r.range))
                        .collect(),
                );
            }
        };
        DocumentSymbol {
            name: "extensions".to_owned(),
            detail: Some(detail),
            kind: SymbolKind::Array,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children,
        }
    }
    fn extension_item(&self, index: usize, ext: String, range: Range) -> DocumentSymbol {
        DocumentSymbol {
            name: ext.to_owned(),
            detail: Some((index + 1).to_string()),
            kind: SymbolKind::Key,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        }
    }
    fn ignores(&self) -> DocumentSymbol {
        let n = self.ignores.len();
        let children = match n {
            0 => None,
            _ => {
                let mut out = vec![];
                for r in &self.ignores {
                    if let Some(s) = self.get(&r.data) {
                        let mut s = s.symbol_item();
                        s.range = r.range;
                        out.push(s)
                    }
                }
                Some(out)
            }
        };
        DocumentSymbol {
            name: "Ignores".to_string(),
            detail: Some(n.to_string()),
            kind: SymbolKind::Number,
            tags: None,
            deprecated: None,
            range: Default::default(),
            selection_range: Default::default(),
            children,
        }
    }
}

impl YGGRule {
    fn symbol_item(&self) -> DocumentSymbol {
        let (detail, kind) = self.symbol_detail();
        DocumentSymbol {
            name: self.name.to_owned(),
            detail,
            kind,
            tags: None,
            deprecated: None,
            range: Default::default(),
            selection_range: Default::default(),
            children: None,
        }
    }
    fn symbol_detail(&self) -> (Option<String>, SymbolKind) {
        if self.force_inline { (Some("inlined".to_string()), SymbolKind::Variable) } else { (None, SymbolKind::Field) }
    }
}

fn test_node() -> DocumentSymbol {
    DocumentSymbol {
        name: "Testing".to_string(),
        detail: Some("Symbol".to_string()),
        kind: SymbolKind::File,
        tags: None,
        deprecated: None,
        range: Default::default(),
        selection_range: Default::default(),
        children: unsafe { Some(test_all_symbol()) },
    }
}

unsafe fn test_all_symbol() -> Vec<DocumentSymbol> {
    (1u8..=26)
        .map(|n| transmute::<u8, SymbolKind>(n))
        .map(|k| DocumentSymbol {
            name: format!("{:?}", k),
            detail: Some(format!("{:?}", k)),
            kind: k,
            tags: None,
            deprecated: None,
            range: Default::default(),
            selection_range: Default::default(),
            children: None,
        })
        .collect()
}
