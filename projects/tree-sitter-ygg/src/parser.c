#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 215
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 75
#define ALIAS_COUNT 0
#define TOKEN_COUNT 46
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 20
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 29

enum {
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
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_id] = "id",
  [anon_sym_LBRACE] = "{",
  [anon_sym_COMMA] = ",",
  [anon_sym_RBRACE] = "}",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [sym_grammar] = "grammar",
  [sym_fragment] = "fragment",
  [sym_ignore] = "ignore",
  [anon_sym_PIPE] = "|",
  [anon_sym_EQ] = "=",
  [anon_sym__EQ] = "_=",
  [anon_sym_AT_EQ] = "@=",
  [anon_sym_CARET_EQ] = "^=",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_CARET] = "^",
  [anon_sym_QMARK] = "\?",
  [anon_sym_STAR] = "*",
  [anon_sym_PLUS] = "+",
  [anon_sym_TILDE] = "~",
  [anon_sym_LT_DASH] = "<-",
  [anon_sym_POUND] = "#",
  [aux_sym_choice_tag_token1] = "choice_tag_token1",
  [anon_sym_COLON] = ":",
  [anon_sym_AT] = "@",
  [anon_sym_DOT] = ".",
  [sym_unsigned] = "unsigned",
  [sym__sign] = "_sign",
  [anon_sym_SQUOTE] = "'",
  [aux_sym_string_token1] = "string_token1",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_token2] = "string_token2",
  [anon_sym_SLASH] = "/",
  [aux_sym_regex_long_token1] = "regex_long_token1",
  [anon_sym_LBRACK_CARET] = "[^",
  [aux_sym_regex_range_item_token1] = "regex_range_item_token1",
  [anon_sym_DASH] = "-",
  [anon_sym_BSLASHp] = "\\p",
  [aux_sym_regex_set_token1] = "regex_set_token1",
  [sym_eos] = "eos",
  [sym_comment_doc] = "comment_doc",
  [sym_COMMENT] = "COMMENT",
  [sym_NEWLINE] = "NEWLINE",
  [sym_WHITESPACE] = "WHITESPACE",
  [sym_program] = "program",
  [sym_statement] = "statement",
  [sym_grammar_statement] = "grammar_statement",
  [sym_fragment_statement] = "fragment_statement",
  [sym_ignore_statement] = "ignore_statement",
  [sym_assign_statement] = "assign_statement",
  [sym_eq] = "eq",
  [sym_expression] = "expression",
  [sym_unary_prefix] = "unary_prefix",
  [sym_unary_suffix] = "unary_suffix",
  [sym__prefix_op] = "_prefix_op",
  [sym__suffix_op] = "_suffix_op",
  [sym_concat_expression] = "concat_expression",
  [sym_choice_expression] = "choice_expression",
  [sym_field_expr] = "field_expr",
  [sym_data] = "data",
  [sym_choice_tag] = "choice_tag",
  [sym_macro_call] = "macro_call",
  [sym_string] = "string",
  [sym_regex_long] = "regex_long",
  [sym_regex_range] = "regex_range",
  [sym_regex_range_item] = "regex_range_item",
  [sym_regex_range_item_group] = "regex_range_item_group",
  [sym_regex_set] = "regex_set",
  [aux_sym_program_repeat1] = "program_repeat1",
  [aux_sym_grammar_statement_repeat1] = "grammar_statement_repeat1",
  [aux_sym_ignore_statement_repeat1] = "ignore_statement_repeat1",
  [aux_sym_macro_call_repeat1] = "macro_call_repeat1",
  [aux_sym_regex_range_repeat1] = "regex_range_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_id] = sym_id,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym_grammar] = sym_grammar,
  [sym_fragment] = sym_fragment,
  [sym_ignore] = sym_ignore,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym__EQ] = anon_sym__EQ,
  [anon_sym_AT_EQ] = anon_sym_AT_EQ,
  [anon_sym_CARET_EQ] = anon_sym_CARET_EQ,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_CARET] = anon_sym_CARET,
  [anon_sym_QMARK] = anon_sym_QMARK,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_LT_DASH] = anon_sym_LT_DASH,
  [anon_sym_POUND] = anon_sym_POUND,
  [aux_sym_choice_tag_token1] = aux_sym_choice_tag_token1,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_DOT] = anon_sym_DOT,
  [sym_unsigned] = sym_unsigned,
  [sym__sign] = sym__sign,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [aux_sym_string_token1] = aux_sym_string_token1,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_token2] = aux_sym_string_token2,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [aux_sym_regex_long_token1] = aux_sym_regex_long_token1,
  [anon_sym_LBRACK_CARET] = anon_sym_LBRACK_CARET,
  [aux_sym_regex_range_item_token1] = aux_sym_regex_range_item_token1,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_BSLASHp] = anon_sym_BSLASHp,
  [aux_sym_regex_set_token1] = aux_sym_regex_set_token1,
  [sym_eos] = sym_eos,
  [sym_comment_doc] = sym_comment_doc,
  [sym_COMMENT] = sym_COMMENT,
  [sym_NEWLINE] = sym_NEWLINE,
  [sym_WHITESPACE] = sym_WHITESPACE,
  [sym_program] = sym_program,
  [sym_statement] = sym_statement,
  [sym_grammar_statement] = sym_grammar_statement,
  [sym_fragment_statement] = sym_fragment_statement,
  [sym_ignore_statement] = sym_ignore_statement,
  [sym_assign_statement] = sym_assign_statement,
  [sym_eq] = sym_eq,
  [sym_expression] = sym_expression,
  [sym_unary_prefix] = sym_unary_prefix,
  [sym_unary_suffix] = sym_unary_suffix,
  [sym__prefix_op] = sym__prefix_op,
  [sym__suffix_op] = sym__suffix_op,
  [sym_concat_expression] = sym_concat_expression,
  [sym_choice_expression] = sym_choice_expression,
  [sym_field_expr] = sym_field_expr,
  [sym_data] = sym_data,
  [sym_choice_tag] = sym_choice_tag,
  [sym_macro_call] = sym_macro_call,
  [sym_string] = sym_string,
  [sym_regex_long] = sym_regex_long,
  [sym_regex_range] = sym_regex_range,
  [sym_regex_range_item] = sym_regex_range_item,
  [sym_regex_range_item_group] = sym_regex_range_item_group,
  [sym_regex_set] = sym_regex_set,
  [aux_sym_program_repeat1] = aux_sym_program_repeat1,
  [aux_sym_grammar_statement_repeat1] = aux_sym_grammar_statement_repeat1,
  [aux_sym_ignore_statement_repeat1] = aux_sym_ignore_statement_repeat1,
  [aux_sym_macro_call_repeat1] = aux_sym_macro_call_repeat1,
  [aux_sym_regex_range_repeat1] = aux_sym_regex_range_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_id] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym_grammar] = {
    .visible = true,
    .named = true,
  },
  [sym_fragment] = {
    .visible = true,
    .named = true,
  },
  [sym_ignore] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym__EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_QMARK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_choice_tag_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [sym_unsigned] = {
    .visible = true,
    .named = true,
  },
  [sym__sign] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_regex_long_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_LBRACK_CARET] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_regex_range_item_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BSLASHp] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_regex_set_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_eos] = {
    .visible = true,
    .named = true,
  },
  [sym_comment_doc] = {
    .visible = true,
    .named = true,
  },
  [sym_COMMENT] = {
    .visible = true,
    .named = true,
  },
  [sym_NEWLINE] = {
    .visible = true,
    .named = true,
  },
  [sym_WHITESPACE] = {
    .visible = true,
    .named = true,
  },
  [sym_program] = {
    .visible = true,
    .named = true,
  },
  [sym_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_grammar_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_fragment_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_ignore_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_assign_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_eq] = {
    .visible = true,
    .named = true,
  },
  [sym_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_unary_prefix] = {
    .visible = true,
    .named = true,
  },
  [sym_unary_suffix] = {
    .visible = true,
    .named = true,
  },
  [sym__prefix_op] = {
    .visible = false,
    .named = true,
  },
  [sym__suffix_op] = {
    .visible = false,
    .named = true,
  },
  [sym_concat_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_choice_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_field_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_data] = {
    .visible = true,
    .named = true,
  },
  [sym_choice_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_macro_call] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_regex_long] = {
    .visible = true,
    .named = true,
  },
  [sym_regex_range] = {
    .visible = true,
    .named = true,
  },
  [sym_regex_range_item] = {
    .visible = true,
    .named = true,
  },
  [sym_regex_range_item_group] = {
    .visible = true,
    .named = true,
  },
  [sym_regex_set] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_program_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_grammar_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_ignore_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_macro_call_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_regex_range_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_base = 1,
  field_dot = 2,
  field_eq = 3,
  field_expression = 4,
  field_ext = 5,
  field_id = 6,
  field_is_neg = 7,
  field_item = 8,
  field_lhs = 9,
  field_mode = 10,
  field_name = 11,
  field_op = 12,
  field_prefix = 13,
  field_rhs = 14,
  field_set = 15,
  field_statement = 16,
  field_suffix = 17,
  field_tag = 18,
  field_text = 19,
  field_ty = 20,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_base] = "base",
  [field_dot] = "dot",
  [field_eq] = "eq",
  [field_expression] = "expression",
  [field_ext] = "ext",
  [field_id] = "id",
  [field_is_neg] = "is_neg",
  [field_item] = "item",
  [field_lhs] = "lhs",
  [field_mode] = "mode",
  [field_name] = "name",
  [field_op] = "op",
  [field_prefix] = "prefix",
  [field_rhs] = "rhs",
  [field_set] = "set",
  [field_statement] = "statement",
  [field_suffix] = "suffix",
  [field_tag] = "tag",
  [field_text] = "text",
  [field_ty] = "ty",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 1},
  [5] = {.index = 4, .length = 2},
  [6] = {.index = 6, .length = 2},
  [7] = {.index = 8, .length = 1},
  [8] = {.index = 9, .length = 3},
  [9] = {.index = 12, .length = 1},
  [10] = {.index = 13, .length = 1},
  [11] = {.index = 14, .length = 3},
  [12] = {.index = 17, .length = 2},
  [13] = {.index = 19, .length = 2},
  [14] = {.index = 21, .length = 2},
  [15] = {.index = 23, .length = 1},
  [16] = {.index = 24, .length = 2},
  [17] = {.index = 26, .length = 2},
  [18] = {.index = 28, .length = 3},
  [19] = {.index = 31, .length = 2},
  [20] = {.index = 33, .length = 1},
  [21] = {.index = 34, .length = 3},
  [22] = {.index = 37, .length = 2},
  [23] = {.index = 39, .length = 1},
  [24] = {.index = 40, .length = 3},
  [25] = {.index = 43, .length = 1},
  [26] = {.index = 44, .length = 3},
  [27] = {.index = 47, .length = 4},
  [28] = {.index = 51, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_statement, 0},
  [1] =
    {field_statement, 0, .inherited = true},
  [2] =
    {field_id, 1},
  [3] =
    {field_item, 1},
  [4] =
    {field_statement, 0, .inherited = true},
    {field_statement, 1, .inherited = true},
  [6] =
    {field_ext, 2},
    {field_id, 1},
  [8] =
    {field_expression, 0},
  [9] =
    {field_eq, 1},
    {field_id, 0},
    {field_rhs, 2},
  [12] =
    {field_item, 2},
  [13] =
    {field_is_neg, 0},
  [14] =
    {field_eq, 1},
    {field_id, 0},
    {field_rhs, 3},
  [17] =
    {field_base, 0},
    {field_suffix, 1},
  [19] =
    {field_base, 1},
    {field_prefix, 0},
  [21] =
    {field_ext, 3},
    {field_id, 1},
  [23] =
    {field_text, 1},
  [24] =
    {field_item, 2},
    {field_item, 3, .inherited = true},
  [26] =
    {field_item, 0, .inherited = true},
    {field_item, 1, .inherited = true},
  [28] =
    {field_lhs, 0},
    {field_op, 1},
    {field_rhs, 2},
  [31] =
    {field_expression, 0},
    {field_tag, 2},
  [33] =
    {field_ext, 1},
  [34] =
    {field_ext, 3},
    {field_ext, 4, .inherited = true},
    {field_id, 1},
  [37] =
    {field_ext, 0, .inherited = true},
    {field_ext, 1, .inherited = true},
  [39] =
    {field_set, 2},
  [40] =
    {field_expression, 0},
    {field_mode, 3},
    {field_tag, 2},
  [43] =
    {field_name, 1},
  [44] =
    {field_expression, 0},
    {field_tag, 2},
    {field_ty, 4},
  [47] =
    {field_expression, 0},
    {field_mode, 3},
    {field_tag, 2},
    {field_ty, 5},
  [51] =
    {field_dot, 3},
    {field_name, 1},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static inline bool sym_id_character_set_1(int32_t c) {
  return (c < 43020
    ? (c < 4096
      ? (c < 2693
        ? (c < 1969
          ? (c < 910
            ? (c < 736
              ? (c < 186
                ? (c < 170
                  ? (c < 'a'
                    ? (c >= 'A' && c <= '_')
                    : c <= 'z')
                  : (c <= 170 || c == 181))
                : (c <= 186 || (c < 248
                  ? (c < 216
                    ? (c >= 192 && c <= 214)
                    : c <= 246)
                  : (c <= 705 || (c >= 710 && c <= 721)))))
              : (c <= 740 || (c < 891
                ? (c < 880
                  ? (c < 750
                    ? c == 748
                    : c <= 750)
                  : (c <= 884 || (c >= 886 && c <= 887)))
                : (c <= 893 || (c < 904
                  ? (c < 902
                    ? c == 895
                    : c <= 902)
                  : (c <= 906 || c == 908))))))
            : (c <= 929 || (c < 1646
              ? (c < 1369
                ? (c < 1162
                  ? (c < 1015
                    ? (c >= 931 && c <= 1013)
                    : c <= 1153)
                  : (c <= 1327 || (c >= 1329 && c <= 1366)))
                : (c <= 1369 || (c < 1519
                  ? (c < 1488
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1514)
                  : (c <= 1522 || (c >= 1568 && c <= 1610)))))
              : (c <= 1647 || (c < 1786
                ? (c < 1765
                  ? (c < 1749
                    ? (c >= 1649 && c <= 1747)
                    : c <= 1749)
                  : (c <= 1766 || (c >= 1774 && c <= 1775)))
                : (c <= 1788 || (c < 1810
                  ? (c < 1808
                    ? c == 1791
                    : c <= 1808)
                  : (c <= 1839 || (c >= 1869 && c <= 1957)))))))))
          : (c <= 1969 || (c < 2474
            ? (c < 2208
              ? (c < 2074
                ? (c < 2042
                  ? (c < 2036
                    ? (c >= 1994 && c <= 2026)
                    : c <= 2037)
                  : (c <= 2042 || (c >= 2048 && c <= 2069)))
                : (c <= 2074 || (c < 2112
                  ? (c < 2088
                    ? c == 2084
                    : c <= 2088)
                  : (c <= 2136 || (c >= 2144 && c <= 2154)))))
              : (c <= 2228 || (c < 2392
                ? (c < 2365
                  ? (c < 2308
                    ? (c >= 2230 && c <= 2247)
                    : c <= 2361)
                  : (c <= 2365 || c == 2384))
                : (c <= 2401 || (c < 2447
                  ? (c < 2437
                    ? (c >= 2417 && c <= 2432)
                    : c <= 2444)
                  : (c <= 2448 || (c >= 2451 && c <= 2472)))))))
            : (c <= 2480 || (c < 2575
              ? (c < 2524
                ? (c < 2493
                  ? (c < 2486
                    ? c == 2482
                    : c <= 2489)
                  : (c <= 2493 || c == 2510))
                : (c <= 2525 || (c < 2556
                  ? (c < 2544
                    ? (c >= 2527 && c <= 2529)
                    : c <= 2545)
                  : (c <= 2556 || (c >= 2565 && c <= 2570)))))
              : (c <= 2576 || (c < 2616
                ? (c < 2610
                  ? (c < 2602
                    ? (c >= 2579 && c <= 2600)
                    : c <= 2608)
                  : (c <= 2611 || (c >= 2613 && c <= 2614)))
                : (c <= 2617 || (c < 2654
                  ? (c >= 2649 && c <= 2652)
                  : (c <= 2654 || (c >= 2674 && c <= 2676)))))))))))
        : (c <= 2701 || (c < 3214
          ? (c < 2947
            ? (c < 2821
              ? (c < 2741
                ? (c < 2730
                  ? (c < 2707
                    ? (c >= 2703 && c <= 2705)
                    : c <= 2728)
                  : (c <= 2736 || (c >= 2738 && c <= 2739)))
                : (c <= 2745 || (c < 2784
                  ? (c < 2768
                    ? c == 2749
                    : c <= 2768)
                  : (c <= 2785 || c == 2809))))
              : (c <= 2828 || (c < 2869
                ? (c < 2858
                  ? (c < 2835
                    ? (c >= 2831 && c <= 2832)
                    : c <= 2856)
                  : (c <= 2864 || (c >= 2866 && c <= 2867)))
                : (c <= 2873 || (c < 2911
                  ? (c < 2908
                    ? c == 2877
                    : c <= 2909)
                  : (c <= 2913 || c == 2929))))))
            : (c <= 2947 || (c < 3024
              ? (c < 2972
                ? (c < 2962
                  ? (c < 2958
                    ? (c >= 2949 && c <= 2954)
                    : c <= 2960)
                  : (c <= 2965 || (c >= 2969 && c <= 2970)))
                : (c <= 2972 || (c < 2984
                  ? (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)
                  : (c <= 2986 || (c >= 2990 && c <= 3001)))))
              : (c <= 3024 || (c < 3133
                ? (c < 3090
                  ? (c < 3086
                    ? (c >= 3077 && c <= 3084)
                    : c <= 3088)
                  : (c <= 3112 || (c >= 3114 && c <= 3129)))
                : (c <= 3133 || (c < 3200
                  ? (c < 3168
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3169)
                  : (c <= 3200 || (c >= 3205 && c <= 3212)))))))))
          : (c <= 3216 || (c < 3520
            ? (c < 3346
              ? (c < 3294
                ? (c < 3253
                  ? (c < 3242
                    ? (c >= 3218 && c <= 3240)
                    : c <= 3251)
                  : (c <= 3257 || c == 3261))
                : (c <= 3294 || (c < 3332
                  ? (c < 3313
                    ? (c >= 3296 && c <= 3297)
                    : c <= 3314)
                  : (c <= 3340 || (c >= 3342 && c <= 3344)))))
              : (c <= 3386 || (c < 3450
                ? (c < 3412
                  ? (c < 3406
                    ? c == 3389
                    : c <= 3406)
                  : (c <= 3414 || (c >= 3423 && c <= 3425)))
                : (c <= 3455 || (c < 3507
                  ? (c < 3482
                    ? (c >= 3461 && c <= 3478)
                    : c <= 3505)
                  : (c <= 3515 || c == 3517))))))
            : (c <= 3526 || (c < 3762
              ? (c < 3716
                ? (c < 3648
                  ? (c < 3634
                    ? (c >= 3585 && c <= 3632)
                    : c <= 3634)
                  : (c <= 3654 || (c >= 3713 && c <= 3714)))
                : (c <= 3716 || (c < 3749
                  ? (c < 3724
                    ? (c >= 3718 && c <= 3722)
                    : c <= 3747)
                  : (c <= 3749 || (c >= 3751 && c <= 3760)))))
              : (c <= 3762 || (c < 3840
                ? (c < 3782
                  ? (c < 3776
                    ? c == 3773
                    : c <= 3780)
                  : (c <= 3782 || (c >= 3804 && c <= 3807)))
                : (c <= 3840 || (c < 3913
                  ? (c >= 3904 && c <= 3911)
                  : (c <= 3948 || (c >= 3976 && c <= 3980)))))))))))))
      : (c <= 4138 || (c < 8025
        ? (c < 5952
          ? (c < 4752
            ? (c < 4295
              ? (c < 4197
                ? (c < 4186
                  ? (c < 4176
                    ? c == 4159
                    : c <= 4181)
                  : (c <= 4189 || c == 4193))
                : (c <= 4198 || (c < 4238
                  ? (c < 4213
                    ? (c >= 4206 && c <= 4208)
                    : c <= 4225)
                  : (c <= 4238 || (c >= 4256 && c <= 4293)))))
              : (c <= 4295 || (c < 4688
                ? (c < 4348
                  ? (c < 4304
                    ? c == 4301
                    : c <= 4346)
                  : (c <= 4680 || (c >= 4682 && c <= 4685)))
                : (c <= 4694 || (c < 4704
                  ? (c < 4698
                    ? c == 4696
                    : c <= 4701)
                  : (c <= 4744 || (c >= 4746 && c <= 4749)))))))
            : (c <= 4784 || (c < 5024
              ? (c < 4808
                ? (c < 4800
                  ? (c < 4792
                    ? (c >= 4786 && c <= 4789)
                    : c <= 4798)
                  : (c <= 4800 || (c >= 4802 && c <= 4805)))
                : (c <= 4822 || (c < 4888
                  ? (c < 4882
                    ? (c >= 4824 && c <= 4880)
                    : c <= 4885)
                  : (c <= 4954 || (c >= 4992 && c <= 5007)))))
              : (c <= 5109 || (c < 5792
                ? (c < 5743
                  ? (c < 5121
                    ? (c >= 5112 && c <= 5117)
                    : c <= 5740)
                  : (c <= 5759 || (c >= 5761 && c <= 5786)))
                : (c <= 5866 || (c < 5902
                  ? (c < 5888
                    ? (c >= 5870 && c <= 5880)
                    : c <= 5900)
                  : (c <= 5905 || (c >= 5920 && c <= 5937)))))))))
          : (c <= 5969 || (c < 7043
            ? (c < 6400
              ? (c < 6108
                ? (c < 6016
                  ? (c < 5998
                    ? (c >= 5984 && c <= 5996)
                    : c <= 6000)
                  : (c <= 6067 || c == 6103))
                : (c <= 6108 || (c < 6314
                  ? (c < 6272
                    ? (c >= 6176 && c <= 6264)
                    : c <= 6312)
                  : (c <= 6314 || (c >= 6320 && c <= 6389)))))
              : (c <= 6430 || (c < 6656
                ? (c < 6528
                  ? (c < 6512
                    ? (c >= 6480 && c <= 6509)
                    : c <= 6516)
                  : (c <= 6571 || (c >= 6576 && c <= 6601)))
                : (c <= 6678 || (c < 6917
                  ? (c < 6823
                    ? (c >= 6688 && c <= 6740)
                    : c <= 6823)
                  : (c <= 6963 || (c >= 6981 && c <= 6987)))))))
            : (c <= 7072 || (c < 7406
              ? (c < 7258
                ? (c < 7168
                  ? (c < 7098
                    ? (c >= 7086 && c <= 7087)
                    : c <= 7141)
                  : (c <= 7203 || (c >= 7245 && c <= 7247)))
                : (c <= 7293 || (c < 7357
                  ? (c < 7312
                    ? (c >= 7296 && c <= 7304)
                    : c <= 7354)
                  : (c <= 7359 || (c >= 7401 && c <= 7404)))))
              : (c <= 7411 || (c < 7960
                ? (c < 7424
                  ? (c < 7418
                    ? (c >= 7413 && c <= 7414)
                    : c <= 7418)
                  : (c <= 7615 || (c >= 7680 && c <= 7957)))
                : (c <= 7965 || (c < 8008
                  ? (c >= 7968 && c <= 8005)
                  : (c <= 8013 || (c >= 8016 && c <= 8023)))))))))))
        : (c <= 8025 || (c < 11631
          ? (c < 8469
            ? (c < 8150
              ? (c < 8118
                ? (c < 8031
                  ? (c < 8029
                    ? c == 8027
                    : c <= 8029)
                  : (c <= 8061 || (c >= 8064 && c <= 8116)))
                : (c <= 8124 || (c < 8134
                  ? (c < 8130
                    ? c == 8126
                    : c <= 8132)
                  : (c <= 8140 || (c >= 8144 && c <= 8147)))))
              : (c <= 8155 || (c < 8319
                ? (c < 8182
                  ? (c < 8178
                    ? (c >= 8160 && c <= 8172)
                    : c <= 8180)
                  : (c <= 8188 || c == 8305))
                : (c <= 8319 || (c < 8455
                  ? (c < 8450
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8450)
                  : (c <= 8455 || (c >= 8458 && c <= 8467)))))))
            : (c <= 8469 || (c < 11264
              ? (c < 8490
                ? (c < 8486
                  ? (c < 8484
                    ? (c >= 8472 && c <= 8477)
                    : c <= 8484)
                  : (c <= 8486 || c == 8488))
                : (c <= 8505 || (c < 8526
                  ? (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)
                  : (c <= 8526 || (c >= 8544 && c <= 8584)))))
              : (c <= 11310 || (c < 11520
                ? (c < 11499
                  ? (c < 11360
                    ? (c >= 11312 && c <= 11358)
                    : c <= 11492)
                  : (c <= 11502 || (c >= 11506 && c <= 11507)))
                : (c <= 11557 || (c < 11565
                  ? c == 11559
                  : (c <= 11565 || (c >= 11568 && c <= 11623)))))))))
          : (c <= 11631 || (c < 12704
            ? (c < 12293
              ? (c < 11704
                ? (c < 11688
                  ? (c < 11680
                    ? (c >= 11648 && c <= 11670)
                    : c <= 11686)
                  : (c <= 11694 || (c >= 11696 && c <= 11702)))
                : (c <= 11710 || (c < 11728
                  ? (c < 11720
                    ? (c >= 11712 && c <= 11718)
                    : c <= 11726)
                  : (c <= 11734 || (c >= 11736 && c <= 11742)))))
              : (c <= 12295 || (c < 12445
                ? (c < 12344
                  ? (c < 12337
                    ? (c >= 12321 && c <= 12329)
                    : c <= 12341)
                  : (c <= 12348 || (c >= 12353 && c <= 12438)))
                : (c <= 12447 || (c < 12549
                  ? (c < 12540
                    ? (c >= 12449 && c <= 12538)
                    : c <= 12543)
                  : (c <= 12591 || (c >= 12593 && c <= 12686)))))))
            : (c <= 12735 || (c < 42623
              ? (c < 42192
                ? (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 40956 || (c >= 40960 && c <= 42124)))
                : (c <= 42237 || (c < 42538
                  ? (c < 42512
                    ? (c >= 42240 && c <= 42508)
                    : c <= 42527)
                  : (c <= 42539 || (c >= 42560 && c <= 42606)))))
              : (c <= 42653 || (c < 42946
                ? (c < 42786
                  ? (c < 42775
                    ? (c >= 42656 && c <= 42735)
                    : c <= 42783)
                  : (c <= 42888 || (c >= 42891 && c <= 42943)))
                : (c <= 42954 || (c < 43011
                  ? (c >= 42997 && c <= 43009)
                  : (c <= 43013 || (c >= 43015 && c <= 43018)))))))))))))))
    : (c <= 43042 || (c < 70453
      ? (c < 66176
        ? (c < 64112
          ? (c < 43697
            ? (c < 43471
              ? (c < 43261
                ? (c < 43250
                  ? (c < 43138
                    ? (c >= 43072 && c <= 43123)
                    : c <= 43187)
                  : (c <= 43255 || c == 43259))
                : (c <= 43262 || (c < 43360
                  ? (c < 43312
                    ? (c >= 43274 && c <= 43301)
                    : c <= 43334)
                  : (c <= 43388 || (c >= 43396 && c <= 43442)))))
              : (c <= 43471 || (c < 43584
                ? (c < 43514
                  ? (c < 43494
                    ? (c >= 43488 && c <= 43492)
                    : c <= 43503)
                  : (c <= 43518 || (c >= 43520 && c <= 43560)))
                : (c <= 43586 || (c < 43642
                  ? (c < 43616
                    ? (c >= 43588 && c <= 43595)
                    : c <= 43638)
                  : (c <= 43642 || (c >= 43646 && c <= 43695)))))))
            : (c <= 43697 || (c < 43793
              ? (c < 43739
                ? (c < 43712
                  ? (c < 43705
                    ? (c >= 43701 && c <= 43702)
                    : c <= 43709)
                  : (c <= 43712 || c == 43714))
                : (c <= 43741 || (c < 43777
                  ? (c < 43762
                    ? (c >= 43744 && c <= 43754)
                    : c <= 43764)
                  : (c <= 43782 || (c >= 43785 && c <= 43790)))))
              : (c <= 43798 || (c < 43888
                ? (c < 43824
                  ? (c < 43816
                    ? (c >= 43808 && c <= 43814)
                    : c <= 43822)
                  : (c <= 43866 || (c >= 43868 && c <= 43881)))
                : (c <= 44002 || (c < 55243
                  ? (c < 55216
                    ? (c >= 44032 && c <= 55203)
                    : c <= 55238)
                  : (c <= 55291 || (c >= 63744 && c <= 64109)))))))))
          : (c <= 64217 || (c < 65147
            ? (c < 64326
              ? (c < 64298
                ? (c < 64285
                  ? (c < 64275
                    ? (c >= 64256 && c <= 64262)
                    : c <= 64279)
                  : (c <= 64285 || (c >= 64287 && c <= 64296)))
                : (c <= 64310 || (c < 64320
                  ? (c < 64318
                    ? (c >= 64312 && c <= 64316)
                    : c <= 64318)
                  : (c <= 64321 || (c >= 64323 && c <= 64324)))))
              : (c <= 64433 || (c < 65008
                ? (c < 64848
                  ? (c < 64612
                    ? (c >= 64467 && c <= 64605)
                    : c <= 64829)
                  : (c <= 64911 || (c >= 64914 && c <= 64967)))
                : (c <= 65017 || (c < 65143
                  ? (c < 65139
                    ? c == 65137
                    : c <= 65139)
                  : (c <= 65143 || c == 65145))))))
            : (c <= 65147 || (c < 65498
              ? (c < 65382
                ? (c < 65313
                  ? (c < 65151
                    ? c == 65149
                    : c <= 65276)
                  : (c <= 65338 || (c >= 65345 && c <= 65370)))
                : (c <= 65437 || (c < 65482
                  ? (c < 65474
                    ? (c >= 65440 && c <= 65470)
                    : c <= 65479)
                  : (c <= 65487 || (c >= 65490 && c <= 65495)))))
              : (c <= 65500 || (c < 65599
                ? (c < 65576
                  ? (c < 65549
                    ? (c >= 65536 && c <= 65547)
                    : c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))
                : (c <= 65613 || (c < 65664
                  ? (c >= 65616 && c <= 65629)
                  : (c <= 65786 || (c >= 65856 && c <= 65908)))))))))))
        : (c <= 66204 || (c < 68416
          ? (c < 67639
            ? (c < 66736
              ? (c < 66432
                ? (c < 66349
                  ? (c < 66304
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66335)
                  : (c <= 66378 || (c >= 66384 && c <= 66421)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c >= 66560 && c <= 66717)))))
              : (c <= 66771 || (c < 67392
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c >= 67072 && c <= 67382)))
                : (c <= 67413 || (c < 67592
                  ? (c < 67584
                    ? (c >= 67424 && c <= 67431)
                    : c <= 67589)
                  : (c <= 67592 || (c >= 67594 && c <= 67637)))))))
            : (c <= 67640 || (c < 68030
              ? (c < 67808
                ? (c < 67680
                  ? (c < 67647
                    ? c == 67644
                    : c <= 67669)
                  : (c <= 67702 || (c >= 67712 && c <= 67742)))
                : (c <= 67826 || (c < 67872
                  ? (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)
                  : (c <= 67897 || (c >= 67968 && c <= 68023)))))
              : (c <= 68031 || (c < 68192
                ? (c < 68117
                  ? (c < 68112
                    ? c == 68096
                    : c <= 68115)
                  : (c <= 68119 || (c >= 68121 && c <= 68149)))
                : (c <= 68220 || (c < 68297
                  ? (c < 68288
                    ? (c >= 68224 && c <= 68252)
                    : c <= 68295)
                  : (c <= 68324 || (c >= 68352 && c <= 68405)))))))))
          : (c <= 68437 || (c < 69968
            ? (c < 69415
              ? (c < 68800
                ? (c < 68608
                  ? (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)
                  : (c <= 68680 || (c >= 68736 && c <= 68786)))
                : (c <= 68850 || (c < 69296
                  ? (c < 69248
                    ? (c >= 68864 && c <= 68899)
                    : c <= 69289)
                  : (c <= 69297 || (c >= 69376 && c <= 69404)))))
              : (c <= 69415 || (c < 69763
                ? (c < 69600
                  ? (c < 69552
                    ? (c >= 69424 && c <= 69445)
                    : c <= 69572)
                  : (c <= 69622 || (c >= 69635 && c <= 69687)))
                : (c <= 69807 || (c < 69956
                  ? (c < 69891
                    ? (c >= 69840 && c <= 69864)
                    : c <= 69926)
                  : (c <= 69956 || c == 69959))))))
            : (c <= 70002 || (c < 70282
              ? (c < 70108
                ? (c < 70081
                  ? (c < 70019
                    ? c == 70006
                    : c <= 70066)
                  : (c <= 70084 || c == 70106))
                : (c <= 70108 || (c < 70272
                  ? (c < 70163
                    ? (c >= 70144 && c <= 70161)
                    : c <= 70187)
                  : (c <= 70278 || c == 70280))))
              : (c <= 70285 || (c < 70415
                ? (c < 70320
                  ? (c < 70303
                    ? (c >= 70287 && c <= 70301)
                    : c <= 70312)
                  : (c <= 70366 || (c >= 70405 && c <= 70412)))
                : (c <= 70416 || (c < 70442
                  ? (c >= 70419 && c <= 70440)
                  : (c <= 70448 || (c >= 70450 && c <= 70451)))))))))))))
      : (c <= 70457 || (c < 113808
        ? (c < 72818
          ? (c < 71945
            ? (c < 71040
              ? (c < 70727
                ? (c < 70493
                  ? (c < 70480
                    ? c == 70461
                    : c <= 70480)
                  : (c <= 70497 || (c >= 70656 && c <= 70708)))
                : (c <= 70730 || (c < 70852
                  ? (c < 70784
                    ? (c >= 70751 && c <= 70753)
                    : c <= 70831)
                  : (c <= 70853 || c == 70855))))
              : (c <= 71086 || (c < 71352
                ? (c < 71236
                  ? (c < 71168
                    ? (c >= 71128 && c <= 71131)
                    : c <= 71215)
                  : (c <= 71236 || (c >= 71296 && c <= 71338)))
                : (c <= 71352 || (c < 71840
                  ? (c < 71680
                    ? (c >= 71424 && c <= 71450)
                    : c <= 71723)
                  : (c <= 71903 || (c >= 71935 && c <= 71942)))))))
            : (c <= 71945 || (c < 72192
              ? (c < 72001
                ? (c < 71960
                  ? (c < 71957
                    ? (c >= 71948 && c <= 71955)
                    : c <= 71958)
                  : (c <= 71983 || c == 71999))
                : (c <= 72001 || (c < 72161
                  ? (c < 72106
                    ? (c >= 72096 && c <= 72103)
                    : c <= 72144)
                  : (c <= 72161 || c == 72163))))
              : (c <= 72192 || (c < 72349
                ? (c < 72272
                  ? (c < 72250
                    ? (c >= 72203 && c <= 72242)
                    : c <= 72250)
                  : (c <= 72272 || (c >= 72284 && c <= 72329)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72384 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72750 || c == 72768))))))))
          : (c <= 72847 || (c < 92992
            ? (c < 73648
              ? (c < 73056
                ? (c < 72971
                  ? (c < 72968
                    ? (c >= 72960 && c <= 72966)
                    : c <= 72969)
                  : (c <= 73008 || c == 73030))
                : (c <= 73061 || (c < 73112
                  ? (c < 73066
                    ? (c >= 73063 && c <= 73064)
                    : c <= 73097)
                  : (c <= 73112 || (c >= 73440 && c <= 73458)))))
              : (c <= 73648 || (c < 82944
                ? (c < 74880
                  ? (c < 74752
                    ? (c >= 73728 && c <= 74649)
                    : c <= 74862)
                  : (c <= 75075 || (c >= 77824 && c <= 78894)))
                : (c <= 83526 || (c < 92880
                  ? (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)
                  : (c <= 92909 || (c >= 92928 && c <= 92975)))))))
            : (c <= 92995 || (c < 100352
              ? (c < 94032
                ? (c < 93760
                  ? (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)
                  : (c <= 93823 || (c >= 93952 && c <= 94026)))
                : (c <= 94032 || (c < 94179
                  ? (c < 94176
                    ? (c >= 94099 && c <= 94111)
                    : c <= 94177)
                  : (c <= 94179 || (c >= 94208 && c <= 100343)))))
              : (c <= 101589 || (c < 110960
                ? (c < 110928
                  ? (c < 110592
                    ? (c >= 101632 && c <= 101640)
                    : c <= 110878)
                  : (c <= 110930 || (c >= 110948 && c <= 110951)))
                : (c <= 111355 || (c < 113776
                  ? (c >= 113664 && c <= 113770)
                  : (c <= 113788 || (c >= 113792 && c <= 113800)))))))))))
        : (c <= 113817 || (c < 126469
          ? (c < 120488
            ? (c < 120005
              ? (c < 119973
                ? (c < 119966
                  ? (c < 119894
                    ? (c >= 119808 && c <= 119892)
                    : c <= 119964)
                  : (c <= 119967 || c == 119970))
                : (c <= 119974 || (c < 119995
                  ? (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)
                  : (c <= 119995 || (c >= 119997 && c <= 120003)))))
              : (c <= 120069 || (c < 120123
                ? (c < 120086
                  ? (c < 120077
                    ? (c >= 120071 && c <= 120074)
                    : c <= 120084)
                  : (c <= 120092 || (c >= 120094 && c <= 120121)))
                : (c <= 120126 || (c < 120138
                  ? (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)
                  : (c <= 120144 || (c >= 120146 && c <= 120485)))))))
            : (c <= 120512 || (c < 120772
              ? (c < 120630
                ? (c < 120572
                  ? (c < 120540
                    ? (c >= 120514 && c <= 120538)
                    : c <= 120570)
                  : (c <= 120596 || (c >= 120598 && c <= 120628)))
                : (c <= 120654 || (c < 120714
                  ? (c < 120688
                    ? (c >= 120656 && c <= 120686)
                    : c <= 120712)
                  : (c <= 120744 || (c >= 120746 && c <= 120770)))))
              : (c <= 120779 || (c < 124928
                ? (c < 123214
                  ? (c < 123191
                    ? (c >= 123136 && c <= 123180)
                    : c <= 123197)
                  : (c <= 123214 || (c >= 123584 && c <= 123627)))
                : (c <= 125124 || (c < 125259
                  ? (c >= 125184 && c <= 125251)
                  : (c <= 125259 || (c >= 126464 && c <= 126467)))))))))
          : (c <= 126495 || (c < 126561
            ? (c < 126537
              ? (c < 126516
                ? (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c >= 126505 && c <= 126514)))
                : (c <= 126519 || (c < 126530
                  ? (c < 126523
                    ? c == 126521
                    : c <= 126523)
                  : (c <= 126530 || c == 126535))))
              : (c <= 126537 || (c < 126551
                ? (c < 126545
                  ? (c < 126541
                    ? c == 126539
                    : c <= 126543)
                  : (c <= 126546 || c == 126548))
                : (c <= 126551 || (c < 126557
                  ? (c < 126555
                    ? c == 126553
                    : c <= 126555)
                  : (c <= 126557 || c == 126559))))))
            : (c <= 126562 || (c < 126629
              ? (c < 126585
                ? (c < 126572
                  ? (c < 126567
                    ? c == 126564
                    : c <= 126570)
                  : (c <= 126578 || (c >= 126580 && c <= 126583)))
                : (c <= 126588 || (c < 126603
                  ? (c < 126592
                    ? c == 126590
                    : c <= 126601)
                  : (c <= 126619 || (c >= 126625 && c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 173824
                  ? (c < 131072
                    ? (c >= 126635 && c <= 126651)
                    : c <= 173789)
                  : (c <= 177972 || (c >= 177984 && c <= 178205)))
                : (c <= 183969 || (c < 194560
                  ? (c >= 183984 && c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static inline bool sym_id_character_set_2(int32_t c) {
  return (c < 43020
    ? (c < 3976
      ? (c < 2674
        ? (c < 1869
          ? (c < 908
            ? (c < 710
              ? (c < 181
                ? (c < 'a'
                  ? (c < '_'
                    ? (c >= 'A' && c <= 'Z')
                    : c <= '_')
                  : (c <= 'z' || c == 170))
                : (c <= 181 || (c < 216
                  ? (c < 192
                    ? c == 186
                    : c <= 214)
                  : (c <= 246 || (c >= 248 && c <= 705)))))
              : (c <= 721 || (c < 886
                ? (c < 750
                  ? (c < 748
                    ? (c >= 736 && c <= 740)
                    : c <= 748)
                  : (c <= 750 || (c >= 880 && c <= 884)))
                : (c <= 887 || (c < 902
                  ? (c < 895
                    ? (c >= 891 && c <= 893)
                    : c <= 895)
                  : (c <= 902 || (c >= 904 && c <= 906)))))))
            : (c <= 908 || (c < 1568
              ? (c < 1329
                ? (c < 1015
                  ? (c < 931
                    ? (c >= 910 && c <= 929)
                    : c <= 1013)
                  : (c <= 1153 || (c >= 1162 && c <= 1327)))
                : (c <= 1366 || (c < 1488
                  ? (c < 1376
                    ? c == 1369
                    : c <= 1416)
                  : (c <= 1514 || (c >= 1519 && c <= 1522)))))
              : (c <= 1610 || (c < 1774
                ? (c < 1749
                  ? (c < 1649
                    ? (c >= 1646 && c <= 1647)
                    : c <= 1747)
                  : (c <= 1749 || (c >= 1765 && c <= 1766)))
                : (c <= 1775 || (c < 1808
                  ? (c < 1791
                    ? (c >= 1786 && c <= 1788)
                    : c <= 1791)
                  : (c <= 1808 || (c >= 1810 && c <= 1839)))))))))
          : (c <= 1957 || (c < 2451
            ? (c < 2144
              ? (c < 2048
                ? (c < 2036
                  ? (c < 1994
                    ? c == 1969
                    : c <= 2026)
                  : (c <= 2037 || c == 2042))
                : (c <= 2069 || (c < 2088
                  ? (c < 2084
                    ? c == 2074
                    : c <= 2084)
                  : (c <= 2088 || (c >= 2112 && c <= 2136)))))
              : (c <= 2154 || (c < 2384
                ? (c < 2308
                  ? (c < 2230
                    ? (c >= 2208 && c <= 2228)
                    : c <= 2247)
                  : (c <= 2361 || c == 2365))
                : (c <= 2384 || (c < 2437
                  ? (c < 2417
                    ? (c >= 2392 && c <= 2401)
                    : c <= 2432)
                  : (c <= 2444 || (c >= 2447 && c <= 2448)))))))
            : (c <= 2472 || (c < 2565
              ? (c < 2510
                ? (c < 2486
                  ? (c < 2482
                    ? (c >= 2474 && c <= 2480)
                    : c <= 2482)
                  : (c <= 2489 || c == 2493))
                : (c <= 2510 || (c < 2544
                  ? (c < 2527
                    ? (c >= 2524 && c <= 2525)
                    : c <= 2529)
                  : (c <= 2545 || c == 2556))))
              : (c <= 2570 || (c < 2613
                ? (c < 2602
                  ? (c < 2579
                    ? (c >= 2575 && c <= 2576)
                    : c <= 2600)
                  : (c <= 2608 || (c >= 2610 && c <= 2611)))
                : (c <= 2614 || (c < 2649
                  ? (c >= 2616 && c <= 2617)
                  : (c <= 2652 || c == 2654))))))))))
        : (c <= 2676 || (c < 3205
          ? (c < 2929
            ? (c < 2809
              ? (c < 2738
                ? (c < 2707
                  ? (c < 2703
                    ? (c >= 2693 && c <= 2701)
                    : c <= 2705)
                  : (c <= 2728 || (c >= 2730 && c <= 2736)))
                : (c <= 2739 || (c < 2768
                  ? (c < 2749
                    ? (c >= 2741 && c <= 2745)
                    : c <= 2749)
                  : (c <= 2768 || (c >= 2784 && c <= 2785)))))
              : (c <= 2809 || (c < 2866
                ? (c < 2835
                  ? (c < 2831
                    ? (c >= 2821 && c <= 2828)
                    : c <= 2832)
                  : (c <= 2856 || (c >= 2858 && c <= 2864)))
                : (c <= 2867 || (c < 2908
                  ? (c < 2877
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2877)
                  : (c <= 2909 || (c >= 2911 && c <= 2913)))))))
            : (c <= 2929 || (c < 2990
              ? (c < 2969
                ? (c < 2958
                  ? (c < 2949
                    ? c == 2947
                    : c <= 2954)
                  : (c <= 2960 || (c >= 2962 && c <= 2965)))
                : (c <= 2970 || (c < 2979
                  ? (c < 2974
                    ? c == 2972
                    : c <= 2975)
                  : (c <= 2980 || (c >= 2984 && c <= 2986)))))
              : (c <= 3001 || (c < 3114
                ? (c < 3086
                  ? (c < 3077
                    ? c == 3024
                    : c <= 3084)
                  : (c <= 3088 || (c >= 3090 && c <= 3112)))
                : (c <= 3129 || (c < 3168
                  ? (c < 3160
                    ? c == 3133
                    : c <= 3162)
                  : (c <= 3169 || c == 3200))))))))
          : (c <= 3212 || (c < 3517
            ? (c < 3342
              ? (c < 3261
                ? (c < 3242
                  ? (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)
                  : (c <= 3251 || (c >= 3253 && c <= 3257)))
                : (c <= 3261 || (c < 3313
                  ? (c < 3296
                    ? c == 3294
                    : c <= 3297)
                  : (c <= 3314 || (c >= 3332 && c <= 3340)))))
              : (c <= 3344 || (c < 3423
                ? (c < 3406
                  ? (c < 3389
                    ? (c >= 3346 && c <= 3386)
                    : c <= 3389)
                  : (c <= 3406 || (c >= 3412 && c <= 3414)))
                : (c <= 3425 || (c < 3482
                  ? (c < 3461
                    ? (c >= 3450 && c <= 3455)
                    : c <= 3478)
                  : (c <= 3505 || (c >= 3507 && c <= 3515)))))))
            : (c <= 3517 || (c < 3751
              ? (c < 3713
                ? (c < 3634
                  ? (c < 3585
                    ? (c >= 3520 && c <= 3526)
                    : c <= 3632)
                  : (c <= 3634 || (c >= 3648 && c <= 3654)))
                : (c <= 3714 || (c < 3724
                  ? (c < 3718
                    ? c == 3716
                    : c <= 3722)
                  : (c <= 3747 || c == 3749))))
              : (c <= 3760 || (c < 3804
                ? (c < 3776
                  ? (c < 3773
                    ? c == 3762
                    : c <= 3773)
                  : (c <= 3780 || c == 3782))
                : (c <= 3807 || (c < 3904
                  ? c == 3840
                  : (c <= 3911 || (c >= 3913 && c <= 3948)))))))))))))
      : (c <= 3980 || (c < 8016
        ? (c < 5920
          ? (c < 4746
            ? (c < 4256
              ? (c < 4193
                ? (c < 4176
                  ? (c < 4159
                    ? (c >= 4096 && c <= 4138)
                    : c <= 4159)
                  : (c <= 4181 || (c >= 4186 && c <= 4189)))
                : (c <= 4193 || (c < 4213
                  ? (c < 4206
                    ? (c >= 4197 && c <= 4198)
                    : c <= 4208)
                  : (c <= 4225 || c == 4238))))
              : (c <= 4293 || (c < 4682
                ? (c < 4304
                  ? (c < 4301
                    ? c == 4295
                    : c <= 4301)
                  : (c <= 4346 || (c >= 4348 && c <= 4680)))
                : (c <= 4685 || (c < 4698
                  ? (c < 4696
                    ? (c >= 4688 && c <= 4694)
                    : c <= 4696)
                  : (c <= 4701 || (c >= 4704 && c <= 4744)))))))
            : (c <= 4749 || (c < 4992
              ? (c < 4802
                ? (c < 4792
                  ? (c < 4786
                    ? (c >= 4752 && c <= 4784)
                    : c <= 4789)
                  : (c <= 4798 || c == 4800))
                : (c <= 4805 || (c < 4882
                  ? (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)
                  : (c <= 4885 || (c >= 4888 && c <= 4954)))))
              : (c <= 5007 || (c < 5761
                ? (c < 5121
                  ? (c < 5112
                    ? (c >= 5024 && c <= 5109)
                    : c <= 5117)
                  : (c <= 5740 || (c >= 5743 && c <= 5759)))
                : (c <= 5786 || (c < 5888
                  ? (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)
                  : (c <= 5900 || (c >= 5902 && c <= 5905)))))))))
          : (c <= 5937 || (c < 6981
            ? (c < 6320
              ? (c < 6103
                ? (c < 5998
                  ? (c < 5984
                    ? (c >= 5952 && c <= 5969)
                    : c <= 5996)
                  : (c <= 6000 || (c >= 6016 && c <= 6067)))
                : (c <= 6103 || (c < 6272
                  ? (c < 6176
                    ? c == 6108
                    : c <= 6264)
                  : (c <= 6312 || c == 6314))))
              : (c <= 6389 || (c < 6576
                ? (c < 6512
                  ? (c < 6480
                    ? (c >= 6400 && c <= 6430)
                    : c <= 6509)
                  : (c <= 6516 || (c >= 6528 && c <= 6571)))
                : (c <= 6601 || (c < 6823
                  ? (c < 6688
                    ? (c >= 6656 && c <= 6678)
                    : c <= 6740)
                  : (c <= 6823 || (c >= 6917 && c <= 6963)))))))
            : (c <= 6987 || (c < 7401
              ? (c < 7245
                ? (c < 7098
                  ? (c < 7086
                    ? (c >= 7043 && c <= 7072)
                    : c <= 7087)
                  : (c <= 7141 || (c >= 7168 && c <= 7203)))
                : (c <= 7247 || (c < 7312
                  ? (c < 7296
                    ? (c >= 7258 && c <= 7293)
                    : c <= 7304)
                  : (c <= 7354 || (c >= 7357 && c <= 7359)))))
              : (c <= 7404 || (c < 7680
                ? (c < 7418
                  ? (c < 7413
                    ? (c >= 7406 && c <= 7411)
                    : c <= 7414)
                  : (c <= 7418 || (c >= 7424 && c <= 7615)))
                : (c <= 7957 || (c < 7968
                  ? (c >= 7960 && c <= 7965)
                  : (c <= 8005 || (c >= 8008 && c <= 8013)))))))))))
        : (c <= 8023 || (c < 11631
          ? (c < 8458
            ? (c < 8144
              ? (c < 8064
                ? (c < 8029
                  ? (c < 8027
                    ? c == 8025
                    : c <= 8027)
                  : (c <= 8029 || (c >= 8031 && c <= 8061)))
                : (c <= 8116 || (c < 8130
                  ? (c < 8126
                    ? (c >= 8118 && c <= 8124)
                    : c <= 8126)
                  : (c <= 8132 || (c >= 8134 && c <= 8140)))))
              : (c <= 8147 || (c < 8305
                ? (c < 8178
                  ? (c < 8160
                    ? (c >= 8150 && c <= 8155)
                    : c <= 8172)
                  : (c <= 8180 || (c >= 8182 && c <= 8188)))
                : (c <= 8305 || (c < 8450
                  ? (c < 8336
                    ? c == 8319
                    : c <= 8348)
                  : (c <= 8450 || c == 8455))))))
            : (c <= 8467 || (c < 8544
              ? (c < 8488
                ? (c < 8484
                  ? (c < 8472
                    ? c == 8469
                    : c <= 8477)
                  : (c <= 8484 || c == 8486))
                : (c <= 8488 || (c < 8517
                  ? (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)
                  : (c <= 8521 || c == 8526))))
              : (c <= 8584 || (c < 11506
                ? (c < 11360
                  ? (c < 11312
                    ? (c >= 11264 && c <= 11310)
                    : c <= 11358)
                  : (c <= 11492 || (c >= 11499 && c <= 11502)))
                : (c <= 11507 || (c < 11565
                  ? (c < 11559
                    ? (c >= 11520 && c <= 11557)
                    : c <= 11559)
                  : (c <= 11565 || (c >= 11568 && c <= 11623)))))))))
          : (c <= 11631 || (c < 12704
            ? (c < 12293
              ? (c < 11704
                ? (c < 11688
                  ? (c < 11680
                    ? (c >= 11648 && c <= 11670)
                    : c <= 11686)
                  : (c <= 11694 || (c >= 11696 && c <= 11702)))
                : (c <= 11710 || (c < 11728
                  ? (c < 11720
                    ? (c >= 11712 && c <= 11718)
                    : c <= 11726)
                  : (c <= 11734 || (c >= 11736 && c <= 11742)))))
              : (c <= 12295 || (c < 12445
                ? (c < 12344
                  ? (c < 12337
                    ? (c >= 12321 && c <= 12329)
                    : c <= 12341)
                  : (c <= 12348 || (c >= 12353 && c <= 12438)))
                : (c <= 12447 || (c < 12549
                  ? (c < 12540
                    ? (c >= 12449 && c <= 12538)
                    : c <= 12543)
                  : (c <= 12591 || (c >= 12593 && c <= 12686)))))))
            : (c <= 12735 || (c < 42623
              ? (c < 42192
                ? (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 40956 || (c >= 40960 && c <= 42124)))
                : (c <= 42237 || (c < 42538
                  ? (c < 42512
                    ? (c >= 42240 && c <= 42508)
                    : c <= 42527)
                  : (c <= 42539 || (c >= 42560 && c <= 42606)))))
              : (c <= 42653 || (c < 42946
                ? (c < 42786
                  ? (c < 42775
                    ? (c >= 42656 && c <= 42735)
                    : c <= 42783)
                  : (c <= 42888 || (c >= 42891 && c <= 42943)))
                : (c <= 42954 || (c < 43011
                  ? (c >= 42997 && c <= 43009)
                  : (c <= 43013 || (c >= 43015 && c <= 43018)))))))))))))))
    : (c <= 43042 || (c < 70453
      ? (c < 66176
        ? (c < 64112
          ? (c < 43697
            ? (c < 43471
              ? (c < 43261
                ? (c < 43250
                  ? (c < 43138
                    ? (c >= 43072 && c <= 43123)
                    : c <= 43187)
                  : (c <= 43255 || c == 43259))
                : (c <= 43262 || (c < 43360
                  ? (c < 43312
                    ? (c >= 43274 && c <= 43301)
                    : c <= 43334)
                  : (c <= 43388 || (c >= 43396 && c <= 43442)))))
              : (c <= 43471 || (c < 43584
                ? (c < 43514
                  ? (c < 43494
                    ? (c >= 43488 && c <= 43492)
                    : c <= 43503)
                  : (c <= 43518 || (c >= 43520 && c <= 43560)))
                : (c <= 43586 || (c < 43642
                  ? (c < 43616
                    ? (c >= 43588 && c <= 43595)
                    : c <= 43638)
                  : (c <= 43642 || (c >= 43646 && c <= 43695)))))))
            : (c <= 43697 || (c < 43793
              ? (c < 43739
                ? (c < 43712
                  ? (c < 43705
                    ? (c >= 43701 && c <= 43702)
                    : c <= 43709)
                  : (c <= 43712 || c == 43714))
                : (c <= 43741 || (c < 43777
                  ? (c < 43762
                    ? (c >= 43744 && c <= 43754)
                    : c <= 43764)
                  : (c <= 43782 || (c >= 43785 && c <= 43790)))))
              : (c <= 43798 || (c < 43888
                ? (c < 43824
                  ? (c < 43816
                    ? (c >= 43808 && c <= 43814)
                    : c <= 43822)
                  : (c <= 43866 || (c >= 43868 && c <= 43881)))
                : (c <= 44002 || (c < 55243
                  ? (c < 55216
                    ? (c >= 44032 && c <= 55203)
                    : c <= 55238)
                  : (c <= 55291 || (c >= 63744 && c <= 64109)))))))))
          : (c <= 64217 || (c < 65147
            ? (c < 64326
              ? (c < 64298
                ? (c < 64285
                  ? (c < 64275
                    ? (c >= 64256 && c <= 64262)
                    : c <= 64279)
                  : (c <= 64285 || (c >= 64287 && c <= 64296)))
                : (c <= 64310 || (c < 64320
                  ? (c < 64318
                    ? (c >= 64312 && c <= 64316)
                    : c <= 64318)
                  : (c <= 64321 || (c >= 64323 && c <= 64324)))))
              : (c <= 64433 || (c < 65008
                ? (c < 64848
                  ? (c < 64612
                    ? (c >= 64467 && c <= 64605)
                    : c <= 64829)
                  : (c <= 64911 || (c >= 64914 && c <= 64967)))
                : (c <= 65017 || (c < 65143
                  ? (c < 65139
                    ? c == 65137
                    : c <= 65139)
                  : (c <= 65143 || c == 65145))))))
            : (c <= 65147 || (c < 65498
              ? (c < 65382
                ? (c < 65313
                  ? (c < 65151
                    ? c == 65149
                    : c <= 65276)
                  : (c <= 65338 || (c >= 65345 && c <= 65370)))
                : (c <= 65437 || (c < 65482
                  ? (c < 65474
                    ? (c >= 65440 && c <= 65470)
                    : c <= 65479)
                  : (c <= 65487 || (c >= 65490 && c <= 65495)))))
              : (c <= 65500 || (c < 65599
                ? (c < 65576
                  ? (c < 65549
                    ? (c >= 65536 && c <= 65547)
                    : c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))
                : (c <= 65613 || (c < 65664
                  ? (c >= 65616 && c <= 65629)
                  : (c <= 65786 || (c >= 65856 && c <= 65908)))))))))))
        : (c <= 66204 || (c < 68416
          ? (c < 67639
            ? (c < 66736
              ? (c < 66432
                ? (c < 66349
                  ? (c < 66304
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66335)
                  : (c <= 66378 || (c >= 66384 && c <= 66421)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c >= 66560 && c <= 66717)))))
              : (c <= 66771 || (c < 67392
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c >= 67072 && c <= 67382)))
                : (c <= 67413 || (c < 67592
                  ? (c < 67584
                    ? (c >= 67424 && c <= 67431)
                    : c <= 67589)
                  : (c <= 67592 || (c >= 67594 && c <= 67637)))))))
            : (c <= 67640 || (c < 68030
              ? (c < 67808
                ? (c < 67680
                  ? (c < 67647
                    ? c == 67644
                    : c <= 67669)
                  : (c <= 67702 || (c >= 67712 && c <= 67742)))
                : (c <= 67826 || (c < 67872
                  ? (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)
                  : (c <= 67897 || (c >= 67968 && c <= 68023)))))
              : (c <= 68031 || (c < 68192
                ? (c < 68117
                  ? (c < 68112
                    ? c == 68096
                    : c <= 68115)
                  : (c <= 68119 || (c >= 68121 && c <= 68149)))
                : (c <= 68220 || (c < 68297
                  ? (c < 68288
                    ? (c >= 68224 && c <= 68252)
                    : c <= 68295)
                  : (c <= 68324 || (c >= 68352 && c <= 68405)))))))))
          : (c <= 68437 || (c < 69968
            ? (c < 69415
              ? (c < 68800
                ? (c < 68608
                  ? (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)
                  : (c <= 68680 || (c >= 68736 && c <= 68786)))
                : (c <= 68850 || (c < 69296
                  ? (c < 69248
                    ? (c >= 68864 && c <= 68899)
                    : c <= 69289)
                  : (c <= 69297 || (c >= 69376 && c <= 69404)))))
              : (c <= 69415 || (c < 69763
                ? (c < 69600
                  ? (c < 69552
                    ? (c >= 69424 && c <= 69445)
                    : c <= 69572)
                  : (c <= 69622 || (c >= 69635 && c <= 69687)))
                : (c <= 69807 || (c < 69956
                  ? (c < 69891
                    ? (c >= 69840 && c <= 69864)
                    : c <= 69926)
                  : (c <= 69956 || c == 69959))))))
            : (c <= 70002 || (c < 70282
              ? (c < 70108
                ? (c < 70081
                  ? (c < 70019
                    ? c == 70006
                    : c <= 70066)
                  : (c <= 70084 || c == 70106))
                : (c <= 70108 || (c < 70272
                  ? (c < 70163
                    ? (c >= 70144 && c <= 70161)
                    : c <= 70187)
                  : (c <= 70278 || c == 70280))))
              : (c <= 70285 || (c < 70415
                ? (c < 70320
                  ? (c < 70303
                    ? (c >= 70287 && c <= 70301)
                    : c <= 70312)
                  : (c <= 70366 || (c >= 70405 && c <= 70412)))
                : (c <= 70416 || (c < 70442
                  ? (c >= 70419 && c <= 70440)
                  : (c <= 70448 || (c >= 70450 && c <= 70451)))))))))))))
      : (c <= 70457 || (c < 113808
        ? (c < 72818
          ? (c < 71945
            ? (c < 71040
              ? (c < 70727
                ? (c < 70493
                  ? (c < 70480
                    ? c == 70461
                    : c <= 70480)
                  : (c <= 70497 || (c >= 70656 && c <= 70708)))
                : (c <= 70730 || (c < 70852
                  ? (c < 70784
                    ? (c >= 70751 && c <= 70753)
                    : c <= 70831)
                  : (c <= 70853 || c == 70855))))
              : (c <= 71086 || (c < 71352
                ? (c < 71236
                  ? (c < 71168
                    ? (c >= 71128 && c <= 71131)
                    : c <= 71215)
                  : (c <= 71236 || (c >= 71296 && c <= 71338)))
                : (c <= 71352 || (c < 71840
                  ? (c < 71680
                    ? (c >= 71424 && c <= 71450)
                    : c <= 71723)
                  : (c <= 71903 || (c >= 71935 && c <= 71942)))))))
            : (c <= 71945 || (c < 72192
              ? (c < 72001
                ? (c < 71960
                  ? (c < 71957
                    ? (c >= 71948 && c <= 71955)
                    : c <= 71958)
                  : (c <= 71983 || c == 71999))
                : (c <= 72001 || (c < 72161
                  ? (c < 72106
                    ? (c >= 72096 && c <= 72103)
                    : c <= 72144)
                  : (c <= 72161 || c == 72163))))
              : (c <= 72192 || (c < 72349
                ? (c < 72272
                  ? (c < 72250
                    ? (c >= 72203 && c <= 72242)
                    : c <= 72250)
                  : (c <= 72272 || (c >= 72284 && c <= 72329)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72384 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72750 || c == 72768))))))))
          : (c <= 72847 || (c < 92992
            ? (c < 73648
              ? (c < 73056
                ? (c < 72971
                  ? (c < 72968
                    ? (c >= 72960 && c <= 72966)
                    : c <= 72969)
                  : (c <= 73008 || c == 73030))
                : (c <= 73061 || (c < 73112
                  ? (c < 73066
                    ? (c >= 73063 && c <= 73064)
                    : c <= 73097)
                  : (c <= 73112 || (c >= 73440 && c <= 73458)))))
              : (c <= 73648 || (c < 82944
                ? (c < 74880
                  ? (c < 74752
                    ? (c >= 73728 && c <= 74649)
                    : c <= 74862)
                  : (c <= 75075 || (c >= 77824 && c <= 78894)))
                : (c <= 83526 || (c < 92880
                  ? (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)
                  : (c <= 92909 || (c >= 92928 && c <= 92975)))))))
            : (c <= 92995 || (c < 100352
              ? (c < 94032
                ? (c < 93760
                  ? (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)
                  : (c <= 93823 || (c >= 93952 && c <= 94026)))
                : (c <= 94032 || (c < 94179
                  ? (c < 94176
                    ? (c >= 94099 && c <= 94111)
                    : c <= 94177)
                  : (c <= 94179 || (c >= 94208 && c <= 100343)))))
              : (c <= 101589 || (c < 110960
                ? (c < 110928
                  ? (c < 110592
                    ? (c >= 101632 && c <= 101640)
                    : c <= 110878)
                  : (c <= 110930 || (c >= 110948 && c <= 110951)))
                : (c <= 111355 || (c < 113776
                  ? (c >= 113664 && c <= 113770)
                  : (c <= 113788 || (c >= 113792 && c <= 113800)))))))))))
        : (c <= 113817 || (c < 126469
          ? (c < 120488
            ? (c < 120005
              ? (c < 119973
                ? (c < 119966
                  ? (c < 119894
                    ? (c >= 119808 && c <= 119892)
                    : c <= 119964)
                  : (c <= 119967 || c == 119970))
                : (c <= 119974 || (c < 119995
                  ? (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)
                  : (c <= 119995 || (c >= 119997 && c <= 120003)))))
              : (c <= 120069 || (c < 120123
                ? (c < 120086
                  ? (c < 120077
                    ? (c >= 120071 && c <= 120074)
                    : c <= 120084)
                  : (c <= 120092 || (c >= 120094 && c <= 120121)))
                : (c <= 120126 || (c < 120138
                  ? (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)
                  : (c <= 120144 || (c >= 120146 && c <= 120485)))))))
            : (c <= 120512 || (c < 120772
              ? (c < 120630
                ? (c < 120572
                  ? (c < 120540
                    ? (c >= 120514 && c <= 120538)
                    : c <= 120570)
                  : (c <= 120596 || (c >= 120598 && c <= 120628)))
                : (c <= 120654 || (c < 120714
                  ? (c < 120688
                    ? (c >= 120656 && c <= 120686)
                    : c <= 120712)
                  : (c <= 120744 || (c >= 120746 && c <= 120770)))))
              : (c <= 120779 || (c < 124928
                ? (c < 123214
                  ? (c < 123191
                    ? (c >= 123136 && c <= 123180)
                    : c <= 123197)
                  : (c <= 123214 || (c >= 123584 && c <= 123627)))
                : (c <= 125124 || (c < 125259
                  ? (c >= 125184 && c <= 125251)
                  : (c <= 125259 || (c >= 126464 && c <= 126467)))))))))
          : (c <= 126495 || (c < 126561
            ? (c < 126537
              ? (c < 126516
                ? (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c >= 126505 && c <= 126514)))
                : (c <= 126519 || (c < 126530
                  ? (c < 126523
                    ? c == 126521
                    : c <= 126523)
                  : (c <= 126530 || c == 126535))))
              : (c <= 126537 || (c < 126551
                ? (c < 126545
                  ? (c < 126541
                    ? c == 126539
                    : c <= 126543)
                  : (c <= 126546 || c == 126548))
                : (c <= 126551 || (c < 126557
                  ? (c < 126555
                    ? c == 126553
                    : c <= 126555)
                  : (c <= 126557 || c == 126559))))))
            : (c <= 126562 || (c < 126629
              ? (c < 126585
                ? (c < 126572
                  ? (c < 126567
                    ? c == 126564
                    : c <= 126570)
                  : (c <= 126578 || (c >= 126580 && c <= 126583)))
                : (c <= 126588 || (c < 126603
                  ? (c < 126592
                    ? c == 126590
                    : c <= 126601)
                  : (c <= 126619 || (c >= 126625 && c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 173824
                  ? (c < 131072
                    ? (c >= 126635 && c <= 126651)
                    : c <= 173789)
                  : (c <= 177972 || (c >= 177984 && c <= 178205)))
                : (c <= 183969 || (c < 194560
                  ? (c >= 183984 && c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static inline bool sym_id_character_set_3(int32_t c) {
  return (c < 43052
    ? (c < 3718
      ? (c < 2730
        ? (c < 2042
          ? (c < 1015
            ? (c < 710
              ? (c < 181
                ? (c < '_'
                  ? (c < 'A'
                    ? (c >= '0' && c <= '9')
                    : c <= 'Z')
                  : (c <= '_' || (c < 170
                    ? (c >= 'a' && c <= 'z')
                    : c <= 170)))
                : (c <= 181 || (c < 192
                  ? (c < 186
                    ? c == 183
                    : c <= 186)
                  : (c <= 214 || (c < 248
                    ? (c >= 216 && c <= 246)
                    : c <= 705)))))
              : (c <= 721 || (c < 891
                ? (c < 750
                  ? (c < 748
                    ? (c >= 736 && c <= 740)
                    : c <= 748)
                  : (c <= 750 || (c < 886
                    ? (c >= 768 && c <= 884)
                    : c <= 887)))
                : (c <= 893 || (c < 908
                  ? (c < 902
                    ? c == 895
                    : c <= 906)
                  : (c <= 908 || (c < 931
                    ? (c >= 910 && c <= 929)
                    : c <= 1013)))))))
            : (c <= 1153 || (c < 1519
              ? (c < 1425
                ? (c < 1329
                  ? (c < 1162
                    ? (c >= 1155 && c <= 1159)
                    : c <= 1327)
                  : (c <= 1366 || (c < 1376
                    ? c == 1369
                    : c <= 1416)))
                : (c <= 1469 || (c < 1476
                  ? (c < 1473
                    ? c == 1471
                    : c <= 1474)
                  : (c <= 1477 || (c < 1488
                    ? c == 1479
                    : c <= 1514)))))
              : (c <= 1522 || (c < 1770
                ? (c < 1646
                  ? (c < 1568
                    ? (c >= 1552 && c <= 1562)
                    : c <= 1641)
                  : (c <= 1747 || (c < 1759
                    ? (c >= 1749 && c <= 1756)
                    : c <= 1768)))
                : (c <= 1788 || (c < 1869
                  ? (c < 1808
                    ? c == 1791
                    : c <= 1866)
                  : (c <= 1969 || (c >= 1984 && c <= 2037)))))))))
          : (c <= 2042 || (c < 2534
            ? (c < 2447
              ? (c < 2230
                ? (c < 2112
                  ? (c < 2048
                    ? c == 2045
                    : c <= 2093)
                  : (c <= 2139 || (c < 2208
                    ? (c >= 2144 && c <= 2154)
                    : c <= 2228)))
                : (c <= 2247 || (c < 2406
                  ? (c < 2275
                    ? (c >= 2259 && c <= 2273)
                    : c <= 2403)
                  : (c <= 2415 || (c < 2437
                    ? (c >= 2417 && c <= 2435)
                    : c <= 2444)))))
              : (c <= 2448 || (c < 2503
                ? (c < 2482
                  ? (c < 2474
                    ? (c >= 2451 && c <= 2472)
                    : c <= 2480)
                  : (c <= 2482 || (c < 2492
                    ? (c >= 2486 && c <= 2489)
                    : c <= 2500)))
                : (c <= 2504 || (c < 2524
                  ? (c < 2519
                    ? (c >= 2507 && c <= 2510)
                    : c <= 2519)
                  : (c <= 2525 || (c >= 2527 && c <= 2531)))))))
            : (c <= 2545 || (c < 2622
              ? (c < 2579
                ? (c < 2561
                  ? (c < 2558
                    ? c == 2556
                    : c <= 2558)
                  : (c <= 2563 || (c < 2575
                    ? (c >= 2565 && c <= 2570)
                    : c <= 2576)))
                : (c <= 2600 || (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c < 2620
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2620)))))
              : (c <= 2626 || (c < 2662
                ? (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2637)
                  : (c <= 2641 || (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)))
                : (c <= 2677 || (c < 2703
                  ? (c < 2693
                    ? (c >= 2689 && c <= 2691)
                    : c <= 2701)
                  : (c <= 2705 || (c >= 2707 && c <= 2728)))))))))))
        : (c <= 2736 || (c < 3142
          ? (c < 2918
            ? (c < 2831
              ? (c < 2768
                ? (c < 2748
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)))
                : (c <= 2768 || (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2815 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))))
              : (c <= 2832 || (c < 2887
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))
                : (c <= 2888 || (c < 2908
                  ? (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)
                  : (c <= 2909 || (c >= 2911 && c <= 2915)))))))
            : (c <= 2927 || (c < 3006
              ? (c < 2969
                ? (c < 2949
                  ? (c < 2946
                    ? c == 2929
                    : c <= 2947)
                  : (c <= 2954 || (c < 2962
                    ? (c >= 2958 && c <= 2960)
                    : c <= 2965)))
                : (c <= 2970 || (c < 2979
                  ? (c < 2974
                    ? c == 2972
                    : c <= 2975)
                  : (c <= 2980 || (c < 2990
                    ? (c >= 2984 && c <= 2986)
                    : c <= 3001)))))
              : (c <= 3010 || (c < 3072
                ? (c < 3024
                  ? (c < 3018
                    ? (c >= 3014 && c <= 3016)
                    : c <= 3021)
                  : (c <= 3024 || (c < 3046
                    ? c == 3031
                    : c <= 3055)))
                : (c <= 3084 || (c < 3114
                  ? (c < 3090
                    ? (c >= 3086 && c <= 3088)
                    : c <= 3112)
                  : (c <= 3129 || (c >= 3133 && c <= 3140)))))))))
          : (c <= 3144 || (c < 3398
            ? (c < 3260
              ? (c < 3200
                ? (c < 3160
                  ? (c < 3157
                    ? (c >= 3146 && c <= 3149)
                    : c <= 3158)
                  : (c <= 3162 || (c < 3174
                    ? (c >= 3168 && c <= 3171)
                    : c <= 3183)))
                : (c <= 3203 || (c < 3218
                  ? (c < 3214
                    ? (c >= 3205 && c <= 3212)
                    : c <= 3216)
                  : (c <= 3240 || (c < 3253
                    ? (c >= 3242 && c <= 3251)
                    : c <= 3257)))))
              : (c <= 3268 || (c < 3302
                ? (c < 3285
                  ? (c < 3274
                    ? (c >= 3270 && c <= 3272)
                    : c <= 3277)
                  : (c <= 3286 || (c < 3296
                    ? c == 3294
                    : c <= 3299)))
                : (c <= 3311 || (c < 3342
                  ? (c < 3328
                    ? (c >= 3313 && c <= 3314)
                    : c <= 3340)
                  : (c <= 3344 || (c >= 3346 && c <= 3396)))))))
            : (c <= 3400 || (c < 3530
              ? (c < 3457
                ? (c < 3423
                  ? (c < 3412
                    ? (c >= 3402 && c <= 3406)
                    : c <= 3415)
                  : (c <= 3427 || (c < 3450
                    ? (c >= 3430 && c <= 3439)
                    : c <= 3455)))
                : (c <= 3459 || (c < 3507
                  ? (c < 3482
                    ? (c >= 3461 && c <= 3478)
                    : c <= 3505)
                  : (c <= 3515 || (c < 3520
                    ? c == 3517
                    : c <= 3526)))))
              : (c <= 3530 || (c < 3585
                ? (c < 3544
                  ? (c < 3542
                    ? (c >= 3535 && c <= 3540)
                    : c <= 3542)
                  : (c <= 3551 || (c < 3570
                    ? (c >= 3558 && c <= 3567)
                    : c <= 3571)))
                : (c <= 3642 || (c < 3713
                  ? (c < 3664
                    ? (c >= 3648 && c <= 3662)
                    : c <= 3673)
                  : (c <= 3714 || c == 3716))))))))))))
      : (c <= 3722 || (c < 7296
        ? (c < 5024
          ? (c < 4256
            ? (c < 3893
              ? (c < 3784
                ? (c < 3751
                  ? (c < 3749
                    ? (c >= 3724 && c <= 3747)
                    : c <= 3749)
                  : (c <= 3773 || (c < 3782
                    ? (c >= 3776 && c <= 3780)
                    : c <= 3782)))
                : (c <= 3789 || (c < 3840
                  ? (c < 3804
                    ? (c >= 3792 && c <= 3801)
                    : c <= 3807)
                  : (c <= 3840 || (c < 3872
                    ? (c >= 3864 && c <= 3865)
                    : c <= 3881)))))
              : (c <= 3893 || (c < 3974
                ? (c < 3902
                  ? (c < 3897
                    ? c == 3895
                    : c <= 3897)
                  : (c <= 3911 || (c < 3953
                    ? (c >= 3913 && c <= 3948)
                    : c <= 3972)))
                : (c <= 3991 || (c < 4096
                  ? (c < 4038
                    ? (c >= 3993 && c <= 4028)
                    : c <= 4038)
                  : (c <= 4169 || (c >= 4176 && c <= 4253)))))))
            : (c <= 4293 || (c < 4786
              ? (c < 4688
                ? (c < 4304
                  ? (c < 4301
                    ? c == 4295
                    : c <= 4301)
                  : (c <= 4346 || (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)))
                : (c <= 4694 || (c < 4704
                  ? (c < 4698
                    ? c == 4696
                    : c <= 4701)
                  : (c <= 4744 || (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)))))
              : (c <= 4789 || (c < 4882
                ? (c < 4802
                  ? (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)
                  : (c <= 4805 || (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)))
                : (c <= 4885 || (c < 4969
                  ? (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)
                  : (c <= 4977 || (c >= 4992 && c <= 5007)))))))))
          : (c <= 5109 || (c < 6400
            ? (c < 5998
              ? (c < 5870
                ? (c < 5743
                  ? (c < 5121
                    ? (c >= 5112 && c <= 5117)
                    : c <= 5740)
                  : (c <= 5759 || (c < 5792
                    ? (c >= 5761 && c <= 5786)
                    : c <= 5866)))
                : (c <= 5880 || (c < 5920
                  ? (c < 5902
                    ? (c >= 5888 && c <= 5900)
                    : c <= 5908)
                  : (c <= 5940 || (c < 5984
                    ? (c >= 5952 && c <= 5971)
                    : c <= 5996)))))
              : (c <= 6000 || (c < 6155
                ? (c < 6103
                  ? (c < 6016
                    ? (c >= 6002 && c <= 6003)
                    : c <= 6099)
                  : (c <= 6103 || (c < 6112
                    ? (c >= 6108 && c <= 6109)
                    : c <= 6121)))
                : (c <= 6157 || (c < 6272
                  ? (c < 6176
                    ? (c >= 6160 && c <= 6169)
                    : c <= 6264)
                  : (c <= 6314 || (c >= 6320 && c <= 6389)))))))
            : (c <= 6430 || (c < 6800
              ? (c < 6576
                ? (c < 6470
                  ? (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)
                  : (c <= 6509 || (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)))
                : (c <= 6601 || (c < 6688
                  ? (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)
                  : (c <= 6750 || (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)))))
              : (c <= 6809 || (c < 7019
                ? (c < 6847
                  ? (c < 6832
                    ? c == 6823
                    : c <= 6845)
                  : (c <= 6848 || (c < 6992
                    ? (c >= 6912 && c <= 6987)
                    : c <= 7001)))
                : (c <= 7027 || (c < 7232
                  ? (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)
                  : (c <= 7241 || (c >= 7245 && c <= 7293)))))))))))
        : (c <= 7304 || (c < 11264
          ? (c < 8178
            ? (c < 8027
              ? (c < 7675
                ? (c < 7376
                  ? (c < 7357
                    ? (c >= 7312 && c <= 7354)
                    : c <= 7359)
                  : (c <= 7378 || (c < 7424
                    ? (c >= 7380 && c <= 7418)
                    : c <= 7673)))
                : (c <= 7957 || (c < 8008
                  ? (c < 7968
                    ? (c >= 7960 && c <= 7965)
                    : c <= 8005)
                  : (c <= 8013 || (c < 8025
                    ? (c >= 8016 && c <= 8023)
                    : c <= 8025)))))
              : (c <= 8027 || (c < 8130
                ? (c < 8064
                  ? (c < 8031
                    ? c == 8029
                    : c <= 8061)
                  : (c <= 8116 || (c < 8126
                    ? (c >= 8118 && c <= 8124)
                    : c <= 8126)))
                : (c <= 8132 || (c < 8150
                  ? (c < 8144
                    ? (c >= 8134 && c <= 8140)
                    : c <= 8147)
                  : (c <= 8155 || (c >= 8160 && c <= 8172)))))))
            : (c <= 8180 || (c < 8458
              ? (c < 8336
                ? (c < 8276
                  ? (c < 8255
                    ? (c >= 8182 && c <= 8188)
                    : c <= 8256)
                  : (c <= 8276 || (c < 8319
                    ? c == 8305
                    : c <= 8319)))
                : (c <= 8348 || (c < 8421
                  ? (c < 8417
                    ? (c >= 8400 && c <= 8412)
                    : c <= 8417)
                  : (c <= 8432 || (c < 8455
                    ? c == 8450
                    : c <= 8455)))))
              : (c <= 8467 || (c < 8490
                ? (c < 8484
                  ? (c < 8472
                    ? c == 8469
                    : c <= 8477)
                  : (c <= 8484 || (c < 8488
                    ? c == 8486
                    : c <= 8488)))
                : (c <= 8505 || (c < 8526
                  ? (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)
                  : (c <= 8526 || (c >= 8544 && c <= 8584)))))))))
          : (c <= 11310 || (c < 12353
            ? (c < 11696
              ? (c < 11565
                ? (c < 11499
                  ? (c < 11360
                    ? (c >= 11312 && c <= 11358)
                    : c <= 11492)
                  : (c <= 11507 || (c < 11559
                    ? (c >= 11520 && c <= 11557)
                    : c <= 11559)))
                : (c <= 11565 || (c < 11647
                  ? (c < 11631
                    ? (c >= 11568 && c <= 11623)
                    : c <= 11631)
                  : (c <= 11670 || (c < 11688
                    ? (c >= 11680 && c <= 11686)
                    : c <= 11694)))))
              : (c <= 11702 || (c < 11744
                ? (c < 11720
                  ? (c < 11712
                    ? (c >= 11704 && c <= 11710)
                    : c <= 11718)
                  : (c <= 11726 || (c < 11736
                    ? (c >= 11728 && c <= 11734)
                    : c <= 11742)))
                : (c <= 11775 || (c < 12337
                  ? (c < 12321
                    ? (c >= 12293 && c <= 12295)
                    : c <= 12335)
                  : (c <= 12341 || (c >= 12344 && c <= 12348)))))))
            : (c <= 12438 || (c < 42192
              ? (c < 12593
                ? (c < 12449
                  ? (c < 12445
                    ? (c >= 12441 && c <= 12442)
                    : c <= 12447)
                  : (c <= 12538 || (c < 12549
                    ? (c >= 12540 && c <= 12543)
                    : c <= 12591)))
                : (c <= 12686 || (c < 13312
                  ? (c < 12784
                    ? (c >= 12704 && c <= 12735)
                    : c <= 12799)
                  : (c <= 19903 || (c < 40960
                    ? (c >= 19968 && c <= 40956)
                    : c <= 42124)))))
              : (c <= 42237 || (c < 42775
                ? (c < 42560
                  ? (c < 42512
                    ? (c >= 42240 && c <= 42508)
                    : c <= 42539)
                  : (c <= 42607 || (c < 42623
                    ? (c >= 42612 && c <= 42621)
                    : c <= 42737)))
                : (c <= 42783 || (c < 42946
                  ? (c < 42891
                    ? (c >= 42786 && c <= 42888)
                    : c <= 42943)
                  : (c <= 42954 || (c >= 42997 && c <= 43047)))))))))))))))
    : (c <= 43052 || (c < 71096
      ? (c < 66864
        ? (c < 64914
          ? (c < 43816
            ? (c < 43520
              ? (c < 43261
                ? (c < 43216
                  ? (c < 43136
                    ? (c >= 43072 && c <= 43123)
                    : c <= 43205)
                  : (c <= 43225 || (c < 43259
                    ? (c >= 43232 && c <= 43255)
                    : c <= 43259)))
                : (c <= 43309 || (c < 43392
                  ? (c < 43360
                    ? (c >= 43312 && c <= 43347)
                    : c <= 43388)
                  : (c <= 43456 || (c < 43488
                    ? (c >= 43471 && c <= 43481)
                    : c <= 43518)))))
              : (c <= 43574 || (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))))
            : (c <= 43822 || (c < 64275
              ? (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))
              : (c <= 64279 || (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64612
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64605)
                  : (c <= 64829 || (c >= 64848 && c <= 64911)))))))))
          : (c <= 64967 || (c < 65549
            ? (c < 65151
              ? (c < 65137
                ? (c < 65056
                  ? (c < 65024
                    ? (c >= 65008 && c <= 65017)
                    : c <= 65039)
                  : (c <= 65071 || (c < 65101
                    ? (c >= 65075 && c <= 65076)
                    : c <= 65103)))
                : (c <= 65137 || (c < 65145
                  ? (c < 65143
                    ? c == 65139
                    : c <= 65143)
                  : (c <= 65145 || (c < 65149
                    ? c == 65147
                    : c <= 65149)))))
              : (c <= 65276 || (c < 65474
                ? (c < 65343
                  ? (c < 65313
                    ? (c >= 65296 && c <= 65305)
                    : c <= 65338)
                  : (c <= 65343 || (c < 65382
                    ? (c >= 65345 && c <= 65370)
                    : c <= 65470)))
                : (c <= 65479 || (c < 65498
                  ? (c < 65490
                    ? (c >= 65482 && c <= 65487)
                    : c <= 65495)
                  : (c <= 65500 || (c >= 65536 && c <= 65547)))))))
            : (c <= 65574 || (c < 66349
              ? (c < 65856
                ? (c < 65599
                  ? (c < 65596
                    ? (c >= 65576 && c <= 65594)
                    : c <= 65597)
                  : (c <= 65613 || (c < 65664
                    ? (c >= 65616 && c <= 65629)
                    : c <= 65786)))
                : (c <= 65908 || (c < 66208
                  ? (c < 66176
                    ? c == 66045
                    : c <= 66204)
                  : (c <= 66256 || (c < 66304
                    ? c == 66272
                    : c <= 66335)))))
              : (c <= 66378 || (c < 66560
                ? (c < 66464
                  ? (c < 66432
                    ? (c >= 66384 && c <= 66426)
                    : c <= 66461)
                  : (c <= 66499 || (c < 66513
                    ? (c >= 66504 && c <= 66511)
                    : c <= 66517)))
                : (c <= 66717 || (c < 66776
                  ? (c < 66736
                    ? (c >= 66720 && c <= 66729)
                    : c <= 66771)
                  : (c <= 66811 || (c >= 66816 && c <= 66855)))))))))))
        : (c <= 66915 || (c < 69632
          ? (c < 68152
            ? (c < 67808
              ? (c < 67594
                ? (c < 67424
                  ? (c < 67392
                    ? (c >= 67072 && c <= 67382)
                    : c <= 67413)
                  : (c <= 67431 || (c < 67592
                    ? (c >= 67584 && c <= 67589)
                    : c <= 67592)))
                : (c <= 67637 || (c < 67647
                  ? (c < 67644
                    ? (c >= 67639 && c <= 67640)
                    : c <= 67644)
                  : (c <= 67669 || (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)))))
              : (c <= 67826 || (c < 68096
                ? (c < 67872
                  ? (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)
                  : (c <= 67897 || (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)))
                : (c <= 68099 || (c < 68117
                  ? (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)
                  : (c <= 68119 || (c >= 68121 && c <= 68149)))))))
            : (c <= 68154 || (c < 68800
              ? (c < 68352
                ? (c < 68224
                  ? (c < 68192
                    ? c == 68159
                    : c <= 68220)
                  : (c <= 68252 || (c < 68297
                    ? (c >= 68288 && c <= 68295)
                    : c <= 68326)))
                : (c <= 68405 || (c < 68480
                  ? (c < 68448
                    ? (c >= 68416 && c <= 68437)
                    : c <= 68466)
                  : (c <= 68497 || (c < 68736
                    ? (c >= 68608 && c <= 68680)
                    : c <= 68786)))))
              : (c <= 68850 || (c < 69376
                ? (c < 69248
                  ? (c < 68912
                    ? (c >= 68864 && c <= 68903)
                    : c <= 68921)
                  : (c <= 69289 || (c < 69296
                    ? (c >= 69291 && c <= 69292)
                    : c <= 69297)))
                : (c <= 69404 || (c < 69552
                  ? (c < 69424
                    ? c == 69415
                    : c <= 69456)
                  : (c <= 69572 || (c >= 69600 && c <= 69622)))))))))
          : (c <= 69702 || (c < 70384
            ? (c < 70094
              ? (c < 69942
                ? (c < 69840
                  ? (c < 69759
                    ? (c >= 69734 && c <= 69743)
                    : c <= 69818)
                  : (c <= 69864 || (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)))
                : (c <= 69951 || (c < 70006
                  ? (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)
                  : (c <= 70006 || (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)))))
              : (c <= 70106 || (c < 70280
                ? (c < 70163
                  ? (c < 70144
                    ? c == 70108
                    : c <= 70161)
                  : (c <= 70199 || (c < 70272
                    ? c == 70206
                    : c <= 70278)))
                : (c <= 70280 || (c < 70303
                  ? (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)
                  : (c <= 70312 || (c >= 70320 && c <= 70378)))))))
            : (c <= 70393 || (c < 70487
              ? (c < 70450
                ? (c < 70415
                  ? (c < 70405
                    ? (c >= 70400 && c <= 70403)
                    : c <= 70412)
                  : (c <= 70416 || (c < 70442
                    ? (c >= 70419 && c <= 70440)
                    : c <= 70448)))
                : (c <= 70451 || (c < 70471
                  ? (c < 70459
                    ? (c >= 70453 && c <= 70457)
                    : c <= 70468)
                  : (c <= 70472 || (c < 70480
                    ? (c >= 70475 && c <= 70477)
                    : c <= 70480)))))
              : (c <= 70487 || (c < 70750
                ? (c < 70512
                  ? (c < 70502
                    ? (c >= 70493 && c <= 70499)
                    : c <= 70508)
                  : (c <= 70516 || (c < 70736
                    ? (c >= 70656 && c <= 70730)
                    : c <= 70745)))
                : (c <= 70753 || (c < 70864
                  ? (c < 70855
                    ? (c >= 70784 && c <= 70853)
                    : c <= 70855)
                  : (c <= 70873 || (c >= 71040 && c <= 71093)))))))))))))
      : (c <= 71104 || (c < 119894
        ? (c < 73104
          ? (c < 72163
            ? (c < 71935
              ? (c < 71360
                ? (c < 71236
                  ? (c < 71168
                    ? (c >= 71128 && c <= 71133)
                    : c <= 71232)
                  : (c <= 71236 || (c < 71296
                    ? (c >= 71248 && c <= 71257)
                    : c <= 71352)))
                : (c <= 71369 || (c < 71472
                  ? (c < 71453
                    ? (c >= 71424 && c <= 71450)
                    : c <= 71467)
                  : (c <= 71481 || (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)))))
              : (c <= 71942 || (c < 71995
                ? (c < 71957
                  ? (c < 71948
                    ? c == 71945
                    : c <= 71955)
                  : (c <= 71958 || (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)))
                : (c <= 72003 || (c < 72106
                  ? (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)
                  : (c <= 72151 || (c >= 72154 && c <= 72161)))))))
            : (c <= 72164 || (c < 72873
              ? (c < 72704
                ? (c < 72272
                  ? (c < 72263
                    ? (c >= 72192 && c <= 72254)
                    : c <= 72263)
                  : (c <= 72345 || (c < 72384
                    ? c == 72349
                    : c <= 72440)))
                : (c <= 72712 || (c < 72784
                  ? (c < 72760
                    ? (c >= 72714 && c <= 72758)
                    : c <= 72768)
                  : (c <= 72793 || (c < 72850
                    ? (c >= 72818 && c <= 72847)
                    : c <= 72871)))))
              : (c <= 72886 || (c < 73023
                ? (c < 72971
                  ? (c < 72968
                    ? (c >= 72960 && c <= 72966)
                    : c <= 72969)
                  : (c <= 73014 || (c < 73020
                    ? c == 73018
                    : c <= 73021)))
                : (c <= 73031 || (c < 73063
                  ? (c < 73056
                    ? (c >= 73040 && c <= 73049)
                    : c <= 73061)
                  : (c <= 73064 || (c >= 73066 && c <= 73102)))))))))
          : (c <= 73105 || (c < 94095
            ? (c < 92768
              ? (c < 74752
                ? (c < 73440
                  ? (c < 73120
                    ? (c >= 73107 && c <= 73112)
                    : c <= 73129)
                  : (c <= 73462 || (c < 73728
                    ? c == 73648
                    : c <= 74649)))
                : (c <= 74862 || (c < 82944
                  ? (c < 77824
                    ? (c >= 74880 && c <= 75075)
                    : c <= 78894)
                  : (c <= 83526 || (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)))))
              : (c <= 92777 || (c < 93027
                ? (c < 92928
                  ? (c < 92912
                    ? (c >= 92880 && c <= 92909)
                    : c <= 92916)
                  : (c <= 92982 || (c < 93008
                    ? (c >= 92992 && c <= 92995)
                    : c <= 93017)))
                : (c <= 93047 || (c < 93952
                  ? (c < 93760
                    ? (c >= 93053 && c <= 93071)
                    : c <= 93823)
                  : (c <= 94026 || (c >= 94031 && c <= 94087)))))))
            : (c <= 94111 || (c < 113776
              ? (c < 101632
                ? (c < 94192
                  ? (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)
                  : (c <= 94193 || (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)))
                : (c <= 101640 || (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110878)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))))
              : (c <= 113788 || (c < 119163
                ? (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 119149
                    ? (c >= 119141 && c <= 119145)
                    : c <= 119154)))
                : (c <= 119170 || (c < 119362
                  ? (c < 119210
                    ? (c >= 119173 && c <= 119179)
                    : c <= 119213)
                  : (c <= 119364 || (c >= 119808 && c <= 119892)))))))))))
        : (c <= 119964 || (c < 124928
          ? (c < 120630
            ? (c < 120094
              ? (c < 119995
                ? (c < 119973
                  ? (c < 119970
                    ? (c >= 119966 && c <= 119967)
                    : c <= 119970)
                  : (c <= 119974 || (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)))
                : (c <= 119995 || (c < 120071
                  ? (c < 120005
                    ? (c >= 119997 && c <= 120003)
                    : c <= 120069)
                  : (c <= 120074 || (c < 120086
                    ? (c >= 120077 && c <= 120084)
                    : c <= 120092)))))
              : (c <= 120121 || (c < 120488
                ? (c < 120134
                  ? (c < 120128
                    ? (c >= 120123 && c <= 120126)
                    : c <= 120132)
                  : (c <= 120134 || (c < 120146
                    ? (c >= 120138 && c <= 120144)
                    : c <= 120485)))
                : (c <= 120512 || (c < 120572
                  ? (c < 120540
                    ? (c >= 120514 && c <= 120538)
                    : c <= 120570)
                  : (c <= 120596 || (c >= 120598 && c <= 120628)))))))
            : (c <= 120654 || (c < 121505
              ? (c < 120782
                ? (c < 120714
                  ? (c < 120688
                    ? (c >= 120656 && c <= 120686)
                    : c <= 120712)
                  : (c <= 120744 || (c < 120772
                    ? (c >= 120746 && c <= 120770)
                    : c <= 120779)))
                : (c <= 120831 || (c < 121461
                  ? (c < 121403
                    ? (c >= 121344 && c <= 121398)
                    : c <= 121452)
                  : (c <= 121461 || (c < 121499
                    ? c == 121476
                    : c <= 121503)))))
              : (c <= 121519 || (c < 123136
                ? (c < 122907
                  ? (c < 122888
                    ? (c >= 122880 && c <= 122886)
                    : c <= 122904)
                  : (c <= 122913 || (c < 122918
                    ? (c >= 122915 && c <= 122916)
                    : c <= 122922)))
                : (c <= 123180 || (c < 123214
                  ? (c < 123200
                    ? (c >= 123184 && c <= 123197)
                    : c <= 123209)
                  : (c <= 123214 || (c >= 123584 && c <= 123641)))))))))
          : (c <= 125124 || (c < 126557
            ? (c < 126523
              ? (c < 126497
                ? (c < 125264
                  ? (c < 125184
                    ? (c >= 125136 && c <= 125142)
                    : c <= 125259)
                  : (c <= 125273 || (c < 126469
                    ? (c >= 126464 && c <= 126467)
                    : c <= 126495)))
                : (c <= 126498 || (c < 126505
                  ? (c < 126503
                    ? c == 126500
                    : c <= 126503)
                  : (c <= 126514 || (c < 126521
                    ? (c >= 126516 && c <= 126519)
                    : c <= 126521)))))
              : (c <= 126523 || (c < 126545
                ? (c < 126537
                  ? (c < 126535
                    ? c == 126530
                    : c <= 126535)
                  : (c <= 126537 || (c < 126541
                    ? c == 126539
                    : c <= 126543)))
                : (c <= 126546 || (c < 126553
                  ? (c < 126551
                    ? c == 126548
                    : c <= 126551)
                  : (c <= 126553 || c == 126555))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173789 || (c < 177984
                    ? (c >= 173824 && c <= 177972)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_id_character_set_4(int32_t c) {
  return (c < 43052
    ? (c < 3718
      ? (c < 2730
        ? (c < 2042
          ? (c < 1015
            ? (c < 710
              ? (c < 181
                ? (c < '_'
                  ? (c < 'A'
                    ? (c >= '0' && c <= '9')
                    : c <= 'Z')
                  : (c <= '_' || (c < 170
                    ? (c >= 'b' && c <= 'z')
                    : c <= 170)))
                : (c <= 181 || (c < 192
                  ? (c < 186
                    ? c == 183
                    : c <= 186)
                  : (c <= 214 || (c < 248
                    ? (c >= 216 && c <= 246)
                    : c <= 705)))))
              : (c <= 721 || (c < 891
                ? (c < 750
                  ? (c < 748
                    ? (c >= 736 && c <= 740)
                    : c <= 748)
                  : (c <= 750 || (c < 886
                    ? (c >= 768 && c <= 884)
                    : c <= 887)))
                : (c <= 893 || (c < 908
                  ? (c < 902
                    ? c == 895
                    : c <= 906)
                  : (c <= 908 || (c < 931
                    ? (c >= 910 && c <= 929)
                    : c <= 1013)))))))
            : (c <= 1153 || (c < 1519
              ? (c < 1425
                ? (c < 1329
                  ? (c < 1162
                    ? (c >= 1155 && c <= 1159)
                    : c <= 1327)
                  : (c <= 1366 || (c < 1376
                    ? c == 1369
                    : c <= 1416)))
                : (c <= 1469 || (c < 1476
                  ? (c < 1473
                    ? c == 1471
                    : c <= 1474)
                  : (c <= 1477 || (c < 1488
                    ? c == 1479
                    : c <= 1514)))))
              : (c <= 1522 || (c < 1770
                ? (c < 1646
                  ? (c < 1568
                    ? (c >= 1552 && c <= 1562)
                    : c <= 1641)
                  : (c <= 1747 || (c < 1759
                    ? (c >= 1749 && c <= 1756)
                    : c <= 1768)))
                : (c <= 1788 || (c < 1869
                  ? (c < 1808
                    ? c == 1791
                    : c <= 1866)
                  : (c <= 1969 || (c >= 1984 && c <= 2037)))))))))
          : (c <= 2042 || (c < 2534
            ? (c < 2447
              ? (c < 2230
                ? (c < 2112
                  ? (c < 2048
                    ? c == 2045
                    : c <= 2093)
                  : (c <= 2139 || (c < 2208
                    ? (c >= 2144 && c <= 2154)
                    : c <= 2228)))
                : (c <= 2247 || (c < 2406
                  ? (c < 2275
                    ? (c >= 2259 && c <= 2273)
                    : c <= 2403)
                  : (c <= 2415 || (c < 2437
                    ? (c >= 2417 && c <= 2435)
                    : c <= 2444)))))
              : (c <= 2448 || (c < 2503
                ? (c < 2482
                  ? (c < 2474
                    ? (c >= 2451 && c <= 2472)
                    : c <= 2480)
                  : (c <= 2482 || (c < 2492
                    ? (c >= 2486 && c <= 2489)
                    : c <= 2500)))
                : (c <= 2504 || (c < 2524
                  ? (c < 2519
                    ? (c >= 2507 && c <= 2510)
                    : c <= 2519)
                  : (c <= 2525 || (c >= 2527 && c <= 2531)))))))
            : (c <= 2545 || (c < 2622
              ? (c < 2579
                ? (c < 2561
                  ? (c < 2558
                    ? c == 2556
                    : c <= 2558)
                  : (c <= 2563 || (c < 2575
                    ? (c >= 2565 && c <= 2570)
                    : c <= 2576)))
                : (c <= 2600 || (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c < 2620
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2620)))))
              : (c <= 2626 || (c < 2662
                ? (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2637)
                  : (c <= 2641 || (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)))
                : (c <= 2677 || (c < 2703
                  ? (c < 2693
                    ? (c >= 2689 && c <= 2691)
                    : c <= 2701)
                  : (c <= 2705 || (c >= 2707 && c <= 2728)))))))))))
        : (c <= 2736 || (c < 3142
          ? (c < 2918
            ? (c < 2831
              ? (c < 2768
                ? (c < 2748
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)))
                : (c <= 2768 || (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2815 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))))
              : (c <= 2832 || (c < 2887
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))
                : (c <= 2888 || (c < 2908
                  ? (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)
                  : (c <= 2909 || (c >= 2911 && c <= 2915)))))))
            : (c <= 2927 || (c < 3006
              ? (c < 2969
                ? (c < 2949
                  ? (c < 2946
                    ? c == 2929
                    : c <= 2947)
                  : (c <= 2954 || (c < 2962
                    ? (c >= 2958 && c <= 2960)
                    : c <= 2965)))
                : (c <= 2970 || (c < 2979
                  ? (c < 2974
                    ? c == 2972
                    : c <= 2975)
                  : (c <= 2980 || (c < 2990
                    ? (c >= 2984 && c <= 2986)
                    : c <= 3001)))))
              : (c <= 3010 || (c < 3072
                ? (c < 3024
                  ? (c < 3018
                    ? (c >= 3014 && c <= 3016)
                    : c <= 3021)
                  : (c <= 3024 || (c < 3046
                    ? c == 3031
                    : c <= 3055)))
                : (c <= 3084 || (c < 3114
                  ? (c < 3090
                    ? (c >= 3086 && c <= 3088)
                    : c <= 3112)
                  : (c <= 3129 || (c >= 3133 && c <= 3140)))))))))
          : (c <= 3144 || (c < 3398
            ? (c < 3260
              ? (c < 3200
                ? (c < 3160
                  ? (c < 3157
                    ? (c >= 3146 && c <= 3149)
                    : c <= 3158)
                  : (c <= 3162 || (c < 3174
                    ? (c >= 3168 && c <= 3171)
                    : c <= 3183)))
                : (c <= 3203 || (c < 3218
                  ? (c < 3214
                    ? (c >= 3205 && c <= 3212)
                    : c <= 3216)
                  : (c <= 3240 || (c < 3253
                    ? (c >= 3242 && c <= 3251)
                    : c <= 3257)))))
              : (c <= 3268 || (c < 3302
                ? (c < 3285
                  ? (c < 3274
                    ? (c >= 3270 && c <= 3272)
                    : c <= 3277)
                  : (c <= 3286 || (c < 3296
                    ? c == 3294
                    : c <= 3299)))
                : (c <= 3311 || (c < 3342
                  ? (c < 3328
                    ? (c >= 3313 && c <= 3314)
                    : c <= 3340)
                  : (c <= 3344 || (c >= 3346 && c <= 3396)))))))
            : (c <= 3400 || (c < 3530
              ? (c < 3457
                ? (c < 3423
                  ? (c < 3412
                    ? (c >= 3402 && c <= 3406)
                    : c <= 3415)
                  : (c <= 3427 || (c < 3450
                    ? (c >= 3430 && c <= 3439)
                    : c <= 3455)))
                : (c <= 3459 || (c < 3507
                  ? (c < 3482
                    ? (c >= 3461 && c <= 3478)
                    : c <= 3505)
                  : (c <= 3515 || (c < 3520
                    ? c == 3517
                    : c <= 3526)))))
              : (c <= 3530 || (c < 3585
                ? (c < 3544
                  ? (c < 3542
                    ? (c >= 3535 && c <= 3540)
                    : c <= 3542)
                  : (c <= 3551 || (c < 3570
                    ? (c >= 3558 && c <= 3567)
                    : c <= 3571)))
                : (c <= 3642 || (c < 3713
                  ? (c < 3664
                    ? (c >= 3648 && c <= 3662)
                    : c <= 3673)
                  : (c <= 3714 || c == 3716))))))))))))
      : (c <= 3722 || (c < 7296
        ? (c < 5024
          ? (c < 4256
            ? (c < 3893
              ? (c < 3784
                ? (c < 3751
                  ? (c < 3749
                    ? (c >= 3724 && c <= 3747)
                    : c <= 3749)
                  : (c <= 3773 || (c < 3782
                    ? (c >= 3776 && c <= 3780)
                    : c <= 3782)))
                : (c <= 3789 || (c < 3840
                  ? (c < 3804
                    ? (c >= 3792 && c <= 3801)
                    : c <= 3807)
                  : (c <= 3840 || (c < 3872
                    ? (c >= 3864 && c <= 3865)
                    : c <= 3881)))))
              : (c <= 3893 || (c < 3974
                ? (c < 3902
                  ? (c < 3897
                    ? c == 3895
                    : c <= 3897)
                  : (c <= 3911 || (c < 3953
                    ? (c >= 3913 && c <= 3948)
                    : c <= 3972)))
                : (c <= 3991 || (c < 4096
                  ? (c < 4038
                    ? (c >= 3993 && c <= 4028)
                    : c <= 4038)
                  : (c <= 4169 || (c >= 4176 && c <= 4253)))))))
            : (c <= 4293 || (c < 4786
              ? (c < 4688
                ? (c < 4304
                  ? (c < 4301
                    ? c == 4295
                    : c <= 4301)
                  : (c <= 4346 || (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)))
                : (c <= 4694 || (c < 4704
                  ? (c < 4698
                    ? c == 4696
                    : c <= 4701)
                  : (c <= 4744 || (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)))))
              : (c <= 4789 || (c < 4882
                ? (c < 4802
                  ? (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)
                  : (c <= 4805 || (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)))
                : (c <= 4885 || (c < 4969
                  ? (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)
                  : (c <= 4977 || (c >= 4992 && c <= 5007)))))))))
          : (c <= 5109 || (c < 6400
            ? (c < 5998
              ? (c < 5870
                ? (c < 5743
                  ? (c < 5121
                    ? (c >= 5112 && c <= 5117)
                    : c <= 5740)
                  : (c <= 5759 || (c < 5792
                    ? (c >= 5761 && c <= 5786)
                    : c <= 5866)))
                : (c <= 5880 || (c < 5920
                  ? (c < 5902
                    ? (c >= 5888 && c <= 5900)
                    : c <= 5908)
                  : (c <= 5940 || (c < 5984
                    ? (c >= 5952 && c <= 5971)
                    : c <= 5996)))))
              : (c <= 6000 || (c < 6155
                ? (c < 6103
                  ? (c < 6016
                    ? (c >= 6002 && c <= 6003)
                    : c <= 6099)
                  : (c <= 6103 || (c < 6112
                    ? (c >= 6108 && c <= 6109)
                    : c <= 6121)))
                : (c <= 6157 || (c < 6272
                  ? (c < 6176
                    ? (c >= 6160 && c <= 6169)
                    : c <= 6264)
                  : (c <= 6314 || (c >= 6320 && c <= 6389)))))))
            : (c <= 6430 || (c < 6800
              ? (c < 6576
                ? (c < 6470
                  ? (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)
                  : (c <= 6509 || (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)))
                : (c <= 6601 || (c < 6688
                  ? (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)
                  : (c <= 6750 || (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)))))
              : (c <= 6809 || (c < 7019
                ? (c < 6847
                  ? (c < 6832
                    ? c == 6823
                    : c <= 6845)
                  : (c <= 6848 || (c < 6992
                    ? (c >= 6912 && c <= 6987)
                    : c <= 7001)))
                : (c <= 7027 || (c < 7232
                  ? (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)
                  : (c <= 7241 || (c >= 7245 && c <= 7293)))))))))))
        : (c <= 7304 || (c < 11264
          ? (c < 8178
            ? (c < 8027
              ? (c < 7675
                ? (c < 7376
                  ? (c < 7357
                    ? (c >= 7312 && c <= 7354)
                    : c <= 7359)
                  : (c <= 7378 || (c < 7424
                    ? (c >= 7380 && c <= 7418)
                    : c <= 7673)))
                : (c <= 7957 || (c < 8008
                  ? (c < 7968
                    ? (c >= 7960 && c <= 7965)
                    : c <= 8005)
                  : (c <= 8013 || (c < 8025
                    ? (c >= 8016 && c <= 8023)
                    : c <= 8025)))))
              : (c <= 8027 || (c < 8130
                ? (c < 8064
                  ? (c < 8031
                    ? c == 8029
                    : c <= 8061)
                  : (c <= 8116 || (c < 8126
                    ? (c >= 8118 && c <= 8124)
                    : c <= 8126)))
                : (c <= 8132 || (c < 8150
                  ? (c < 8144
                    ? (c >= 8134 && c <= 8140)
                    : c <= 8147)
                  : (c <= 8155 || (c >= 8160 && c <= 8172)))))))
            : (c <= 8180 || (c < 8458
              ? (c < 8336
                ? (c < 8276
                  ? (c < 8255
                    ? (c >= 8182 && c <= 8188)
                    : c <= 8256)
                  : (c <= 8276 || (c < 8319
                    ? c == 8305
                    : c <= 8319)))
                : (c <= 8348 || (c < 8421
                  ? (c < 8417
                    ? (c >= 8400 && c <= 8412)
                    : c <= 8417)
                  : (c <= 8432 || (c < 8455
                    ? c == 8450
                    : c <= 8455)))))
              : (c <= 8467 || (c < 8490
                ? (c < 8484
                  ? (c < 8472
                    ? c == 8469
                    : c <= 8477)
                  : (c <= 8484 || (c < 8488
                    ? c == 8486
                    : c <= 8488)))
                : (c <= 8505 || (c < 8526
                  ? (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)
                  : (c <= 8526 || (c >= 8544 && c <= 8584)))))))))
          : (c <= 11310 || (c < 12353
            ? (c < 11696
              ? (c < 11565
                ? (c < 11499
                  ? (c < 11360
                    ? (c >= 11312 && c <= 11358)
                    : c <= 11492)
                  : (c <= 11507 || (c < 11559
                    ? (c >= 11520 && c <= 11557)
                    : c <= 11559)))
                : (c <= 11565 || (c < 11647
                  ? (c < 11631
                    ? (c >= 11568 && c <= 11623)
                    : c <= 11631)
                  : (c <= 11670 || (c < 11688
                    ? (c >= 11680 && c <= 11686)
                    : c <= 11694)))))
              : (c <= 11702 || (c < 11744
                ? (c < 11720
                  ? (c < 11712
                    ? (c >= 11704 && c <= 11710)
                    : c <= 11718)
                  : (c <= 11726 || (c < 11736
                    ? (c >= 11728 && c <= 11734)
                    : c <= 11742)))
                : (c <= 11775 || (c < 12337
                  ? (c < 12321
                    ? (c >= 12293 && c <= 12295)
                    : c <= 12335)
                  : (c <= 12341 || (c >= 12344 && c <= 12348)))))))
            : (c <= 12438 || (c < 42192
              ? (c < 12593
                ? (c < 12449
                  ? (c < 12445
                    ? (c >= 12441 && c <= 12442)
                    : c <= 12447)
                  : (c <= 12538 || (c < 12549
                    ? (c >= 12540 && c <= 12543)
                    : c <= 12591)))
                : (c <= 12686 || (c < 13312
                  ? (c < 12784
                    ? (c >= 12704 && c <= 12735)
                    : c <= 12799)
                  : (c <= 19903 || (c < 40960
                    ? (c >= 19968 && c <= 40956)
                    : c <= 42124)))))
              : (c <= 42237 || (c < 42775
                ? (c < 42560
                  ? (c < 42512
                    ? (c >= 42240 && c <= 42508)
                    : c <= 42539)
                  : (c <= 42607 || (c < 42623
                    ? (c >= 42612 && c <= 42621)
                    : c <= 42737)))
                : (c <= 42783 || (c < 42946
                  ? (c < 42891
                    ? (c >= 42786 && c <= 42888)
                    : c <= 42943)
                  : (c <= 42954 || (c >= 42997 && c <= 43047)))))))))))))))
    : (c <= 43052 || (c < 71096
      ? (c < 66864
        ? (c < 64914
          ? (c < 43816
            ? (c < 43520
              ? (c < 43261
                ? (c < 43216
                  ? (c < 43136
                    ? (c >= 43072 && c <= 43123)
                    : c <= 43205)
                  : (c <= 43225 || (c < 43259
                    ? (c >= 43232 && c <= 43255)
                    : c <= 43259)))
                : (c <= 43309 || (c < 43392
                  ? (c < 43360
                    ? (c >= 43312 && c <= 43347)
                    : c <= 43388)
                  : (c <= 43456 || (c < 43488
                    ? (c >= 43471 && c <= 43481)
                    : c <= 43518)))))
              : (c <= 43574 || (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))))
            : (c <= 43822 || (c < 64275
              ? (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))
              : (c <= 64279 || (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64612
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64605)
                  : (c <= 64829 || (c >= 64848 && c <= 64911)))))))))
          : (c <= 64967 || (c < 65549
            ? (c < 65151
              ? (c < 65137
                ? (c < 65056
                  ? (c < 65024
                    ? (c >= 65008 && c <= 65017)
                    : c <= 65039)
                  : (c <= 65071 || (c < 65101
                    ? (c >= 65075 && c <= 65076)
                    : c <= 65103)))
                : (c <= 65137 || (c < 65145
                  ? (c < 65143
                    ? c == 65139
                    : c <= 65143)
                  : (c <= 65145 || (c < 65149
                    ? c == 65147
                    : c <= 65149)))))
              : (c <= 65276 || (c < 65474
                ? (c < 65343
                  ? (c < 65313
                    ? (c >= 65296 && c <= 65305)
                    : c <= 65338)
                  : (c <= 65343 || (c < 65382
                    ? (c >= 65345 && c <= 65370)
                    : c <= 65470)))
                : (c <= 65479 || (c < 65498
                  ? (c < 65490
                    ? (c >= 65482 && c <= 65487)
                    : c <= 65495)
                  : (c <= 65500 || (c >= 65536 && c <= 65547)))))))
            : (c <= 65574 || (c < 66349
              ? (c < 65856
                ? (c < 65599
                  ? (c < 65596
                    ? (c >= 65576 && c <= 65594)
                    : c <= 65597)
                  : (c <= 65613 || (c < 65664
                    ? (c >= 65616 && c <= 65629)
                    : c <= 65786)))
                : (c <= 65908 || (c < 66208
                  ? (c < 66176
                    ? c == 66045
                    : c <= 66204)
                  : (c <= 66256 || (c < 66304
                    ? c == 66272
                    : c <= 66335)))))
              : (c <= 66378 || (c < 66560
                ? (c < 66464
                  ? (c < 66432
                    ? (c >= 66384 && c <= 66426)
                    : c <= 66461)
                  : (c <= 66499 || (c < 66513
                    ? (c >= 66504 && c <= 66511)
                    : c <= 66517)))
                : (c <= 66717 || (c < 66776
                  ? (c < 66736
                    ? (c >= 66720 && c <= 66729)
                    : c <= 66771)
                  : (c <= 66811 || (c >= 66816 && c <= 66855)))))))))))
        : (c <= 66915 || (c < 69632
          ? (c < 68152
            ? (c < 67808
              ? (c < 67594
                ? (c < 67424
                  ? (c < 67392
                    ? (c >= 67072 && c <= 67382)
                    : c <= 67413)
                  : (c <= 67431 || (c < 67592
                    ? (c >= 67584 && c <= 67589)
                    : c <= 67592)))
                : (c <= 67637 || (c < 67647
                  ? (c < 67644
                    ? (c >= 67639 && c <= 67640)
                    : c <= 67644)
                  : (c <= 67669 || (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)))))
              : (c <= 67826 || (c < 68096
                ? (c < 67872
                  ? (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)
                  : (c <= 67897 || (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)))
                : (c <= 68099 || (c < 68117
                  ? (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)
                  : (c <= 68119 || (c >= 68121 && c <= 68149)))))))
            : (c <= 68154 || (c < 68800
              ? (c < 68352
                ? (c < 68224
                  ? (c < 68192
                    ? c == 68159
                    : c <= 68220)
                  : (c <= 68252 || (c < 68297
                    ? (c >= 68288 && c <= 68295)
                    : c <= 68326)))
                : (c <= 68405 || (c < 68480
                  ? (c < 68448
                    ? (c >= 68416 && c <= 68437)
                    : c <= 68466)
                  : (c <= 68497 || (c < 68736
                    ? (c >= 68608 && c <= 68680)
                    : c <= 68786)))))
              : (c <= 68850 || (c < 69376
                ? (c < 69248
                  ? (c < 68912
                    ? (c >= 68864 && c <= 68903)
                    : c <= 68921)
                  : (c <= 69289 || (c < 69296
                    ? (c >= 69291 && c <= 69292)
                    : c <= 69297)))
                : (c <= 69404 || (c < 69552
                  ? (c < 69424
                    ? c == 69415
                    : c <= 69456)
                  : (c <= 69572 || (c >= 69600 && c <= 69622)))))))))
          : (c <= 69702 || (c < 70384
            ? (c < 70094
              ? (c < 69942
                ? (c < 69840
                  ? (c < 69759
                    ? (c >= 69734 && c <= 69743)
                    : c <= 69818)
                  : (c <= 69864 || (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)))
                : (c <= 69951 || (c < 70006
                  ? (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)
                  : (c <= 70006 || (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)))))
              : (c <= 70106 || (c < 70280
                ? (c < 70163
                  ? (c < 70144
                    ? c == 70108
                    : c <= 70161)
                  : (c <= 70199 || (c < 70272
                    ? c == 70206
                    : c <= 70278)))
                : (c <= 70280 || (c < 70303
                  ? (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)
                  : (c <= 70312 || (c >= 70320 && c <= 70378)))))))
            : (c <= 70393 || (c < 70487
              ? (c < 70450
                ? (c < 70415
                  ? (c < 70405
                    ? (c >= 70400 && c <= 70403)
                    : c <= 70412)
                  : (c <= 70416 || (c < 70442
                    ? (c >= 70419 && c <= 70440)
                    : c <= 70448)))
                : (c <= 70451 || (c < 70471
                  ? (c < 70459
                    ? (c >= 70453 && c <= 70457)
                    : c <= 70468)
                  : (c <= 70472 || (c < 70480
                    ? (c >= 70475 && c <= 70477)
                    : c <= 70480)))))
              : (c <= 70487 || (c < 70750
                ? (c < 70512
                  ? (c < 70502
                    ? (c >= 70493 && c <= 70499)
                    : c <= 70508)
                  : (c <= 70516 || (c < 70736
                    ? (c >= 70656 && c <= 70730)
                    : c <= 70745)))
                : (c <= 70753 || (c < 70864
                  ? (c < 70855
                    ? (c >= 70784 && c <= 70853)
                    : c <= 70855)
                  : (c <= 70873 || (c >= 71040 && c <= 71093)))))))))))))
      : (c <= 71104 || (c < 119894
        ? (c < 73104
          ? (c < 72163
            ? (c < 71935
              ? (c < 71360
                ? (c < 71236
                  ? (c < 71168
                    ? (c >= 71128 && c <= 71133)
                    : c <= 71232)
                  : (c <= 71236 || (c < 71296
                    ? (c >= 71248 && c <= 71257)
                    : c <= 71352)))
                : (c <= 71369 || (c < 71472
                  ? (c < 71453
                    ? (c >= 71424 && c <= 71450)
                    : c <= 71467)
                  : (c <= 71481 || (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)))))
              : (c <= 71942 || (c < 71995
                ? (c < 71957
                  ? (c < 71948
                    ? c == 71945
                    : c <= 71955)
                  : (c <= 71958 || (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)))
                : (c <= 72003 || (c < 72106
                  ? (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)
                  : (c <= 72151 || (c >= 72154 && c <= 72161)))))))
            : (c <= 72164 || (c < 72873
              ? (c < 72704
                ? (c < 72272
                  ? (c < 72263
                    ? (c >= 72192 && c <= 72254)
                    : c <= 72263)
                  : (c <= 72345 || (c < 72384
                    ? c == 72349
                    : c <= 72440)))
                : (c <= 72712 || (c < 72784
                  ? (c < 72760
                    ? (c >= 72714 && c <= 72758)
                    : c <= 72768)
                  : (c <= 72793 || (c < 72850
                    ? (c >= 72818 && c <= 72847)
                    : c <= 72871)))))
              : (c <= 72886 || (c < 73023
                ? (c < 72971
                  ? (c < 72968
                    ? (c >= 72960 && c <= 72966)
                    : c <= 72969)
                  : (c <= 73014 || (c < 73020
                    ? c == 73018
                    : c <= 73021)))
                : (c <= 73031 || (c < 73063
                  ? (c < 73056
                    ? (c >= 73040 && c <= 73049)
                    : c <= 73061)
                  : (c <= 73064 || (c >= 73066 && c <= 73102)))))))))
          : (c <= 73105 || (c < 94095
            ? (c < 92768
              ? (c < 74752
                ? (c < 73440
                  ? (c < 73120
                    ? (c >= 73107 && c <= 73112)
                    : c <= 73129)
                  : (c <= 73462 || (c < 73728
                    ? c == 73648
                    : c <= 74649)))
                : (c <= 74862 || (c < 82944
                  ? (c < 77824
                    ? (c >= 74880 && c <= 75075)
                    : c <= 78894)
                  : (c <= 83526 || (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)))))
              : (c <= 92777 || (c < 93027
                ? (c < 92928
                  ? (c < 92912
                    ? (c >= 92880 && c <= 92909)
                    : c <= 92916)
                  : (c <= 92982 || (c < 93008
                    ? (c >= 92992 && c <= 92995)
                    : c <= 93017)))
                : (c <= 93047 || (c < 93952
                  ? (c < 93760
                    ? (c >= 93053 && c <= 93071)
                    : c <= 93823)
                  : (c <= 94026 || (c >= 94031 && c <= 94087)))))))
            : (c <= 94111 || (c < 113776
              ? (c < 101632
                ? (c < 94192
                  ? (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)
                  : (c <= 94193 || (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)))
                : (c <= 101640 || (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110878)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))))
              : (c <= 113788 || (c < 119163
                ? (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 119149
                    ? (c >= 119141 && c <= 119145)
                    : c <= 119154)))
                : (c <= 119170 || (c < 119362
                  ? (c < 119210
                    ? (c >= 119173 && c <= 119179)
                    : c <= 119213)
                  : (c <= 119364 || (c >= 119808 && c <= 119892)))))))))))
        : (c <= 119964 || (c < 124928
          ? (c < 120630
            ? (c < 120094
              ? (c < 119995
                ? (c < 119973
                  ? (c < 119970
                    ? (c >= 119966 && c <= 119967)
                    : c <= 119970)
                  : (c <= 119974 || (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)))
                : (c <= 119995 || (c < 120071
                  ? (c < 120005
                    ? (c >= 119997 && c <= 120003)
                    : c <= 120069)
                  : (c <= 120074 || (c < 120086
                    ? (c >= 120077 && c <= 120084)
                    : c <= 120092)))))
              : (c <= 120121 || (c < 120488
                ? (c < 120134
                  ? (c < 120128
                    ? (c >= 120123 && c <= 120126)
                    : c <= 120132)
                  : (c <= 120134 || (c < 120146
                    ? (c >= 120138 && c <= 120144)
                    : c <= 120485)))
                : (c <= 120512 || (c < 120572
                  ? (c < 120540
                    ? (c >= 120514 && c <= 120538)
                    : c <= 120570)
                  : (c <= 120596 || (c >= 120598 && c <= 120628)))))))
            : (c <= 120654 || (c < 121505
              ? (c < 120782
                ? (c < 120714
                  ? (c < 120688
                    ? (c >= 120656 && c <= 120686)
                    : c <= 120712)
                  : (c <= 120744 || (c < 120772
                    ? (c >= 120746 && c <= 120770)
                    : c <= 120779)))
                : (c <= 120831 || (c < 121461
                  ? (c < 121403
                    ? (c >= 121344 && c <= 121398)
                    : c <= 121452)
                  : (c <= 121461 || (c < 121499
                    ? c == 121476
                    : c <= 121503)))))
              : (c <= 121519 || (c < 123136
                ? (c < 122907
                  ? (c < 122888
                    ? (c >= 122880 && c <= 122886)
                    : c <= 122904)
                  : (c <= 122913 || (c < 122918
                    ? (c >= 122915 && c <= 122916)
                    : c <= 122922)))
                : (c <= 123180 || (c < 123214
                  ? (c < 123200
                    ? (c >= 123184 && c <= 123197)
                    : c <= 123209)
                  : (c <= 123214 || (c >= 123584 && c <= 123641)))))))))
          : (c <= 125124 || (c < 126557
            ? (c < 126523
              ? (c < 126497
                ? (c < 125264
                  ? (c < 125184
                    ? (c >= 125136 && c <= 125142)
                    : c <= 125259)
                  : (c <= 125273 || (c < 126469
                    ? (c >= 126464 && c <= 126467)
                    : c <= 126495)))
                : (c <= 126498 || (c < 126505
                  ? (c < 126503
                    ? c == 126500
                    : c <= 126503)
                  : (c <= 126514 || (c < 126521
                    ? (c >= 126516 && c <= 126519)
                    : c <= 126521)))))
              : (c <= 126523 || (c < 126545
                ? (c < 126537
                  ? (c < 126535
                    ? c == 126530
                    : c <= 126535)
                  : (c <= 126537 || (c < 126541
                    ? c == 126539
                    : c <= 126543)))
                : (c <= 126546 || (c < 126553
                  ? (c < 126551
                    ? c == 126548
                    : c <= 126551)
                  : (c <= 126553 || c == 126555))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173789 || (c < 177984
                    ? (c >= 173824 && c <= 177972)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(22);
      if (lookahead == '\n') ADVANCE(96);
      if (lookahead == '\r') ADVANCE(95);
      if (lookahead == '!') ADVANCE(47);
      if (lookahead == '"') ADVANCE(84);
      if (lookahead == '#') ADVANCE(46);
      if (lookahead == '\'') ADVANCE(76);
      if (lookahead == '(') ADVANCE(37);
      if (lookahead == ')') ADVANCE(38);
      if (lookahead == '*') ADVANCE(42);
      if (lookahead == '+') ADVANCE(43);
      if (lookahead == ',') ADVANCE(24);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '.') ADVANCE(51);
      if (lookahead == '/') ADVANCE(92);
      if (lookahead == '0') ADVANCE(74);
      if (lookahead == ':') ADVANCE(48);
      if (lookahead == ';') ADVANCE(103);
      if (lookahead == '<') ADVANCE(98);
      if (lookahead == '=') ADVANCE(33);
      if (lookahead == '?') ADVANCE(41);
      if (lookahead == '@') ADVANCE(50);
      if (lookahead == '[') ADVANCE(27);
      if (lookahead == '\\') ADVANCE(99);
      if (lookahead == ']') ADVANCE(28);
      if (lookahead == '^') ADVANCE(40);
      if (lookahead == 'f') ADVANCE(68);
      if (lookahead == 'g') ADVANCE(69);
      if (lookahead == 'i') ADVANCE(60);
      if (lookahead == '{') ADVANCE(23);
      if (lookahead == '|') ADVANCE(32);
      if (lookahead == '}') ADVANCE(25);
      if (lookahead == '~') ADVANCE(44);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(95);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(75);
      if (sym_id_character_set_1(lookahead)) ADVANCE(73);
      if (lookahead != 0) ADVANCE(95);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(96);
      if (lookahead == '\r') ADVANCE(95);
      if (lookahead == '-') ADVANCE(100);
      if (lookahead == '/') ADVANCE(97);
      if (lookahead == '\\') ADVANCE(99);
      if (lookahead == ']') ADVANCE(28);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(95);
      if (lookahead != 0) ADVANCE(95);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(96);
      if (lookahead == '\r') ADVANCE(95);
      if (lookahead == '/') ADVANCE(97);
      if (lookahead == '\\') ADVANCE(99);
      if (lookahead == ']') ADVANCE(28);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(95);
      if (lookahead != 0) ADVANCE(95);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(12);
      if (lookahead == '*') ADVANCE(87);
      if (lookahead != 0) ADVANCE(88);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(12);
      if (lookahead == '*') ADVANCE(79);
      if (lookahead != 0) ADVANCE(80);
      END_STATE();
    case 5:
      if (lookahead == '\n') ADVANCE(112);
      if (lookahead == '\r') ADVANCE(110);
      if (lookahead == '#') ADVANCE(46);
      if (lookahead == ')') ADVANCE(38);
      if (lookahead == '*') ADVANCE(42);
      if (lookahead == '+') ADVANCE(43);
      if (lookahead == ',') ADVANCE(24);
      if (lookahead == '/') ADVANCE(9);
      if (lookahead == ':') ADVANCE(48);
      if (lookahead == '<') ADVANCE(13);
      if (lookahead == '?') ADVANCE(41);
      if (lookahead == 'g' ||
          lookahead == 'i') ADVANCE(93);
      if (lookahead == '|') ADVANCE(32);
      if (lookahead == '~') ADVANCE(44);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(111);
      if (lookahead == '!' ||
          lookahead == '^') ADVANCE(47);
      END_STATE();
    case 6:
      if (lookahead == '\n') ADVANCE(112);
      if (lookahead == '\r') ADVANCE(110);
      if (lookahead == '/') ADVANCE(9);
      if (lookahead == '=') ADVANCE(33);
      if (lookahead == '@') ADVANCE(14);
      if (lookahead == '^') ADVANCE(15);
      if (lookahead == '_') ADVANCE(16);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(111);
      END_STATE();
    case 7:
      if (lookahead == '\n') ADVANCE(112);
      if (lookahead == '\r') ADVANCE(110);
      if (lookahead == '/') ADVANCE(9);
      if (lookahead == '[') ADVANCE(26);
      if (lookahead == '{') ADVANCE(23);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(111);
      if (sym_id_character_set_2(lookahead)) ADVANCE(73);
      END_STATE();
    case 8:
      if (lookahead == '\n') ADVANCE(112);
      if (lookahead == '\r') ADVANCE(110);
      if (lookahead == '/') ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(111);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(102);
      END_STATE();
    case 9:
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '/') ADVANCE(109);
      END_STATE();
    case 10:
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '/') ADVANCE(108);
      END_STATE();
    case 11:
      if (lookahead == '*') ADVANCE(11);
      if (lookahead == '/') ADVANCE(105);
      if (lookahead != 0) ADVANCE(12);
      END_STATE();
    case 12:
      if (lookahead == '*') ADVANCE(11);
      if (lookahead != 0) ADVANCE(12);
      END_STATE();
    case 13:
      if (lookahead == '-') ADVANCE(45);
      END_STATE();
    case 14:
      if (lookahead == '=') ADVANCE(35);
      END_STATE();
    case 15:
      if (lookahead == '=') ADVANCE(36);
      END_STATE();
    case 16:
      if (lookahead == '=') ADVANCE(34);
      END_STATE();
    case 17:
      if (lookahead == 'p') ADVANCE(101);
      END_STATE();
    case 18:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(91);
      END_STATE();
    case 19:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(83);
      END_STATE();
    case 20:
      if (eof) ADVANCE(22);
      if (lookahead == '\n') ADVANCE(112);
      if (lookahead == '\r') ADVANCE(110);
      if (lookahead == '"') ADVANCE(84);
      if (lookahead == '#') ADVANCE(46);
      if (lookahead == '\'') ADVANCE(76);
      if (lookahead == '(') ADVANCE(37);
      if (lookahead == ')') ADVANCE(38);
      if (lookahead == '*') ADVANCE(42);
      if (lookahead == '+') ADVANCE(43);
      if (lookahead == ',') ADVANCE(24);
      if (lookahead == '.') ADVANCE(51);
      if (lookahead == '/') ADVANCE(92);
      if (lookahead == '0') ADVANCE(74);
      if (lookahead == ':') ADVANCE(48);
      if (lookahead == '<') ADVANCE(13);
      if (lookahead == '?') ADVANCE(41);
      if (lookahead == '@') ADVANCE(49);
      if (lookahead == '[') ADVANCE(27);
      if (lookahead == '\\') ADVANCE(17);
      if (lookahead == ']') ADVANCE(28);
      if (lookahead == '^') ADVANCE(39);
      if (lookahead == '{') ADVANCE(23);
      if (lookahead == '|') ADVANCE(32);
      if (lookahead == '}') ADVANCE(25);
      if (lookahead == '~') ADVANCE(44);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(111);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(75);
      if (sym_id_character_set_1(lookahead)) ADVANCE(73);
      END_STATE();
    case 21:
      if (eof) ADVANCE(22);
      if (lookahead == '\n') ADVANCE(112);
      if (lookahead == '\r') ADVANCE(110);
      if (lookahead == '"') ADVANCE(84);
      if (lookahead == '#') ADVANCE(46);
      if (lookahead == '\'') ADVANCE(76);
      if (lookahead == '*') ADVANCE(42);
      if (lookahead == '+') ADVANCE(43);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(48);
      if (lookahead == ';') ADVANCE(103);
      if (lookahead == '<') ADVANCE(13);
      if (lookahead == '?') ADVANCE(41);
      if (lookahead == '[') ADVANCE(26);
      if (lookahead == 'f') ADVANCE(68);
      if (lookahead == 'g') ADVANCE(69);
      if (lookahead == 'i') ADVANCE(60);
      if (lookahead == '{') ADVANCE(23);
      if (lookahead == '|') ADVANCE(32);
      if (lookahead == '~') ADVANCE(44);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(111);
      if (lookahead == '!' ||
          lookahead == '^') ADVANCE(47);
      if (sym_id_character_set_2(lookahead)) ADVANCE(73);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      if (lookahead == '^') ADVANCE(94);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_grammar);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_fragment);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_ignore);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym__EQ);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_AT_EQ);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_CARET_EQ);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_CARET);
      if (lookahead == '=') ADVANCE(36);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_QMARK);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_LT_DASH);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(aux_sym_choice_tag_token1);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead == '=') ADVANCE(35);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == '!') ADVANCE(31);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == '!') ADVANCE(29);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == '!') ADVANCE(30);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(64);
      if (sym_id_character_set_4(lookahead)) ADVANCE(73);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(61);
      if (sym_id_character_set_4(lookahead)) ADVANCE(73);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(71);
      if (sym_id_character_set_4(lookahead)) ADVANCE(73);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'e') ADVANCE(52);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'e') ADVANCE(66);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'g') ADVANCE(65);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'g') ADVANCE(62);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(59);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(57);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(63);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'n') ADVANCE(67);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'n') ADVANCE(72);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'o') ADVANCE(70);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(56);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(55);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(58);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(53);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 't') ADVANCE(54);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_id);
      if (sym_id_character_set_3(lookahead)) ADVANCE(73);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_unsigned);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_unsigned);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(75);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\n') ADVANCE(78);
      if (lookahead == '\r') ADVANCE(83);
      if (lookahead == '/') ADVANCE(82);
      if (lookahead == '\\') ADVANCE(19);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(83);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(83);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\r') ADVANCE(83);
      if (lookahead == '\\') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(83);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\'') ADVANCE(12);
      if (lookahead == '*') ADVANCE(79);
      if (lookahead == '/') ADVANCE(83);
      if (lookahead == '\\') ADVANCE(4);
      if (lookahead != 0) ADVANCE(80);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\'') ADVANCE(12);
      if (lookahead == '*') ADVANCE(79);
      if (lookahead == '\\') ADVANCE(4);
      if (lookahead != 0) ADVANCE(80);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\'') ADVANCE(109);
      if (lookahead == '\\') ADVANCE(107);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(83);
      if (lookahead != 0) ADVANCE(81);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '*') ADVANCE(80);
      if (lookahead == '/') ADVANCE(81);
      if (lookahead == '\\') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(83);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\\') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(83);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\n') ADVANCE(86);
      if (lookahead == '\r') ADVANCE(91);
      if (lookahead == '/') ADVANCE(90);
      if (lookahead == '\\') ADVANCE(18);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(91);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(91);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\r') ADVANCE(91);
      if (lookahead == '\\') ADVANCE(18);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(91);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '"') ADVANCE(12);
      if (lookahead == '*') ADVANCE(87);
      if (lookahead == '/') ADVANCE(91);
      if (lookahead == '\\') ADVANCE(3);
      if (lookahead != 0) ADVANCE(88);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '"') ADVANCE(12);
      if (lookahead == '*') ADVANCE(87);
      if (lookahead == '\\') ADVANCE(3);
      if (lookahead != 0) ADVANCE(88);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '"') ADVANCE(109);
      if (lookahead == '\\') ADVANCE(106);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(91);
      if (lookahead != 0) ADVANCE(89);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '*') ADVANCE(88);
      if (lookahead == '/') ADVANCE(89);
      if (lookahead == '\\') ADVANCE(18);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(91);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\\') ADVANCE(18);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(91);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '/') ADVANCE(109);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(aux_sym_regex_long_token1);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_LBRACK_CARET);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == '\r') ADVANCE(110);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '/') ADVANCE(109);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == '-') ADVANCE(45);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == 'p') ADVANCE(101);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_BSLASHp);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(aux_sym_regex_set_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(102);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(sym_eos);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(sym_comment_doc);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(104);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(sym_COMMENT);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(sym_COMMENT);
      if (lookahead == '\r') ADVANCE(91);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(89);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(sym_COMMENT);
      if (lookahead == '\r') ADVANCE(83);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(81);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(sym_COMMENT);
      if (lookahead == '!' ||
          lookahead == '*' ||
          lookahead == '?') ADVANCE(104);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(109);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(sym_COMMENT);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(109);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(sym_NEWLINE);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(sym_WHITESPACE);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(sym_WHITESPACE);
      if (lookahead == '\r') ADVANCE(110);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 21},
  [2] = {.lex_state = 20},
  [3] = {.lex_state = 20},
  [4] = {.lex_state = 20},
  [5] = {.lex_state = 20},
  [6] = {.lex_state = 20},
  [7] = {.lex_state = 20},
  [8] = {.lex_state = 20},
  [9] = {.lex_state = 20},
  [10] = {.lex_state = 20},
  [11] = {.lex_state = 20},
  [12] = {.lex_state = 20},
  [13] = {.lex_state = 20},
  [14] = {.lex_state = 20},
  [15] = {.lex_state = 20},
  [16] = {.lex_state = 20},
  [17] = {.lex_state = 20},
  [18] = {.lex_state = 20},
  [19] = {.lex_state = 20},
  [20] = {.lex_state = 20},
  [21] = {.lex_state = 20},
  [22] = {.lex_state = 20},
  [23] = {.lex_state = 20},
  [24] = {.lex_state = 20},
  [25] = {.lex_state = 20},
  [26] = {.lex_state = 20},
  [27] = {.lex_state = 20},
  [28] = {.lex_state = 20},
  [29] = {.lex_state = 21},
  [30] = {.lex_state = 21},
  [31] = {.lex_state = 21},
  [32] = {.lex_state = 21},
  [33] = {.lex_state = 21},
  [34] = {.lex_state = 21},
  [35] = {.lex_state = 21},
  [36] = {.lex_state = 21},
  [37] = {.lex_state = 21},
  [38] = {.lex_state = 21},
  [39] = {.lex_state = 21},
  [40] = {.lex_state = 21},
  [41] = {.lex_state = 21},
  [42] = {.lex_state = 21},
  [43] = {.lex_state = 21},
  [44] = {.lex_state = 21},
  [45] = {.lex_state = 21},
  [46] = {.lex_state = 21},
  [47] = {.lex_state = 21},
  [48] = {.lex_state = 21},
  [49] = {.lex_state = 21},
  [50] = {.lex_state = 21},
  [51] = {.lex_state = 21},
  [52] = {.lex_state = 21},
  [53] = {.lex_state = 21},
  [54] = {.lex_state = 21},
  [55] = {.lex_state = 21},
  [56] = {.lex_state = 21},
  [57] = {.lex_state = 21},
  [58] = {.lex_state = 21},
  [59] = {.lex_state = 21},
  [60] = {.lex_state = 20},
  [61] = {.lex_state = 20},
  [62] = {.lex_state = 20},
  [63] = {.lex_state = 20},
  [64] = {.lex_state = 20},
  [65] = {.lex_state = 5},
  [66] = {.lex_state = 20},
  [67] = {.lex_state = 20},
  [68] = {.lex_state = 20},
  [69] = {.lex_state = 5},
  [70] = {.lex_state = 20},
  [71] = {.lex_state = 20},
  [72] = {.lex_state = 20},
  [73] = {.lex_state = 20},
  [74] = {.lex_state = 20},
  [75] = {.lex_state = 20},
  [76] = {.lex_state = 20},
  [77] = {.lex_state = 20},
  [78] = {.lex_state = 20},
  [79] = {.lex_state = 20},
  [80] = {.lex_state = 20},
  [81] = {.lex_state = 20},
  [82] = {.lex_state = 20},
  [83] = {.lex_state = 20},
  [84] = {.lex_state = 20},
  [85] = {.lex_state = 20},
  [86] = {.lex_state = 20},
  [87] = {.lex_state = 20},
  [88] = {.lex_state = 20},
  [89] = {.lex_state = 20},
  [90] = {.lex_state = 20},
  [91] = {.lex_state = 20},
  [92] = {.lex_state = 20},
  [93] = {.lex_state = 20},
  [94] = {.lex_state = 20},
  [95] = {.lex_state = 20},
  [96] = {.lex_state = 2},
  [97] = {.lex_state = 21},
  [98] = {.lex_state = 21},
  [99] = {.lex_state = 21},
  [100] = {.lex_state = 2},
  [101] = {.lex_state = 21},
  [102] = {.lex_state = 2},
  [103] = {.lex_state = 21},
  [104] = {.lex_state = 21},
  [105] = {.lex_state = 21},
  [106] = {.lex_state = 21},
  [107] = {.lex_state = 21},
  [108] = {.lex_state = 21},
  [109] = {.lex_state = 21},
  [110] = {.lex_state = 21},
  [111] = {.lex_state = 2},
  [112] = {.lex_state = 2},
  [113] = {.lex_state = 21},
  [114] = {.lex_state = 21},
  [115] = {.lex_state = 21},
  [116] = {.lex_state = 21},
  [117] = {.lex_state = 21},
  [118] = {.lex_state = 21},
  [119] = {.lex_state = 21},
  [120] = {.lex_state = 21},
  [121] = {.lex_state = 21},
  [122] = {.lex_state = 21},
  [123] = {.lex_state = 21},
  [124] = {.lex_state = 21},
  [125] = {.lex_state = 21},
  [126] = {.lex_state = 21},
  [127] = {.lex_state = 21},
  [128] = {.lex_state = 21},
  [129] = {.lex_state = 21},
  [130] = {.lex_state = 21},
  [131] = {.lex_state = 21},
  [132] = {.lex_state = 21},
  [133] = {.lex_state = 21},
  [134] = {.lex_state = 21},
  [135] = {.lex_state = 21},
  [136] = {.lex_state = 21},
  [137] = {.lex_state = 21},
  [138] = {.lex_state = 21},
  [139] = {.lex_state = 21},
  [140] = {.lex_state = 2},
  [141] = {.lex_state = 6},
  [142] = {.lex_state = 20},
  [143] = {.lex_state = 1},
  [144] = {.lex_state = 20},
  [145] = {.lex_state = 20},
  [146] = {.lex_state = 20},
  [147] = {.lex_state = 20},
  [148] = {.lex_state = 1},
  [149] = {.lex_state = 20},
  [150] = {.lex_state = 1},
  [151] = {.lex_state = 20},
  [152] = {.lex_state = 20},
  [153] = {.lex_state = 1},
  [154] = {.lex_state = 20},
  [155] = {.lex_state = 20},
  [156] = {.lex_state = 20},
  [157] = {.lex_state = 20},
  [158] = {.lex_state = 20},
  [159] = {.lex_state = 20},
  [160] = {.lex_state = 20},
  [161] = {.lex_state = 20},
  [162] = {.lex_state = 7},
  [163] = {.lex_state = 20},
  [164] = {.lex_state = 20},
  [165] = {.lex_state = 20},
  [166] = {.lex_state = 20},
  [167] = {.lex_state = 20},
  [168] = {.lex_state = 20},
  [169] = {.lex_state = 20},
  [170] = {.lex_state = 20},
  [171] = {.lex_state = 20},
  [172] = {.lex_state = 20},
  [173] = {.lex_state = 20},
  [174] = {.lex_state = 20},
  [175] = {.lex_state = 20},
  [176] = {.lex_state = 20},
  [177] = {.lex_state = 20},
  [178] = {.lex_state = 20},
  [179] = {.lex_state = 20},
  [180] = {.lex_state = 20},
  [181] = {.lex_state = 20},
  [182] = {.lex_state = 20},
  [183] = {.lex_state = 20},
  [184] = {.lex_state = 20},
  [185] = {.lex_state = 20},
  [186] = {.lex_state = 20},
  [187] = {.lex_state = 85},
  [188] = {.lex_state = 77},
  [189] = {.lex_state = 20},
  [190] = {.lex_state = 20},
  [191] = {.lex_state = 20},
  [192] = {.lex_state = 20},
  [193] = {.lex_state = 20},
  [194] = {.lex_state = 20},
  [195] = {.lex_state = 20},
  [196] = {.lex_state = 20},
  [197] = {.lex_state = 20},
  [198] = {.lex_state = 20},
  [199] = {.lex_state = 20},
  [200] = {.lex_state = 20},
  [201] = {.lex_state = 77},
  [202] = {.lex_state = 85},
  [203] = {.lex_state = 20},
  [204] = {.lex_state = 20},
  [205] = {.lex_state = 8},
  [206] = {.lex_state = 20},
  [207] = {.lex_state = 20},
  [208] = {.lex_state = 8},
  [209] = {.lex_state = 20},
  [210] = {.lex_state = 8},
  [211] = {.lex_state = 20},
  [212] = {.lex_state = 20},
  [213] = {.lex_state = 20},
  [214] = {.lex_state = 20},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_id] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym_grammar] = ACTIONS(1),
    [sym_fragment] = ACTIONS(1),
    [sym_ignore] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_AT_EQ] = ACTIONS(1),
    [anon_sym_CARET_EQ] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_CARET] = ACTIONS(1),
    [anon_sym_QMARK] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_LT_DASH] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [aux_sym_choice_tag_token1] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [sym_unsigned] = ACTIONS(1),
    [sym__sign] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [aux_sym_regex_long_token1] = ACTIONS(1),
    [anon_sym_LBRACK_CARET] = ACTIONS(1),
    [aux_sym_regex_range_item_token1] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_BSLASHp] = ACTIONS(1),
    [sym_eos] = ACTIONS(1),
    [sym_COMMENT] = ACTIONS(3),
    [sym_NEWLINE] = ACTIONS(5),
    [sym_WHITESPACE] = ACTIONS(5),
  },
  [1] = {
    [sym_program] = STATE(199),
    [sym_statement] = STATE(128),
    [sym_grammar_statement] = STATE(123),
    [sym_fragment_statement] = STATE(123),
    [sym_ignore_statement] = STATE(123),
    [sym_assign_statement] = STATE(123),
    [aux_sym_program_repeat1] = STATE(58),
    [ts_builtin_sym_end] = ACTIONS(7),
    [sym_id] = ACTIONS(9),
    [sym_grammar] = ACTIONS(11),
    [sym_fragment] = ACTIONS(13),
    [sym_ignore] = ACTIONS(15),
    [sym_comment_doc] = ACTIONS(17),
    [sym_COMMENT] = ACTIONS(5),
    [sym_NEWLINE] = ACTIONS(3),
    [sym_WHITESPACE] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_RPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [66] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(41), 1,
      anon_sym_PIPE,
    STATE(13), 1,
      sym__prefix_op,
    STATE(84), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [132] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(43), 1,
      anon_sym_RPAREN,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [198] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(45), 1,
      anon_sym_RPAREN,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [264] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(47), 1,
      anon_sym_PIPE,
    STATE(13), 1,
      sym__prefix_op,
    STATE(80), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [330] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(49), 1,
      anon_sym_RPAREN,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [396] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(51), 1,
      anon_sym_RPAREN,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [462] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(53), 1,
      anon_sym_RPAREN,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [528] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(55), 1,
      anon_sym_RPAREN,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [594] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(59), 1,
      anon_sym_LBRACK,
    ACTIONS(61), 1,
      anon_sym_PIPE,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_CARET,
    ACTIONS(67), 1,
      anon_sym_AT,
    ACTIONS(69), 1,
      anon_sym_SQUOTE,
    ACTIONS(71), 1,
      anon_sym_DQUOTE,
    ACTIONS(73), 1,
      anon_sym_SLASH,
    ACTIONS(75), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(77), 1,
      anon_sym_BSLASHp,
    STATE(14), 1,
      sym__prefix_op,
    STATE(36), 1,
      sym_expression,
    STATE(180), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(57), 2,
      sym_id,
      sym_unsigned,
    STATE(48), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(47), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [660] = 18,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    ACTIONS(79), 1,
      anon_sym_RPAREN,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [726] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(68), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [789] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(59), 1,
      anon_sym_LBRACK,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_CARET,
    ACTIONS(67), 1,
      anon_sym_AT,
    ACTIONS(69), 1,
      anon_sym_SQUOTE,
    ACTIONS(71), 1,
      anon_sym_DQUOTE,
    ACTIONS(73), 1,
      anon_sym_SLASH,
    ACTIONS(75), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(77), 1,
      anon_sym_BSLASHp,
    STATE(14), 1,
      sym__prefix_op,
    STATE(32), 1,
      sym_expression,
    STATE(180), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(57), 2,
      sym_id,
      sym_unsigned,
    STATE(48), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(47), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [852] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(59), 1,
      anon_sym_LBRACK,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_CARET,
    ACTIONS(67), 1,
      anon_sym_AT,
    ACTIONS(69), 1,
      anon_sym_SQUOTE,
    ACTIONS(71), 1,
      anon_sym_DQUOTE,
    ACTIONS(73), 1,
      anon_sym_SLASH,
    ACTIONS(75), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(77), 1,
      anon_sym_BSLASHp,
    STATE(14), 1,
      sym__prefix_op,
    STATE(33), 1,
      sym_expression,
    STATE(51), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(57), 2,
      sym_id,
      sym_unsigned,
    STATE(48), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(47), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [915] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(64), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [978] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(67), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1041] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(59), 1,
      anon_sym_LBRACK,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_CARET,
    ACTIONS(67), 1,
      anon_sym_AT,
    ACTIONS(69), 1,
      anon_sym_SQUOTE,
    ACTIONS(71), 1,
      anon_sym_DQUOTE,
    ACTIONS(73), 1,
      anon_sym_SLASH,
    ACTIONS(75), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(77), 1,
      anon_sym_BSLASHp,
    STATE(14), 1,
      sym__prefix_op,
    STATE(37), 1,
      sym_expression,
    STATE(180), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(57), 2,
      sym_id,
      sym_unsigned,
    STATE(48), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(47), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1104] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(66), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1167] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(59), 1,
      anon_sym_LBRACK,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_CARET,
    ACTIONS(67), 1,
      anon_sym_AT,
    ACTIONS(69), 1,
      anon_sym_SQUOTE,
    ACTIONS(71), 1,
      anon_sym_DQUOTE,
    ACTIONS(73), 1,
      anon_sym_SLASH,
    ACTIONS(75), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(77), 1,
      anon_sym_BSLASHp,
    STATE(14), 1,
      sym__prefix_op,
    STATE(35), 1,
      sym_expression,
    STATE(180), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(57), 2,
      sym_id,
      sym_unsigned,
    STATE(48), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(47), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1230] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(59), 1,
      anon_sym_LBRACK,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_CARET,
    ACTIONS(67), 1,
      anon_sym_AT,
    ACTIONS(69), 1,
      anon_sym_SQUOTE,
    ACTIONS(71), 1,
      anon_sym_DQUOTE,
    ACTIONS(73), 1,
      anon_sym_SLASH,
    ACTIONS(75), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(77), 1,
      anon_sym_BSLASHp,
    STATE(14), 1,
      sym__prefix_op,
    STATE(30), 1,
      sym_expression,
    STATE(180), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(57), 2,
      sym_id,
      sym_unsigned,
    STATE(48), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(47), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1293] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(71), 1,
      sym_expression,
    STATE(92), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1356] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(88), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1419] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(73), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1482] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(63), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1545] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(61), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1608] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(76), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1671] = 17,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(21), 1,
      anon_sym_LBRACK,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_CARET,
    ACTIONS(29), 1,
      anon_sym_AT,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_SLASH,
    ACTIONS(37), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(39), 1,
      anon_sym_BSLASHp,
    STATE(13), 1,
      sym__prefix_op,
    STATE(72), 1,
      sym_expression,
    STATE(184), 1,
      sym_choice_tag,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(19), 2,
      sym_id,
      sym_unsigned,
    STATE(77), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(81), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1734] = 6,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(83), 1,
      sym_id,
    ACTIONS(85), 1,
      aux_sym_choice_tag_token1,
    ACTIONS(87), 1,
      anon_sym_COLON,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(81), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [1766] = 11,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(91), 1,
      sym_id,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(97), 1,
      anon_sym_TILDE,
    ACTIONS(99), 1,
      anon_sym_LT_DASH,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(103), 1,
      sym_eos,
    STATE(42), 1,
      sym__suffix_op,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(95), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(89), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [1807] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(107), 1,
      sym_id,
    ACTIONS(109), 1,
      aux_sym_regex_long_token1,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(105), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [1836] = 6,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(113), 1,
      sym_id,
    STATE(42), 1,
      sym__suffix_op,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(95), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(111), 10,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [1867] = 6,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(115), 1,
      sym_id,
    STATE(42), 1,
      sym__suffix_op,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(95), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(93), 10,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [1898] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(119), 1,
      sym_id,
    ACTIONS(121), 1,
      anon_sym_COLON,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(117), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [1927] = 8,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(125), 1,
      sym_id,
    STATE(42), 1,
      sym__suffix_op,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(95), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(123), 8,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      sym_eos,
      sym_comment_doc,
  [1962] = 11,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(97), 1,
      anon_sym_TILDE,
    ACTIONS(99), 1,
      anon_sym_LT_DASH,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(129), 1,
      sym_id,
    ACTIONS(131), 1,
      sym_eos,
    STATE(42), 1,
      sym__suffix_op,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(95), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(127), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [2003] = 9,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(97), 1,
      anon_sym_TILDE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(135), 1,
      sym_id,
    STATE(42), 1,
      sym__suffix_op,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(95), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(133), 7,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_LT_DASH,
      sym_eos,
      sym_comment_doc,
  [2040] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(139), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(137), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2066] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(143), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(141), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2092] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(147), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(145), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2118] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(151), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(149), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2144] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(155), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(153), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2170] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(159), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(157), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2196] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(163), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(161), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2222] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(167), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(165), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2248] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(171), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(169), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2274] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(175), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(173), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2300] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(179), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(177), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2326] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(183), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(181), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2352] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(187), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(185), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2378] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(191), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(189), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2404] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(195), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(193), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2430] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(199), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(197), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2456] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(203), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(201), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2482] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(207), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(205), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2508] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(211), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(209), 13,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
      sym_comment_doc,
  [2534] = 10,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_SQUOTE,
    ACTIONS(71), 1,
      anon_sym_DQUOTE,
    ACTIONS(215), 1,
      sym_id,
    ACTIONS(217), 1,
      anon_sym_LBRACE,
    ACTIONS(219), 1,
      anon_sym_LBRACK,
    ACTIONS(221), 1,
      sym_eos,
    STATE(110), 1,
      sym_string,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(213), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [2570] = 11,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(9), 1,
      sym_id,
    ACTIONS(11), 1,
      sym_grammar,
    ACTIONS(13), 1,
      sym_fragment,
    ACTIONS(15), 1,
      sym_ignore,
    ACTIONS(17), 1,
      sym_comment_doc,
    ACTIONS(223), 1,
      ts_builtin_sym_end,
    STATE(59), 1,
      aux_sym_program_repeat1,
    STATE(128), 1,
      sym_statement,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    STATE(123), 4,
      sym_grammar_statement,
      sym_fragment_statement,
      sym_ignore_statement,
      sym_assign_statement,
  [2608] = 11,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(225), 1,
      ts_builtin_sym_end,
    ACTIONS(227), 1,
      sym_id,
    ACTIONS(230), 1,
      sym_grammar,
    ACTIONS(233), 1,
      sym_fragment,
    ACTIONS(236), 1,
      sym_ignore,
    ACTIONS(239), 1,
      sym_comment_doc,
    STATE(59), 1,
      aux_sym_program_repeat1,
    STATE(128), 1,
      sym_statement,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    STATE(123), 4,
      sym_grammar_statement,
      sym_fragment_statement,
      sym_ignore_statement,
      sym_assign_statement,
  [2646] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(244), 2,
      anon_sym_LBRACK,
      anon_sym_SLASH,
    ACTIONS(242), 10,
      anon_sym_PIPE,
      anon_sym_LPAREN,
      anon_sym_CARET,
      anon_sym_AT,
      sym_id,
      sym_unsigned,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK_CARET,
      anon_sym_BSLASHp,
  [2670] = 11,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(246), 1,
      anon_sym_COMMA,
    ACTIONS(248), 1,
      anon_sym_RPAREN,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    STATE(79), 1,
      sym__suffix_op,
    STATE(156), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2707] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(209), 11,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [2728] = 11,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    ACTIONS(256), 1,
      anon_sym_COMMA,
    ACTIONS(258), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      sym__suffix_op,
    STATE(157), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2765] = 11,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    ACTIONS(260), 1,
      anon_sym_COMMA,
    ACTIONS(262), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      sym__suffix_op,
    STATE(165), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2802] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(264), 1,
      aux_sym_choice_tag_token1,
    ACTIONS(266), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(81), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [2827] = 11,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    ACTIONS(268), 1,
      anon_sym_COMMA,
    ACTIONS(270), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      sym__suffix_op,
    STATE(168), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2864] = 8,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(133), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LT_DASH,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2894] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(111), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [2918] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(272), 1,
      aux_sym_regex_long_token1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(105), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [2940] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(274), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(117), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [2962] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(93), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [2986] = 9,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(276), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [3018] = 7,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(123), 4,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
  [3046] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(201), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3065] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(141), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3084] = 9,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    ACTIONS(278), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [3115] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(177), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3134] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(193), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3153] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(153), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3172] = 9,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    ACTIONS(280), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [3203] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(173), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3222] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(165), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3241] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(181), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3260] = 9,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    ACTIONS(282), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [3291] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(205), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3310] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(145), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3329] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(137), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3348] = 9,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(93), 1,
      anon_sym_PIPE,
    ACTIONS(101), 1,
      anon_sym_POUND,
    ACTIONS(252), 1,
      anon_sym_TILDE,
    ACTIONS(254), 1,
      anon_sym_LT_DASH,
    ACTIONS(284), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      sym__suffix_op,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(250), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [3379] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(185), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3398] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(157), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3417] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(161), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3436] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(189), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3455] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(169), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3474] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(197), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3493] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(149), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [3512] = 8,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(286), 1,
      anon_sym_RBRACK,
    ACTIONS(288), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(290), 1,
      anon_sym_BSLASHp,
    STATE(102), 1,
      aux_sym_regex_range_repeat1,
    STATE(153), 1,
      sym_regex_range_item,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(143), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [3539] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(294), 1,
      sym_id,
    ACTIONS(296), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(292), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3560] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(300), 1,
      sym_id,
    ACTIONS(302), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(298), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3581] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(306), 1,
      sym_id,
    ACTIONS(308), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(304), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3602] = 8,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(288), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(290), 1,
      anon_sym_BSLASHp,
    ACTIONS(310), 1,
      anon_sym_RBRACK,
    STATE(96), 1,
      aux_sym_regex_range_repeat1,
    STATE(153), 1,
      sym_regex_range_item,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(143), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [3629] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(314), 1,
      sym_id,
    ACTIONS(316), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(312), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3650] = 8,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(318), 1,
      anon_sym_RBRACK,
    ACTIONS(320), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(323), 1,
      anon_sym_BSLASHp,
    STATE(102), 1,
      aux_sym_regex_range_repeat1,
    STATE(153), 1,
      sym_regex_range_item,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(143), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [3677] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(328), 1,
      sym_id,
    ACTIONS(330), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(326), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3698] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(334), 1,
      sym_id,
    ACTIONS(336), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(332), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3719] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(328), 1,
      sym_id,
    ACTIONS(338), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(326), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3740] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(342), 1,
      sym_id,
    ACTIONS(344), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(340), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3761] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(348), 1,
      sym_id,
    ACTIONS(350), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(346), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3782] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(348), 1,
      sym_id,
    ACTIONS(352), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(346), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3803] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(356), 1,
      sym_id,
    ACTIONS(358), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(354), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3824] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(362), 1,
      sym_id,
    ACTIONS(364), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(360), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3845] = 8,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(288), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(290), 1,
      anon_sym_BSLASHp,
    ACTIONS(366), 1,
      anon_sym_RBRACK,
    STATE(112), 1,
      aux_sym_regex_range_repeat1,
    STATE(153), 1,
      sym_regex_range_item,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(143), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [3872] = 8,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(288), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(290), 1,
      anon_sym_BSLASHp,
    ACTIONS(368), 1,
      anon_sym_RBRACK,
    STATE(102), 1,
      aux_sym_regex_range_repeat1,
    STATE(153), 1,
      sym_regex_range_item,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(143), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [3899] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(372), 1,
      sym_id,
    ACTIONS(374), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(370), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3920] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(372), 1,
      sym_id,
    ACTIONS(376), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(370), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3941] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(380), 1,
      sym_id,
    ACTIONS(382), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(378), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3962] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(356), 1,
      sym_id,
    ACTIONS(384), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(354), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [3983] = 5,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(388), 1,
      sym_id,
    ACTIONS(390), 1,
      sym_eos,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(386), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4004] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(394), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(392), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4022] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(398), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(396), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4040] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(402), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(400), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4058] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(406), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(404), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4076] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(372), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(370), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4094] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(410), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(408), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4112] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(348), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(346), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4130] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(372), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(370), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4148] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(414), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(412), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4166] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(414), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(412), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4184] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(418), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(416), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4202] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(300), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(298), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4220] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(422), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(420), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4238] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(426), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(424), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4256] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(430), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(428), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4274] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(348), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(346), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4292] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(434), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(432), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4310] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(434), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(432), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4328] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(438), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(436), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4346] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(442), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(440), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4364] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(446), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(444), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4382] = 4,
    ACTIONS(3), 1,
      sym_NEWLINE,
    ACTIONS(306), 1,
      sym_id,
    ACTIONS(5), 2,
      sym_COMMENT,
      sym_WHITESPACE,
    ACTIONS(304), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      sym_comment_doc,
  [4400] = 6,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(288), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(290), 1,
      anon_sym_BSLASHp,
    STATE(150), 1,
      sym_regex_range_item,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(143), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [4421] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    STATE(11), 1,
      sym_eq,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(448), 4,
      anon_sym_EQ,
      anon_sym__EQ,
      anon_sym_AT_EQ,
      anon_sym_CARET_EQ,
  [4438] = 6,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(450), 1,
      anon_sym_RBRACE,
    STATE(155), 1,
      sym_string,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4458] = 4,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(454), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(452), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [4474] = 6,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(456), 1,
      anon_sym_RBRACE,
    STATE(163), 1,
      sym_string,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4494] = 6,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(450), 1,
      anon_sym_RBRACK,
    STATE(170), 1,
      sym_string,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4514] = 6,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(458), 1,
      anon_sym_RBRACK,
    STATE(163), 1,
      sym_string,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4534] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(460), 1,
      anon_sym_COMMA,
    STATE(147), 1,
      aux_sym_grammar_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(463), 2,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [4552] = 4,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(167), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(165), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [4568] = 6,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(465), 1,
      anon_sym_RBRACK,
    STATE(163), 1,
      sym_string,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4588] = 4,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(469), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(467), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [4604] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(471), 1,
      anon_sym_COMMA,
    STATE(151), 1,
      aux_sym_ignore_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(474), 2,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [4622] = 6,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(465), 1,
      anon_sym_RBRACE,
    STATE(163), 1,
      sym_string,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4642] = 5,
    ACTIONS(3), 1,
      sym_COMMENT,
    ACTIONS(478), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(480), 1,
      anon_sym_DASH,
    ACTIONS(5), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(476), 2,
      anon_sym_RBRACK,
      anon_sym_BSLASHp,
  [4660] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(482), 1,
      anon_sym_COMMA,
    ACTIONS(484), 1,
      anon_sym_RBRACE,
    STATE(166), 1,
      aux_sym_ignore_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4677] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(486), 1,
      anon_sym_COMMA,
    ACTIONS(488), 1,
      anon_sym_RBRACE,
    STATE(160), 1,
      aux_sym_grammar_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4694] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(45), 1,
      anon_sym_RPAREN,
    ACTIONS(490), 1,
      anon_sym_COMMA,
    STATE(169), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4711] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(55), 1,
      anon_sym_RPAREN,
    ACTIONS(492), 1,
      anon_sym_COMMA,
    STATE(169), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4728] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(494), 3,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [4741] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(496), 1,
      anon_sym_COMMA,
    ACTIONS(498), 1,
      anon_sym_RBRACK,
    STATE(147), 1,
      aux_sym_grammar_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4758] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(500), 1,
      anon_sym_COMMA,
    ACTIONS(502), 1,
      anon_sym_RBRACE,
    STATE(147), 1,
      aux_sym_grammar_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4775] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(484), 1,
      anon_sym_RBRACK,
    ACTIONS(504), 1,
      anon_sym_COMMA,
    STATE(164), 1,
      aux_sym_ignore_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4792] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(506), 1,
      sym_id,
    ACTIONS(508), 1,
      anon_sym_LBRACE,
    ACTIONS(510), 1,
      anon_sym_LBRACK,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4809] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
    ACTIONS(512), 3,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [4822] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(514), 1,
      anon_sym_COMMA,
    ACTIONS(516), 1,
      anon_sym_RBRACK,
    STATE(151), 1,
      aux_sym_ignore_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4839] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(25), 1,
      anon_sym_RPAREN,
    ACTIONS(518), 1,
      anon_sym_COMMA,
    STATE(169), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4856] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(520), 1,
      anon_sym_COMMA,
    ACTIONS(522), 1,
      anon_sym_RBRACE,
    STATE(151), 1,
      aux_sym_ignore_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4873] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(31), 1,
      anon_sym_SQUOTE,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    STATE(163), 1,
      sym_string,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4890] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(51), 1,
      anon_sym_RPAREN,
    ACTIONS(524), 1,
      anon_sym_COMMA,
    STATE(169), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4907] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(276), 1,
      anon_sym_RPAREN,
    ACTIONS(526), 1,
      anon_sym_COMMA,
    STATE(169), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4924] = 5,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(488), 1,
      anon_sym_RBRACK,
    ACTIONS(529), 1,
      anon_sym_COMMA,
    STATE(159), 1,
      aux_sym_grammar_statement_repeat1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4941] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(531), 1,
      anon_sym_LPAREN,
    ACTIONS(533), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4955] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(535), 1,
      sym_id,
    ACTIONS(537), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4969] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(539), 1,
      sym_id,
    ACTIONS(541), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4983] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(541), 1,
      anon_sym_RBRACK,
    ACTIONS(543), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [4997] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(535), 1,
      sym_id,
    ACTIONS(545), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5011] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(535), 1,
      sym_id,
    ACTIONS(537), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5025] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(547), 1,
      anon_sym_LPAREN,
    ACTIONS(549), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5039] = 4,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(535), 1,
      sym_id,
    ACTIONS(551), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5053] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(553), 1,
      anon_sym_SQUOTE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5064] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(555), 1,
      anon_sym_PIPE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5075] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(557), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5086] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(559), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5097] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(561), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5108] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(563), 1,
      anon_sym_PIPE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5119] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(553), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5130] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(565), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5141] = 2,
    ACTIONS(567), 1,
      aux_sym_string_token2,
    ACTIONS(5), 3,
      sym_COMMENT,
      sym_NEWLINE,
      sym_WHITESPACE,
  [5150] = 2,
    ACTIONS(569), 1,
      aux_sym_string_token1,
    ACTIONS(5), 3,
      sym_COMMENT,
      sym_NEWLINE,
      sym_WHITESPACE,
  [5159] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(571), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5170] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(573), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5181] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(575), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5192] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(577), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5203] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(579), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5214] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(581), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5225] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(583), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5236] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(585), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5247] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(587), 1,
      anon_sym_LPAREN,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5258] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(589), 1,
      anon_sym_SQUOTE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5269] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(591), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5280] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(593), 1,
      anon_sym_RBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5291] = 2,
    ACTIONS(595), 1,
      aux_sym_string_token1,
    ACTIONS(5), 3,
      sym_COMMENT,
      sym_NEWLINE,
      sym_WHITESPACE,
  [5300] = 2,
    ACTIONS(597), 1,
      aux_sym_string_token2,
    ACTIONS(5), 3,
      sym_COMMENT,
      sym_NEWLINE,
      sym_WHITESPACE,
  [5309] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(535), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5320] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(589), 1,
      anon_sym_DQUOTE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5331] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(599), 1,
      aux_sym_regex_set_token1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5342] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(601), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5353] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(603), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5364] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(605), 1,
      aux_sym_regex_set_token1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5375] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(607), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5386] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(609), 1,
      aux_sym_regex_set_token1,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5397] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(611), 1,
      anon_sym_LPAREN,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5408] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(613), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5419] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(615), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
  [5430] = 3,
    ACTIONS(5), 1,
      sym_WHITESPACE,
    ACTIONS(617), 1,
      sym_id,
    ACTIONS(3), 2,
      sym_COMMENT,
      sym_NEWLINE,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 66,
  [SMALL_STATE(4)] = 132,
  [SMALL_STATE(5)] = 198,
  [SMALL_STATE(6)] = 264,
  [SMALL_STATE(7)] = 330,
  [SMALL_STATE(8)] = 396,
  [SMALL_STATE(9)] = 462,
  [SMALL_STATE(10)] = 528,
  [SMALL_STATE(11)] = 594,
  [SMALL_STATE(12)] = 660,
  [SMALL_STATE(13)] = 726,
  [SMALL_STATE(14)] = 789,
  [SMALL_STATE(15)] = 852,
  [SMALL_STATE(16)] = 915,
  [SMALL_STATE(17)] = 978,
  [SMALL_STATE(18)] = 1041,
  [SMALL_STATE(19)] = 1104,
  [SMALL_STATE(20)] = 1167,
  [SMALL_STATE(21)] = 1230,
  [SMALL_STATE(22)] = 1293,
  [SMALL_STATE(23)] = 1356,
  [SMALL_STATE(24)] = 1419,
  [SMALL_STATE(25)] = 1482,
  [SMALL_STATE(26)] = 1545,
  [SMALL_STATE(27)] = 1608,
  [SMALL_STATE(28)] = 1671,
  [SMALL_STATE(29)] = 1734,
  [SMALL_STATE(30)] = 1766,
  [SMALL_STATE(31)] = 1807,
  [SMALL_STATE(32)] = 1836,
  [SMALL_STATE(33)] = 1867,
  [SMALL_STATE(34)] = 1898,
  [SMALL_STATE(35)] = 1927,
  [SMALL_STATE(36)] = 1962,
  [SMALL_STATE(37)] = 2003,
  [SMALL_STATE(38)] = 2040,
  [SMALL_STATE(39)] = 2066,
  [SMALL_STATE(40)] = 2092,
  [SMALL_STATE(41)] = 2118,
  [SMALL_STATE(42)] = 2144,
  [SMALL_STATE(43)] = 2170,
  [SMALL_STATE(44)] = 2196,
  [SMALL_STATE(45)] = 2222,
  [SMALL_STATE(46)] = 2248,
  [SMALL_STATE(47)] = 2274,
  [SMALL_STATE(48)] = 2300,
  [SMALL_STATE(49)] = 2326,
  [SMALL_STATE(50)] = 2352,
  [SMALL_STATE(51)] = 2378,
  [SMALL_STATE(52)] = 2404,
  [SMALL_STATE(53)] = 2430,
  [SMALL_STATE(54)] = 2456,
  [SMALL_STATE(55)] = 2482,
  [SMALL_STATE(56)] = 2508,
  [SMALL_STATE(57)] = 2534,
  [SMALL_STATE(58)] = 2570,
  [SMALL_STATE(59)] = 2608,
  [SMALL_STATE(60)] = 2646,
  [SMALL_STATE(61)] = 2670,
  [SMALL_STATE(62)] = 2707,
  [SMALL_STATE(63)] = 2728,
  [SMALL_STATE(64)] = 2765,
  [SMALL_STATE(65)] = 2802,
  [SMALL_STATE(66)] = 2827,
  [SMALL_STATE(67)] = 2864,
  [SMALL_STATE(68)] = 2894,
  [SMALL_STATE(69)] = 2918,
  [SMALL_STATE(70)] = 2940,
  [SMALL_STATE(71)] = 2962,
  [SMALL_STATE(72)] = 2986,
  [SMALL_STATE(73)] = 3018,
  [SMALL_STATE(74)] = 3046,
  [SMALL_STATE(75)] = 3065,
  [SMALL_STATE(76)] = 3084,
  [SMALL_STATE(77)] = 3115,
  [SMALL_STATE(78)] = 3134,
  [SMALL_STATE(79)] = 3153,
  [SMALL_STATE(80)] = 3172,
  [SMALL_STATE(81)] = 3203,
  [SMALL_STATE(82)] = 3222,
  [SMALL_STATE(83)] = 3241,
  [SMALL_STATE(84)] = 3260,
  [SMALL_STATE(85)] = 3291,
  [SMALL_STATE(86)] = 3310,
  [SMALL_STATE(87)] = 3329,
  [SMALL_STATE(88)] = 3348,
  [SMALL_STATE(89)] = 3379,
  [SMALL_STATE(90)] = 3398,
  [SMALL_STATE(91)] = 3417,
  [SMALL_STATE(92)] = 3436,
  [SMALL_STATE(93)] = 3455,
  [SMALL_STATE(94)] = 3474,
  [SMALL_STATE(95)] = 3493,
  [SMALL_STATE(96)] = 3512,
  [SMALL_STATE(97)] = 3539,
  [SMALL_STATE(98)] = 3560,
  [SMALL_STATE(99)] = 3581,
  [SMALL_STATE(100)] = 3602,
  [SMALL_STATE(101)] = 3629,
  [SMALL_STATE(102)] = 3650,
  [SMALL_STATE(103)] = 3677,
  [SMALL_STATE(104)] = 3698,
  [SMALL_STATE(105)] = 3719,
  [SMALL_STATE(106)] = 3740,
  [SMALL_STATE(107)] = 3761,
  [SMALL_STATE(108)] = 3782,
  [SMALL_STATE(109)] = 3803,
  [SMALL_STATE(110)] = 3824,
  [SMALL_STATE(111)] = 3845,
  [SMALL_STATE(112)] = 3872,
  [SMALL_STATE(113)] = 3899,
  [SMALL_STATE(114)] = 3920,
  [SMALL_STATE(115)] = 3941,
  [SMALL_STATE(116)] = 3962,
  [SMALL_STATE(117)] = 3983,
  [SMALL_STATE(118)] = 4004,
  [SMALL_STATE(119)] = 4022,
  [SMALL_STATE(120)] = 4040,
  [SMALL_STATE(121)] = 4058,
  [SMALL_STATE(122)] = 4076,
  [SMALL_STATE(123)] = 4094,
  [SMALL_STATE(124)] = 4112,
  [SMALL_STATE(125)] = 4130,
  [SMALL_STATE(126)] = 4148,
  [SMALL_STATE(127)] = 4166,
  [SMALL_STATE(128)] = 4184,
  [SMALL_STATE(129)] = 4202,
  [SMALL_STATE(130)] = 4220,
  [SMALL_STATE(131)] = 4238,
  [SMALL_STATE(132)] = 4256,
  [SMALL_STATE(133)] = 4274,
  [SMALL_STATE(134)] = 4292,
  [SMALL_STATE(135)] = 4310,
  [SMALL_STATE(136)] = 4328,
  [SMALL_STATE(137)] = 4346,
  [SMALL_STATE(138)] = 4364,
  [SMALL_STATE(139)] = 4382,
  [SMALL_STATE(140)] = 4400,
  [SMALL_STATE(141)] = 4421,
  [SMALL_STATE(142)] = 4438,
  [SMALL_STATE(143)] = 4458,
  [SMALL_STATE(144)] = 4474,
  [SMALL_STATE(145)] = 4494,
  [SMALL_STATE(146)] = 4514,
  [SMALL_STATE(147)] = 4534,
  [SMALL_STATE(148)] = 4552,
  [SMALL_STATE(149)] = 4568,
  [SMALL_STATE(150)] = 4588,
  [SMALL_STATE(151)] = 4604,
  [SMALL_STATE(152)] = 4622,
  [SMALL_STATE(153)] = 4642,
  [SMALL_STATE(154)] = 4660,
  [SMALL_STATE(155)] = 4677,
  [SMALL_STATE(156)] = 4694,
  [SMALL_STATE(157)] = 4711,
  [SMALL_STATE(158)] = 4728,
  [SMALL_STATE(159)] = 4741,
  [SMALL_STATE(160)] = 4758,
  [SMALL_STATE(161)] = 4775,
  [SMALL_STATE(162)] = 4792,
  [SMALL_STATE(163)] = 4809,
  [SMALL_STATE(164)] = 4822,
  [SMALL_STATE(165)] = 4839,
  [SMALL_STATE(166)] = 4856,
  [SMALL_STATE(167)] = 4873,
  [SMALL_STATE(168)] = 4890,
  [SMALL_STATE(169)] = 4907,
  [SMALL_STATE(170)] = 4924,
  [SMALL_STATE(171)] = 4941,
  [SMALL_STATE(172)] = 4955,
  [SMALL_STATE(173)] = 4969,
  [SMALL_STATE(174)] = 4983,
  [SMALL_STATE(175)] = 4997,
  [SMALL_STATE(176)] = 5011,
  [SMALL_STATE(177)] = 5025,
  [SMALL_STATE(178)] = 5039,
  [SMALL_STATE(179)] = 5053,
  [SMALL_STATE(180)] = 5064,
  [SMALL_STATE(181)] = 5075,
  [SMALL_STATE(182)] = 5086,
  [SMALL_STATE(183)] = 5097,
  [SMALL_STATE(184)] = 5108,
  [SMALL_STATE(185)] = 5119,
  [SMALL_STATE(186)] = 5130,
  [SMALL_STATE(187)] = 5141,
  [SMALL_STATE(188)] = 5150,
  [SMALL_STATE(189)] = 5159,
  [SMALL_STATE(190)] = 5170,
  [SMALL_STATE(191)] = 5181,
  [SMALL_STATE(192)] = 5192,
  [SMALL_STATE(193)] = 5203,
  [SMALL_STATE(194)] = 5214,
  [SMALL_STATE(195)] = 5225,
  [SMALL_STATE(196)] = 5236,
  [SMALL_STATE(197)] = 5247,
  [SMALL_STATE(198)] = 5258,
  [SMALL_STATE(199)] = 5269,
  [SMALL_STATE(200)] = 5280,
  [SMALL_STATE(201)] = 5291,
  [SMALL_STATE(202)] = 5300,
  [SMALL_STATE(203)] = 5309,
  [SMALL_STATE(204)] = 5320,
  [SMALL_STATE(205)] = 5331,
  [SMALL_STATE(206)] = 5342,
  [SMALL_STATE(207)] = 5353,
  [SMALL_STATE(208)] = 5364,
  [SMALL_STATE(209)] = 5375,
  [SMALL_STATE(210)] = 5386,
  [SMALL_STATE(211)] = 5397,
  [SMALL_STATE(212)] = 5408,
  [SMALL_STATE(213)] = 5419,
  [SMALL_STATE(214)] = 5430,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 0),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(141),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(183),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [59] = {.entry = {.count = 1, .reusable = false}}, SHIFT(100),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(193),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 3, .production_id = 19),
  [83] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 3, .production_id = 19),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 4, .production_id = 11),
  [91] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 4, .production_id = 11),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 1, .production_id = 7),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_long, 2),
  [107] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_long, 2),
  [109] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_prefix, 2, .production_id = 13),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_prefix, 2, .production_id = 13),
  [115] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 1, .production_id = 7),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 4, .production_id = 24),
  [119] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 4, .production_id = 24),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_concat_expression, 3, .production_id = 18),
  [125] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_concat_expression, 3, .production_id = 18),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 3, .production_id = 8),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 3, .production_id = 8),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_expr, 3, .production_id = 18),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_expr, 3, .production_id = 18),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range, 3, .production_id = 10),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range, 3, .production_id = 10),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 8, .production_id = 28),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 8, .production_id = 28),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 5, .production_id = 26),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 5, .production_id = 26),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 5, .production_id = 25),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 5, .production_id = 25),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_suffix, 2, .production_id = 12),
  [155] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_suffix, 2, .production_id = 12),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_long, 3),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_long, 3),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 7, .production_id = 25),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 7, .production_id = 25),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_set, 4, .production_id = 23),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_set, 4, .production_id = 23),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 6, .production_id = 27),
  [171] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 6, .production_id = 27),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1),
  [175] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 1),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data, 1),
  [179] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_data, 1),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 9, .production_id = 28),
  [183] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 9, .production_id = 28),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 3),
  [187] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 3),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_expression, 3, .production_id = 18),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_expression, 3, .production_id = 18),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range, 2, .production_id = 10),
  [195] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range, 2, .production_id = 10),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 4),
  [199] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 4),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 7, .production_id = 28),
  [203] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 7, .production_id = 28),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 6, .production_id = 25),
  [207] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 6, .production_id = 25),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, .production_id = 15),
  [211] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, .production_id = 15),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 2, .production_id = 3),
  [215] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 2, .production_id = 3),
  [217] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [221] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 1, .production_id = 2),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5),
  [227] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(141),
  [230] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(206),
  [233] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(207),
  [236] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(162),
  [239] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(123),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_eq, 1),
  [244] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_eq, 1),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [276] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_macro_call_repeat1, 2),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [288] = {.entry = {.count = 1, .reusable = false}}, SHIFT(143),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [292] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 4, .production_id = 3),
  [294] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 4, .production_id = 3),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [298] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 5, .production_id = 9),
  [300] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 5, .production_id = 9),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [304] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 6, .production_id = 14),
  [306] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 6, .production_id = 14),
  [308] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [310] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [312] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 4, .production_id = 9),
  [314] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 4, .production_id = 9),
  [316] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [318] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 2),
  [320] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_regex_range_repeat1, 2), SHIFT_REPEAT(143),
  [323] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 2), SHIFT_REPEAT(209),
  [326] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 6, .production_id = 21),
  [328] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 6, .production_id = 21),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [332] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 5, .production_id = 14),
  [334] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 5, .production_id = 14),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 3),
  [342] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 3),
  [344] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [346] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 6, .production_id = 16),
  [348] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 6, .production_id = 16),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [352] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [354] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 5, .production_id = 16),
  [356] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 5, .production_id = 16),
  [358] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [360] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 3, .production_id = 6),
  [362] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 3, .production_id = 6),
  [364] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [366] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [368] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [370] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 7, .production_id = 21),
  [372] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 7, .production_id = 21),
  [374] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [376] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [378] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment_statement, 2, .production_id = 3),
  [380] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fragment_statement, 2, .production_id = 3),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [386] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 2, .production_id = 4),
  [388] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 2, .production_id = 4),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [392] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 6, .production_id = 9),
  [394] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 6, .production_id = 9),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 3, .production_id = 3),
  [398] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 3, .production_id = 3),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 7, .production_id = 14),
  [402] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 7, .production_id = 14),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 5, .production_id = 3),
  [406] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 5, .production_id = 3),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 1),
  [410] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 1),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 7, .production_id = 16),
  [414] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 7, .production_id = 16),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 1, .production_id = 1),
  [418] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_program_repeat1, 1, .production_id = 1),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment_statement, 3, .production_id = 3),
  [422] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fragment_statement, 3, .production_id = 3),
  [424] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 3, .production_id = 4),
  [426] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 3, .production_id = 4),
  [428] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 4, .production_id = 6),
  [430] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 4, .production_id = 6),
  [432] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 8, .production_id = 21),
  [434] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 8, .production_id = 21),
  [436] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 4),
  [438] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 4),
  [440] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 4, .production_id = 8),
  [442] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 4, .production_id = 8),
  [444] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 5, .production_id = 11),
  [446] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 5, .production_id = 11),
  [448] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [450] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [452] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range_item, 1),
  [454] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range_item, 1),
  [456] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [458] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [460] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_grammar_statement_repeat1, 2, .production_id = 22), SHIFT_REPEAT(167),
  [463] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_grammar_statement_repeat1, 2, .production_id = 22),
  [465] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [467] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range_item_group, 3, .production_id = 18),
  [469] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range_item_group, 3, .production_id = 18),
  [471] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ignore_statement_repeat1, 2, .production_id = 17), SHIFT_REPEAT(203),
  [474] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ignore_statement_repeat1, 2, .production_id = 17),
  [476] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 1),
  [478] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_regex_range_repeat1, 1),
  [480] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [482] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [484] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [486] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [488] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [490] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [492] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [494] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ignore_statement_repeat1, 2, .production_id = 4),
  [496] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [498] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [500] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [502] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [504] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [508] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [510] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [512] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_grammar_statement_repeat1, 2, .production_id = 20),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [516] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [518] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [520] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [522] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [524] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [526] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_macro_call_repeat1, 2), SHIFT_REPEAT(28),
  [529] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [531] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [533] = {.entry = {.count = 1, .reusable = true}}, SHIFT(214),
  [535] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [537] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [539] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [541] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [543] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [547] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [549] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [551] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [553] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [555] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [557] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [559] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [561] = {.entry = {.count = 1, .reusable = false}}, SHIFT(69),
  [563] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [565] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [567] = {.entry = {.count = 1, .reusable = false}}, SHIFT(204),
  [569] = {.entry = {.count = 1, .reusable = false}}, SHIFT(198),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [573] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [575] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [577] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [579] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [583] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [585] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [587] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [589] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [591] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [593] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [595] = {.entry = {.count = 1, .reusable = false}}, SHIFT(179),
  [597] = {.entry = {.count = 1, .reusable = false}}, SHIFT(185),
  [599] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [601] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [603] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [609] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [613] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [615] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [617] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_ygg(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_id,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
