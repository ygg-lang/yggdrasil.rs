
use super::*;

impl GrammarState {
    pub fn report_meta(&self) -> HintItems {
        let document_symbol = vec![
            DocumentSymbol {
                name: "Metadata".to_string(),
                detail: Some("ddddd".to_string()),
                kind: SymbolKind::File,
                tags: None,
                deprecated: None,
                range: Default::default(),
                selection_range: Default::default(),
                children: Some(self.top_area())
            },
            DocumentSymbol {
                name: "323333".to_string(),
                detail: Some("ssss".to_string()),
                kind: SymbolKind::File,
                tags: None,
                deprecated: None,
                range: Default::default(),
                selection_range: Default::default(),
                children: unsafe {
                    Some(test_all_symbol())
                }
            },
        ];

        HintItems {
            diagnostic: vec![],
            code_lens: vec![],
            document_symbol,
        }
    }
    fn top_area(&self) -> Vec<DocumentSymbol> {
        vec![
            DocumentSymbol {
                name: self.name.to_owned(),
                detail: Some("kkkkkk".to_string()),
                kind: SymbolKind::Namespace,
                tags: None,
                deprecated: None,
                range: Default::default(),
                selection_range: Default::default(),
                children: None
            },
            DocumentSymbol {
                name: "SSS".to_string(),
                detail: Some("rrrrrrr".to_string()),
                kind: SymbolKind::File,
                tags: None,
                deprecated: None,
                range: Default::default(),
                selection_range: Default::default(),
                children: None
            }
        ]
    }
}


unsafe fn test_all_symbol() -> Vec<DocumentSymbol> {
    (1u8..=26)
        .map(|n| transmute::<u8, SymbolKind>(n))
        .map(
        | k | DocumentSymbol {
            name: format!("{:?}", k),
            detail: Some(format!("{:?}", k)),
            kind: k,
            tags: None,
            deprecated: None,
            range: Default::default(),
            selection_range: Default::default(),
            children: None
        }
    ).collect()
}