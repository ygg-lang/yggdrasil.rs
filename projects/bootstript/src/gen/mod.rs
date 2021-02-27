pub mod ast;
pub mod atomic;
pub mod gst;

mod errors;

pub use errors::{MyError, Result};

use std::mem::transmute;
use tree_sitter::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    sym_id = 1,
    anon_sym_LBRACE = 2,
    anon_sym_COMMA = 3,
    anon_sym_RBRACE = 4,
    sym_grammar = 5,
    sym_fragment = 6,
    sym_ignore = 7,
    anon_sym_EQ = 8,
    anon_sym_CARET = 9,
    anon_sym_BANG = 10,
    anon_sym_TILDE = 11,
    anon_sym_PIPE = 12,
    anon_sym_LT_DASH = 13,
    sym_unsigned = 14,
    sym__sign = 15,
    anon_sym_SQUOTE = 16,
    aux_sym_string_token1 = 17,
    anon_sym_DQUOTE = 18,
    sym_Regex = 19,
    sym_eos = 20,
    sym_whitespace = 21,
    sym_program = 22,
    sym_grammar_statement = 23,
    sym_fragment_statement = 24,
    sym_assign_statement = 25,
    sym__expression = 26,
    sym_unary_expression = 27,
    sym_binary_expression = 28,
    sym_string = 29,
    aux_sym_program_repeat1 = 30,
    aux_sym__grammar_exts_repeat1 = 31,
}

impl SyntaxKind {
    pub fn node_kind(node: &Node) -> Self {
        unsafe { transmute::<u16, Self>(node.kind_id()) }
    }
}
