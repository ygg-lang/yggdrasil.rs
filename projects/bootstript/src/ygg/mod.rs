pub mod ast;
// pub mod atomic;
// pub mod gst;

mod errors;

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
    sym_grammar = 5,
    sym_fragment = 6,
    anon_sym_PIPE = 7,
    anon_sym_EQ = 8,
    anon_sym__EQ = 9,
    anon_sym_AT_EQ = 10,
    anon_sym_LPAREN = 11,
    anon_sym_RPAREN = 12,
    anon_sym_CARET = 13,
    anon_sym_QMARK = 14,
    anon_sym_STAR = 15,
    anon_sym_PLUS = 16,
    anon_sym_TILDE = 17,
    anon_sym_LT_DASH = 18,
    aux_sym_variant_tag_token1 = 19,
    anon_sym_AT = 20,
    anon_sym_DOT = 21,
    sym_unsigned = 22,
    sym__sign = 23,
    anon_sym_SQUOTE = 24,
    aux_sym_string_token1 = 25,
    anon_sym_DQUOTE = 26,
    aux_sym_string_token2 = 27,
    anon_sym_SLASH = 28,
    aux_sym_regex_long_token1 = 29,
    anon_sym_LBRACK_CARET = 30,
    anon_sym_LBRACK = 31,
    anon_sym_RBRACK = 32,
    aux_sym_regex_range_item_token1 = 33,
    anon_sym_DASH = 34,
    anon_sym_BSLASHp = 35,
    aux_sym_regex_set_token1 = 36,
    sym_eos = 37,
    sym_NEWLINE = 38,
    sym_WHITESPACE = 39,
    sym_program = 40,
    sym_statement = 41,
    sym_grammar_statement = 42,
    sym_fragment_statement = 43,
    sym_assign_statement = 44,
    sym_eq = 45,
    sym_expression = 46,
    sym_unary_prefix = 47,
    sym_unary_suffix = 48,
    sym__prefix_op = 49,
    sym__suffix_op = 50,
    sym_concat_expr = 51,
    sym_or_expr = 52,
    sym_field_expr = 53,
    sym_variant_tag = 54,
    sym_macro_call = 55,
    sym_string = 56,
    sym_regex_long = 57,
    sym_regex_range = 58,
    sym_regex_range_item = 59,
    sym_regex_range_item_group = 60,
    sym_regex_set = 61,
    aux_sym_program_repeat1 = 62,
    aux_sym__grammar_exts_repeat1 = 63,
    aux_sym_concat_expr_repeat1 = 64,
    aux_sym_or_expr_repeat1 = 65,
    aux_sym_macro_call_repeat1 = 66,
    aux_sym_regex_range_repeat1 = 67,
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
