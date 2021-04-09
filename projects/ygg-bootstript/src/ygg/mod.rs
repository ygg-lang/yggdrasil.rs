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
    anon_sym_POUND = 19,
    aux_sym_choice_tag_token1 = 20,
    anon_sym_COLON = 21,
    anon_sym_AT = 22,
    anon_sym_DOT = 23,
    sym_unsigned = 24,
    sym__sign = 25,
    anon_sym_SQUOTE = 26,
    aux_sym_string_token1 = 27,
    anon_sym_DQUOTE = 28,
    aux_sym_string_token2 = 29,
    anon_sym_SLASH = 30,
    aux_sym_regex_long_token1 = 31,
    anon_sym_LBRACK_CARET = 32,
    anon_sym_LBRACK = 33,
    anon_sym_RBRACK = 34,
    aux_sym_regex_range_item_token1 = 35,
    anon_sym_DASH = 36,
    anon_sym_BSLASHp = 37,
    aux_sym_regex_set_token1 = 38,
    sym_eos = 39,
    sym_NEWLINE = 40,
    sym_WHITESPACE = 41,
    sym_program = 42,
    sym_statement = 43,
    sym_grammar_statement = 44,
    sym_fragment_statement = 45,
    sym_assign_statement = 46,
    sym_eq = 47,
    sym_expression = 48,
    sym_unary_prefix = 49,
    sym_unary_suffix = 50,
    sym__prefix_op = 51,
    sym__suffix_op = 52,
    sym_concat_expression = 53,
    sym_choice_expression = 54,
    sym_field_expr = 55,
    sym_data = 56,
    sym_choice_tag = 57,
    sym_macro_call = 58,
    sym_string = 59,
    sym_regex_long = 60,
    sym_regex_range = 61,
    sym_regex_range_item = 62,
    sym_regex_range_item_group = 63,
    sym_regex_set = 64,
    aux_sym_program_repeat1 = 65,
    aux_sym__grammar_exts_repeat1 = 66,
    aux_sym_macro_call_repeat1 = 67,
    aux_sym_regex_range_repeat1 = 68,
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
