use std::collections::BTreeMap;
use std::lazy::SyncLazy;
use yggdrasil_bootstrap::ast::{Symbol, SymbolPath};
use super::*;

pub fn build_symbol(path: &SymbolPath) -> BareExpression {
    match path.symbol.len() {
        1 => unsafe {
            build_maybe_builtin(path.symbol.get_unchecked(0))
        }
        _ => {
            BareExpression::StringLiteral(path.symbol.iter().map(|e| e.data.as_ref()).collect::<Vec<_>>().join("__"))
        }
    }
}

fn build_maybe_builtin(s: &Symbol) -> BareExpression {
    match s.data.as_str() {
        "ANY" => BareExpression::Dot,
        "EOI" => BareExpression::EndOfInput,
        _ => BareExpression::StringLiteral(s.data.to_owned())
    }
}
