pub mod ast;
// pub mod atomic;
// pub mod gst;

mod errors;

pub use errors::{MyError, Result};
use tree_sitter::TreeCursor;
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
    anon_sym__EQ = 9,
    anon_sym_CARET = 10,
    anon_sym_BANG = 11,
    anon_sym_TILDE = 12,
    anon_sym_PIPE = 13,
    anon_sym_LT_DASH = 14,
    sym_unsigned = 15,
    sym__sign = 16,
    anon_sym_SQUOTE = 17,
    aux_sym_string_token1 = 18,
    anon_sym_DQUOTE = 19,
    sym_Regex = 20,
    sym_eos = 21,
    sym_whitespace = 22,
    sym_program = 23,
    sym_grammar_statement = 24,
    sym_fragment_statement = 25,
    sym_assign_statement = 26,
    sym__expression = 27,
    sym_unary_expression = 28,
    sym_binary_expression = 29,
    sym_string = 30,
    aux_sym_program_repeat1 = 31,
    aux_sym__grammar_exts_repeat1 = 32,
}

macro_rules! from_node {
    ($t:ty, $n:ident, $e: expr) => {
    impl<'a> From<$t> for SyntaxKind {
        fn from($n: $t) -> Self {
            unsafe { transmute::<u16, Self>($e) }
        }
    }
    };
}

from_node!(Node<'a>, node, node.kind_id());
from_node!(&Node<'a>, node, node.kind_id());
from_node!(&TreeCursor<'a>, cursor, cursor.node().kind_id());