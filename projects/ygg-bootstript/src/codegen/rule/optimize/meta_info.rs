use super::*;

impl GrammarState {
    pub fn report_meta(&self) -> HintItems {
        let document_symbol = vec![self.top_area(), test_node()];
        HintItems { diagnostic: vec![], code_lens: vec![], document_symbol }
    }
    fn top_area(&self) -> DocumentSymbol {
        let name = match self.is_grammar {
            true => "Grammar",
            false => "Fragment",
        };
        let range = self.name_position;
        let mut terms = vec![];
        if self.is_grammar {
            terms.push(self.extension())
        }
        DocumentSymbol {
            name: name.to_string(),
            detail: Some(self.name.to_owned()),
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
        let range = self.name_position;
        let n = self.extensions.len();
        match n {
            0 => {
                detail = 0.to_string();
                children = None;
            }
            _ => {
                detail = n.to_string();
                children = Some(self.extensions.iter().enumerate().map(|(i, (e, r))| self.extension_item(i, e, r)).collect());
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
    fn extension_item(&self, index: usize, ext: &String, range: &Range) -> DocumentSymbol {
        let range = *range;
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
