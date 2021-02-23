pub mod ast;
pub mod gst;

mod errors;

pub use errors::{Result, MyError};

use tree_sitter::Node;



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    sym_Id = 1,
    anon_sym_LBRACE = 2,
    anon_sym_COMMA = 3,
    anon_sym_RBRACE = 4,
    sym_Grammar = 5,
    sym_whitespace = 6,
    sym_Fragment = 7,
    sym_Ignore = 8,
    anon_sym_EQ = 9,
    anon_sym_CARET = 10,
    anon_sym_BANG = 11,
    anon_sym_TILDE = 12,
    anon_sym_PIPE = 13,
    anon_sym_LT_DASH = 14,
    sym_Unsigned = 15,
    sym__sign = 16,
    anon_sym_SQUOTE = 17,
    aux_sym_String_token1 = 18,
    anon_sym_DQUOTE = 19,
    sym_Regex = 20,
    sym_Eos = 21,
    sym_Program = 22,
    sym_GrammarStatement = 23,
    sym_FragmentStatement = 24,
    sym_AssignStatement = 25,
    sym__expression = 26,
    sym_UnaryExpression = 27,
    sym_BinaryExpression = 28,
    sym_String = 29,
    aux_sym_Program_repeat1 = 30,
    aux_sym__grammar_exts_repeat1 = 31,
}
