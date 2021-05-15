pub mod ast;
// pub mod atomic;
// pub mod gst;

mod errors;
#[cfg(feature = "lsp")]
mod lsp;

#[cfg(feature = "lsp")]
pub use lsp::convert_range;

pub use errors::{Result, YGGError};
use std::mem::transmute;
use tree_sitter::{Node, TreeCursor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    sym_id = 1,
    anon_sym_LBRACE = 2,
    anon_sym_COMMA = 3,
    anon_sym_RBRACE = 4,
    anon_sym_LBRACK = 5,
    anon_sym_RBRACK = 6,
    sym_grammar = 7,
    sym_fragment = 8,
    sym_ignore = 9,
    anon_sym_PIPE = 10,
    anon_sym_EQ = 11,
    anon_sym__EQ = 12,
    anon_sym_AT_EQ = 13,
    anon_sym_CARET_EQ = 14,
    anon_sym_LPAREN = 15,
    anon_sym_RPAREN = 16,
    anon_sym_CARET = 17,
    anon_sym_QMARK = 18,
    anon_sym_STAR = 19,
    anon_sym_PLUS = 20,
    anon_sym_TILDE = 21,
    anon_sym_LT_DASH = 22,
    anon_sym_POUND = 23,
    aux_sym_choice_tag_token1 = 24,
    anon_sym_COLON = 25,
    anon_sym_AT = 26,
    anon_sym_DOT = 27,
    sym_unsigned = 28,
    sym__sign = 29,
    anon_sym_SQUOTE = 30,
    aux_sym_string_token1 = 31,
    anon_sym_DQUOTE = 32,
    aux_sym_string_token2 = 33,
    anon_sym_SLASH = 34,
    aux_sym_regex_long_token1 = 35,
    anon_sym_LBRACK_CARET = 36,
    aux_sym_regex_range_item_token1 = 37,
    anon_sym_DASH = 38,
    anon_sym_BSLASHp = 39,
    aux_sym_regex_set_token1 = 40,
    sym_eos = 41,
    sym_comment_doc = 42,
    sym_COMMENT = 43,
    sym_NEWLINE = 44,
    sym_WHITESPACE = 45,
    sym_program = 46,
    sym_statement = 47,
    sym_grammar_statement = 48,
    sym_fragment_statement = 49,
    sym_ignore_statement = 50,
    sym_assign_statement = 51,
    sym_eq = 52,
    sym_expression = 53,
    sym_unary_prefix = 54,
    sym_unary_suffix = 55,
    sym__prefix_op = 56,
    sym__suffix_op = 57,
    sym_concat_expression = 58,
    sym_choice_expression = 59,
    sym_field_expr = 60,
    sym_data = 61,
    sym_choice_tag = 62,
    sym_macro_call = 63,
    sym_string = 64,
    sym_regex_long = 65,
    sym_regex_range = 66,
    sym_regex_range_item = 67,
    sym_regex_range_item_group = 68,
    sym_regex_set = 69,
    aux_sym_program_repeat1 = 70,
    aux_sym_grammar_statement_repeat1 = 71,
    aux_sym_ignore_statement_repeat1 = 72,
    aux_sym_macro_call_repeat1 = 73,
    aux_sym_regex_range_repeat1 = 74,
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
