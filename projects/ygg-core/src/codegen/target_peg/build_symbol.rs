use super::*;
use yggdrasil_bootstrap::ast::{Symbol, SymbolPath};

pub fn build_symbol(path: &SymbolPath) -> BareExpression {
    match path.symbol.len() {
        1 => unsafe { build_maybe_builtin(path.symbol.get_unchecked(0)) },
        // all symbol must link to eliminate scope at codegen
        _ => unreachable!("Unlinked Symbol {:?}", path),
    }
}

fn build_maybe_builtin(s: &Symbol) -> BareExpression {
    match s.data.as_str() {
        "ANY" => BareExpression::Dot,
        "EOI" => BareExpression::EndOfInput,
        _ => BareExpression::StringLiteral(s.data.to_owned()),
    }
}
