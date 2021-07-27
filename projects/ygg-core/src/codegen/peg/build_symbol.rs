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
        "ASCII_WHITESPACE" => BareExpression::Regex(String::from("[0-9]")),
        "ASCII_HEX" => BareExpression::Regex(String::from("[0-9a-fA-F]")),
        "ASCII_HEX+" => BareExpression::Regex(String::from("[0-9a-fA-F]+")),
        "ASCII_OCT" => BareExpression::Regex(String::from("[0-7]")),
        "ASCII_OCT+" => BareExpression::Regex(String::from("[0-7]+")),
        "ASCII_BIN" => BareExpression::Regex(String::from("[0-1]")),
        "ASCII_BIN+" => BareExpression::Regex(String::from("[0-1]+")),
        "ASCII_ALPHA_DIGIT" => BareExpression::Regex(String::from("[0-9a-zA-Z]")),
        "ASCII_UPPERCASE" => BareExpression::Regex(String::from("[A-Z]")),
        "ASCII_LOWERCASE" => BareExpression::Regex(String::from("[a-z]+")),
        "ASCII_WHITESPACE" => BareExpression::Regex(String::from("[0-9]")),
        "ASCII_WHITESPACE" => BareExpression::Regex(String::from("[0-9]")),
        "ASCII_WHITESPACE" => BareExpression::Regex(String::from("[0-9]")),
        "ASCII_WHITESPACE" => BareExpression::Regex(String::from("[0-9]")),
        "ASCII_WHITESPACE" => BareExpression::Regex(String::from("[0-9]")),
        "ASCII_WHITESPACE" => BareExpression::Regex(String::from("[0-9]")),
        _ => BareExpression::StringLiteral(s.data.to_owned())
    }
}