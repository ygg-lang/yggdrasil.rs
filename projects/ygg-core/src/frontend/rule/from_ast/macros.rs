use super::*;
use yggdrasil_bootstrap::ast::MacroCall;

impl<'i> GrammarContext<'i> {
    pub(super) fn read_macros(&mut self, call: MacroCall) {
        let path = call.symbol.symbol;
        assert!(path.len(), 1);
        let symbol = path.get(0).map(|s| s.data.as_str()).unwrap();
        match symbol {
            "clear_hook" => {}
            "hook" => {}
            _ => (),
        }
    }
}
