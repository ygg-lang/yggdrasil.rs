#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 162
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 73
#define ALIAS_COUNT 0
#define TOKEN_COUNT 44
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 19
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 28

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
  sym_NEWLINE = 42,
  sym_WHITESPACE = 43,
  sym_program = 44,
  sym_statement = 45,
  sym_grammar_statement = 46,
  sym_fragment_statement = 47,
  sym_ignore_statement = 48,
  sym_assign_statement = 49,
  sym_eq = 50,
  sym_expression = 51,
  sym_unary_prefix = 52,
  sym_unary_suffix = 53,
  sym__prefix_op = 54,
  sym__suffix_op = 55,
  sym_concat_expression = 56,
  sym_choice_expression = 57,
  sym_field_expr = 58,
  sym_data = 59,
  sym_choice_tag = 60,
  sym_macro_call = 61,
  sym_string = 62,
  sym_regex_long = 63,
  sym_regex_range = 64,
  sym_regex_range_item = 65,
  sym_regex_range_item_group = 66,
  sym_regex_set = 67,
  aux_sym_program_repeat1 = 68,
  aux_sym_grammar_statement_repeat1 = 69,
  aux_sym_ignore_statement_repeat1 = 70,
  aux_sym_macro_call_repeat1 = 71,
  aux_sym_regex_range_repeat1 = 72,
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
  field_ty = 19,
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
  [15] = {.index = 23, .length = 2},
  [16] = {.index = 25, .length = 2},
  [17] = {.index = 27, .length = 3},
  [18] = {.index = 30, .length = 2},
  [19] = {.index = 32, .length = 1},
  [20] = {.index = 33, .length = 3},
  [21] = {.index = 36, .length = 2},
  [22] = {.index = 38, .length = 1},
  [23] = {.index = 39, .length = 3},
  [24] = {.index = 42, .length = 1},
  [25] = {.index = 43, .length = 3},
  [26] = {.index = 46, .length = 4},
  [27] = {.index = 50, .length = 2},
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
    {field_item, 2},
    {field_item, 3, .inherited = true},
  [25] =
    {field_item, 0, .inherited = true},
    {field_item, 1, .inherited = true},
  [27] =
    {field_lhs, 0},
    {field_op, 1},
    {field_rhs, 2},
  [30] =
    {field_expression, 0},
    {field_tag, 2},
  [32] =
    {field_ext, 1},
  [33] =
    {field_ext, 3},
    {field_ext, 4, .inherited = true},
    {field_id, 1},
  [36] =
    {field_ext, 0, .inherited = true},
    {field_ext, 1, .inherited = true},
  [38] =
    {field_set, 2},
  [39] =
    {field_expression, 0},
    {field_mode, 3},
    {field_tag, 2},
  [42] =
    {field_name, 1},
  [43] =
    {field_expression, 0},
    {field_tag, 2},
    {field_ty, 4},
  [46] =
    {field_expression, 0},
    {field_mode, 3},
    {field_tag, 2},
    {field_ty, 5},
  [50] =
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
      if (eof) ADVANCE(15);
      if (lookahead == '\n') ADVANCE(81);
      if (lookahead == '\r') ADVANCE(80);
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '"') ADVANCE(73);
      if (lookahead == '#') ADVANCE(39);
      if (lookahead == '\'') ADVANCE(69);
      if (lookahead == '(') ADVANCE(30);
      if (lookahead == ')') ADVANCE(31);
      if (lookahead == '*') ADVANCE(35);
      if (lookahead == '+') ADVANCE(36);
      if (lookahead == ',') ADVANCE(17);
      if (lookahead == '-') ADVANCE(84);
      if (lookahead == '.') ADVANCE(44);
      if (lookahead == '/') ADVANCE(77);
      if (lookahead == '0') ADVANCE(67);
      if (lookahead == ':') ADVANCE(41);
      if (lookahead == ';') ADVANCE(87);
      if (lookahead == '<') ADVANCE(82);
      if (lookahead == '=') ADVANCE(26);
      if (lookahead == '?') ADVANCE(34);
      if (lookahead == '@') ADVANCE(43);
      if (lookahead == '[') ADVANCE(20);
      if (lookahead == '\\') ADVANCE(83);
      if (lookahead == ']') ADVANCE(21);
      if (lookahead == '^') ADVANCE(33);
      if (lookahead == 'f') ADVANCE(61);
      if (lookahead == 'g') ADVANCE(62);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '{') ADVANCE(16);
      if (lookahead == '|') ADVANCE(25);
      if (lookahead == '}') ADVANCE(18);
      if (lookahead == '~') ADVANCE(37);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(80);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (sym_id_character_set_1(lookahead)) ADVANCE(66);
      if (lookahead != 0) ADVANCE(80);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(81);
      if (lookahead == '\r') ADVANCE(80);
      if (lookahead == '-') ADVANCE(84);
      if (lookahead == '\\') ADVANCE(83);
      if (lookahead == ']') ADVANCE(21);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(80);
      if (lookahead != 0) ADVANCE(80);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(81);
      if (lookahead == '\r') ADVANCE(80);
      if (lookahead == '\\') ADVANCE(83);
      if (lookahead == ']') ADVANCE(21);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(80);
      if (lookahead != 0) ADVANCE(80);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(90);
      if (lookahead == '\r') ADVANCE(88);
      if (lookahead == '"') ADVANCE(73);
      if (lookahead == '\'') ADVANCE(69);
      if (lookahead == '(') ADVANCE(30);
      if (lookahead == ')') ADVANCE(31);
      if (lookahead == '/') ADVANCE(77);
      if (lookahead == '0') ADVANCE(67);
      if (lookahead == '@') ADVANCE(42);
      if (lookahead == '[') ADVANCE(20);
      if (lookahead == '\\') ADVANCE(11);
      if (lookahead == ']') ADVANCE(21);
      if (lookahead == '^') ADVANCE(32);
      if (lookahead == '|') ADVANCE(25);
      if (lookahead == '}') ADVANCE(18);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(89);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (sym_id_character_set_1(lookahead)) ADVANCE(66);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(90);
      if (lookahead == '\r') ADVANCE(88);
      if (lookahead == '#') ADVANCE(39);
      if (lookahead == ')') ADVANCE(31);
      if (lookahead == '*') ADVANCE(35);
      if (lookahead == '+') ADVANCE(36);
      if (lookahead == ',') ADVANCE(17);
      if (lookahead == '<') ADVANCE(7);
      if (lookahead == '=') ADVANCE(26);
      if (lookahead == '?') ADVANCE(34);
      if (lookahead == '@') ADVANCE(8);
      if (lookahead == '^') ADVANCE(9);
      if (lookahead == '_') ADVANCE(10);
      if (lookahead == 'g' ||
          lookahead == 'i') ADVANCE(78);
      if (lookahead == '|') ADVANCE(25);
      if (lookahead == '~') ADVANCE(37);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(89);
      END_STATE();
    case 5:
      if (lookahead == '\n') ADVANCE(90);
      if (lookahead == '\r') ADVANCE(88);
      if (lookahead == '[') ADVANCE(19);
      if (lookahead == '{') ADVANCE(16);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(89);
      if (sym_id_character_set_2(lookahead)) ADVANCE(66);
      END_STATE();
    case 6:
      if (lookahead == '\n') ADVANCE(90);
      if (lookahead == '\r') ADVANCE(88);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(89);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(86);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(38);
      END_STATE();
    case 8:
      if (lookahead == '=') ADVANCE(28);
      END_STATE();
    case 9:
      if (lookahead == '=') ADVANCE(29);
      END_STATE();
    case 10:
      if (lookahead == '=') ADVANCE(27);
      END_STATE();
    case 11:
      if (lookahead == 'p') ADVANCE(85);
      END_STATE();
    case 12:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(76);
      END_STATE();
    case 13:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(72);
      END_STATE();
    case 14:
      if (eof) ADVANCE(15);
      if (lookahead == '\n') ADVANCE(90);
      if (lookahead == '\r') ADVANCE(88);
      if (lookahead == '"') ADVANCE(73);
      if (lookahead == '#') ADVANCE(39);
      if (lookahead == '\'') ADVANCE(69);
      if (lookahead == '(') ADVANCE(30);
      if (lookahead == ')') ADVANCE(31);
      if (lookahead == '*') ADVANCE(35);
      if (lookahead == '+') ADVANCE(36);
      if (lookahead == ',') ADVANCE(17);
      if (lookahead == '.') ADVANCE(44);
      if (lookahead == '/') ADVANCE(77);
      if (lookahead == ':') ADVANCE(41);
      if (lookahead == ';') ADVANCE(87);
      if (lookahead == '<') ADVANCE(7);
      if (lookahead == '?') ADVANCE(34);
      if (lookahead == '[') ADVANCE(19);
      if (lookahead == ']') ADVANCE(21);
      if (lookahead == 'f') ADVANCE(61);
      if (lookahead == 'g') ADVANCE(62);
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '{') ADVANCE(16);
      if (lookahead == '|') ADVANCE(25);
      if (lookahead == '}') ADVANCE(18);
      if (lookahead == '~') ADVANCE(37);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(89);
      if (lookahead == '!' ||
          lookahead == '^') ADVANCE(40);
      if (sym_id_character_set_2(lookahead)) ADVANCE(66);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      if (lookahead == '^') ADVANCE(79);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_grammar);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_fragment);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_ignore);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym__EQ);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_AT_EQ);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_CARET_EQ);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_CARET);
      if (lookahead == '=') ADVANCE(29);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_QMARK);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_LT_DASH);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(aux_sym_choice_tag_token1);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead == '=') ADVANCE(28);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == '!') ADVANCE(24);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == '!') ADVANCE(22);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == '!') ADVANCE(23);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(57);
      if (sym_id_character_set_4(lookahead)) ADVANCE(66);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(54);
      if (sym_id_character_set_4(lookahead)) ADVANCE(66);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(64);
      if (sym_id_character_set_4(lookahead)) ADVANCE(66);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'e') ADVANCE(45);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'e') ADVANCE(59);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'g') ADVANCE(58);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'g') ADVANCE(55);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(52);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(50);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(56);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'n') ADVANCE(60);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'n') ADVANCE(65);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'o') ADVANCE(63);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(49);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(48);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(51);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(46);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 't') ADVANCE(47);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(sym_id);
      if (sym_id_character_set_3(lookahead)) ADVANCE(66);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_unsigned);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_unsigned);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(68);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\n') ADVANCE(71);
      if (lookahead == '\r') ADVANCE(72);
      if (lookahead == '\\') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(72);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(72);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\r') ADVANCE(72);
      if (lookahead == '\\') ADVANCE(13);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(72);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\\') ADVANCE(13);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(72);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\n') ADVANCE(75);
      if (lookahead == '\r') ADVANCE(76);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(76);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(76);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\r') ADVANCE(76);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(76);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(76);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(aux_sym_regex_long_token1);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(anon_sym_LBRACK_CARET);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == '\r') ADVANCE(88);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == '-') ADVANCE(38);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == 'p') ADVANCE(85);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_BSLASHp);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(aux_sym_regex_set_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(86);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(sym_eos);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_NEWLINE);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(sym_WHITESPACE);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(sym_WHITESPACE);
      if (lookahead == '\r') ADVANCE(88);
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
  [1] = {.lex_state = 14},
  [2] = {.lex_state = 3},
  [3] = {.lex_state = 3},
  [4] = {.lex_state = 3},
  [5] = {.lex_state = 3},
  [6] = {.lex_state = 3},
  [7] = {.lex_state = 3},
  [8] = {.lex_state = 3},
  [9] = {.lex_state = 3},
  [10] = {.lex_state = 3},
  [11] = {.lex_state = 3},
  [12] = {.lex_state = 3},
  [13] = {.lex_state = 3},
  [14] = {.lex_state = 3},
  [15] = {.lex_state = 3},
  [16] = {.lex_state = 3},
  [17] = {.lex_state = 3},
  [18] = {.lex_state = 3},
  [19] = {.lex_state = 3},
  [20] = {.lex_state = 3},
  [21] = {.lex_state = 14},
  [22] = {.lex_state = 14},
  [23] = {.lex_state = 14},
  [24] = {.lex_state = 14},
  [25] = {.lex_state = 14},
  [26] = {.lex_state = 14},
  [27] = {.lex_state = 14},
  [28] = {.lex_state = 14},
  [29] = {.lex_state = 14},
  [30] = {.lex_state = 14},
  [31] = {.lex_state = 14},
  [32] = {.lex_state = 14},
  [33] = {.lex_state = 14},
  [34] = {.lex_state = 14},
  [35] = {.lex_state = 14},
  [36] = {.lex_state = 14},
  [37] = {.lex_state = 14},
  [38] = {.lex_state = 14},
  [39] = {.lex_state = 14},
  [40] = {.lex_state = 14},
  [41] = {.lex_state = 14},
  [42] = {.lex_state = 14},
  [43] = {.lex_state = 14},
  [44] = {.lex_state = 14},
  [45] = {.lex_state = 14},
  [46] = {.lex_state = 14},
  [47] = {.lex_state = 14},
  [48] = {.lex_state = 14},
  [49] = {.lex_state = 3},
  [50] = {.lex_state = 14},
  [51] = {.lex_state = 14},
  [52] = {.lex_state = 14},
  [53] = {.lex_state = 14},
  [54] = {.lex_state = 14},
  [55] = {.lex_state = 14},
  [56] = {.lex_state = 4},
  [57] = {.lex_state = 14},
  [58] = {.lex_state = 14},
  [59] = {.lex_state = 14},
  [60] = {.lex_state = 2},
  [61] = {.lex_state = 2},
  [62] = {.lex_state = 2},
  [63] = {.lex_state = 14},
  [64] = {.lex_state = 14},
  [65] = {.lex_state = 14},
  [66] = {.lex_state = 14},
  [67] = {.lex_state = 14},
  [68] = {.lex_state = 14},
  [69] = {.lex_state = 14},
  [70] = {.lex_state = 14},
  [71] = {.lex_state = 14},
  [72] = {.lex_state = 14},
  [73] = {.lex_state = 14},
  [74] = {.lex_state = 14},
  [75] = {.lex_state = 14},
  [76] = {.lex_state = 14},
  [77] = {.lex_state = 14},
  [78] = {.lex_state = 14},
  [79] = {.lex_state = 14},
  [80] = {.lex_state = 14},
  [81] = {.lex_state = 14},
  [82] = {.lex_state = 14},
  [83] = {.lex_state = 2},
  [84] = {.lex_state = 14},
  [85] = {.lex_state = 14},
  [86] = {.lex_state = 14},
  [87] = {.lex_state = 14},
  [88] = {.lex_state = 14},
  [89] = {.lex_state = 14},
  [90] = {.lex_state = 14},
  [91] = {.lex_state = 14},
  [92] = {.lex_state = 14},
  [93] = {.lex_state = 14},
  [94] = {.lex_state = 4},
  [95] = {.lex_state = 14},
  [96] = {.lex_state = 14},
  [97] = {.lex_state = 14},
  [98] = {.lex_state = 14},
  [99] = {.lex_state = 14},
  [100] = {.lex_state = 14},
  [101] = {.lex_state = 14},
  [102] = {.lex_state = 14},
  [103] = {.lex_state = 14},
  [104] = {.lex_state = 14},
  [105] = {.lex_state = 14},
  [106] = {.lex_state = 1},
  [107] = {.lex_state = 1},
  [108] = {.lex_state = 1},
  [109] = {.lex_state = 14},
  [110] = {.lex_state = 14},
  [111] = {.lex_state = 14},
  [112] = {.lex_state = 14},
  [113] = {.lex_state = 1},
  [114] = {.lex_state = 14},
  [115] = {.lex_state = 14},
  [116] = {.lex_state = 14},
  [117] = {.lex_state = 14},
  [118] = {.lex_state = 14},
  [119] = {.lex_state = 14},
  [120] = {.lex_state = 5},
  [121] = {.lex_state = 14},
  [122] = {.lex_state = 14},
  [123] = {.lex_state = 14},
  [124] = {.lex_state = 14},
  [125] = {.lex_state = 14},
  [126] = {.lex_state = 14},
  [127] = {.lex_state = 14},
  [128] = {.lex_state = 14},
  [129] = {.lex_state = 14},
  [130] = {.lex_state = 14},
  [131] = {.lex_state = 3},
  [132] = {.lex_state = 3},
  [133] = {.lex_state = 3},
  [134] = {.lex_state = 3},
  [135] = {.lex_state = 3},
  [136] = {.lex_state = 3},
  [137] = {.lex_state = 14},
  [138] = {.lex_state = 14},
  [139] = {.lex_state = 74},
  [140] = {.lex_state = 14},
  [141] = {.lex_state = 14},
  [142] = {.lex_state = 14},
  [143] = {.lex_state = 3},
  [144] = {.lex_state = 3},
  [145] = {.lex_state = 14},
  [146] = {.lex_state = 14},
  [147] = {.lex_state = 3},
  [148] = {.lex_state = 14},
  [149] = {.lex_state = 3},
  [150] = {.lex_state = 14},
  [151] = {.lex_state = 3},
  [152] = {.lex_state = 14},
  [153] = {.lex_state = 3},
  [154] = {.lex_state = 14},
  [155] = {.lex_state = 6},
  [156] = {.lex_state = 3},
  [157] = {.lex_state = 14},
  [158] = {.lex_state = 3},
  [159] = {.lex_state = 70},
  [160] = {.lex_state = 6},
  [161] = {.lex_state = 14},
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
    [sym_NEWLINE] = ACTIONS(3),
    [sym_WHITESPACE] = ACTIONS(3),
  },
  [1] = {
    [sym_program] = STATE(150),
    [sym_statement] = STATE(93),
    [sym_grammar_statement] = STATE(81),
    [sym_fragment_statement] = STATE(81),
    [sym_ignore_statement] = STATE(81),
    [sym_assign_statement] = STATE(81),
    [aux_sym_program_repeat1] = STATE(52),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_id] = ACTIONS(7),
    [sym_grammar] = ACTIONS(9),
    [sym_fragment] = ACTIONS(11),
    [sym_ignore] = ACTIONS(13),
    [sym_NEWLINE] = ACTIONS(15),
    [sym_WHITESPACE] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_RPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(57), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [65] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    STATE(8), 1,
      sym__prefix_op,
    STATE(59), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [130] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(41), 1,
      anon_sym_RPAREN,
    STATE(8), 1,
      sym__prefix_op,
    STATE(57), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [195] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(43), 1,
      anon_sym_RPAREN,
    STATE(8), 1,
      sym__prefix_op,
    STATE(57), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [260] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(45), 1,
      anon_sym_RPAREN,
    STATE(8), 1,
      sym__prefix_op,
    STATE(57), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [325] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(47), 1,
      anon_sym_PIPE,
    ACTIONS(49), 1,
      anon_sym_CARET,
    ACTIONS(51), 1,
      anon_sym_SLASH,
    STATE(17), 1,
      sym__prefix_op,
    STATE(47), 1,
      sym_expression,
    STATE(140), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [390] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(24), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [452] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(51), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [514] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(57), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [576] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(49), 1,
      anon_sym_CARET,
    ACTIONS(51), 1,
      anon_sym_SLASH,
    STATE(17), 1,
      sym__prefix_op,
    STATE(25), 1,
      sym_expression,
    STATE(29), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [638] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(49), 1,
      anon_sym_CARET,
    ACTIONS(51), 1,
      anon_sym_SLASH,
    STATE(17), 1,
      sym__prefix_op,
    STATE(48), 1,
      sym_expression,
    STATE(140), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [700] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(49), 1,
      anon_sym_CARET,
    ACTIONS(51), 1,
      anon_sym_SLASH,
    STATE(17), 1,
      sym__prefix_op,
    STATE(26), 1,
      sym_expression,
    STATE(140), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [762] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(58), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [824] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(26), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [886] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(54), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [948] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(49), 1,
      anon_sym_CARET,
    ACTIONS(51), 1,
      anon_sym_SLASH,
    STATE(17), 1,
      sym__prefix_op,
    STATE(24), 1,
      sym_expression,
    STATE(140), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1010] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    ACTIONS(49), 1,
      anon_sym_CARET,
    ACTIONS(51), 1,
      anon_sym_SLASH,
    STATE(17), 1,
      sym__prefix_op,
    STATE(46), 1,
      sym_expression,
    STATE(140), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1072] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(55), 1,
      sym_expression,
    STATE(148), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1134] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LBRACK,
    ACTIONS(21), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_CARET,
    ACTIONS(27), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(33), 1,
      anon_sym_SLASH,
    ACTIONS(35), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(37), 1,
      anon_sym_BSLASHp,
    STATE(8), 1,
      sym__prefix_op,
    STATE(25), 1,
      sym_expression,
    STATE(29), 1,
      sym_choice_tag,
    ACTIONS(17), 2,
      sym_id,
      sym_unsigned,
    STATE(35), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(39), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1196] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(55), 1,
      sym_id,
    ACTIONS(53), 16,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1224] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(59), 1,
      sym_id,
    ACTIONS(61), 1,
      aux_sym_choice_tag_token1,
    ACTIONS(63), 1,
      anon_sym_COLON,
    ACTIONS(57), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1256] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(67), 1,
      sym_id,
    ACTIONS(69), 1,
      anon_sym_COLON,
    ACTIONS(65), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1285] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(73), 1,
      sym_id,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(71), 11,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1316] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(79), 1,
      sym_id,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(77), 11,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1347] = 8,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(83), 1,
      sym_id,
    ACTIONS(85), 1,
      anon_sym_POUND,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(81), 9,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      sym_eos,
  [1382] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(89), 1,
      sym_id,
    ACTIONS(87), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1408] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(93), 1,
      sym_id,
    ACTIONS(91), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1434] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(97), 1,
      sym_id,
    ACTIONS(95), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1460] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(101), 1,
      sym_id,
    ACTIONS(99), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1486] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(105), 1,
      sym_id,
    ACTIONS(103), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1512] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(109), 1,
      sym_id,
    ACTIONS(107), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1538] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(113), 1,
      sym_id,
    ACTIONS(111), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1564] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(117), 1,
      sym_id,
    ACTIONS(115), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1590] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(121), 1,
      sym_id,
    ACTIONS(119), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1616] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(125), 1,
      sym_id,
    ACTIONS(123), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1642] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(129), 1,
      sym_id,
    ACTIONS(127), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1668] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(133), 1,
      sym_id,
    ACTIONS(131), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1694] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(137), 1,
      sym_id,
    ACTIONS(135), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1720] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(141), 1,
      sym_id,
    ACTIONS(139), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1746] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(145), 1,
      sym_id,
    ACTIONS(143), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1772] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(149), 1,
      sym_id,
    ACTIONS(147), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1798] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(153), 1,
      sym_id,
    ACTIONS(151), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1824] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(157), 1,
      sym_id,
    ACTIONS(155), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1850] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(161), 1,
      sym_id,
    ACTIONS(163), 1,
      aux_sym_regex_long_token1,
    ACTIONS(159), 12,
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
  [1877] = 11,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(167), 1,
      sym_id,
    ACTIONS(169), 1,
      anon_sym_TILDE,
    ACTIONS(171), 1,
      anon_sym_LT_DASH,
    ACTIONS(173), 1,
      sym_eos,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(165), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [1916] = 11,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(169), 1,
      anon_sym_TILDE,
    ACTIONS(171), 1,
      anon_sym_LT_DASH,
    ACTIONS(177), 1,
      sym_id,
    ACTIONS(179), 1,
      sym_eos,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(175), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [1955] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(169), 1,
      anon_sym_TILDE,
    ACTIONS(183), 1,
      sym_id,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(181), 6,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
      anon_sym_LT_DASH,
      sym_eos,
  [1990] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(187), 1,
      anon_sym_LBRACK,
    ACTIONS(185), 11,
      anon_sym_PIPE,
      anon_sym_LPAREN,
      anon_sym_CARET,
      anon_sym_AT,
      sym_id,
      sym_unsigned,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_SLASH,
      anon_sym_LBRACK_CARET,
      anon_sym_BSLASHp,
  [2013] = 10,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(191), 1,
      sym_id,
    ACTIONS(193), 1,
      anon_sym_LBRACE,
    ACTIONS(195), 1,
      anon_sym_LBRACK,
    ACTIONS(197), 1,
      sym_eos,
    STATE(64), 1,
      sym_string,
    ACTIONS(189), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2047] = 11,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(199), 1,
      anon_sym_COMMA,
    ACTIONS(201), 1,
      anon_sym_RPAREN,
    ACTIONS(203), 1,
      anon_sym_TILDE,
    ACTIONS(205), 1,
      anon_sym_LT_DASH,
    STATE(41), 1,
      sym__suffix_op,
    STATE(130), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2083] = 10,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(7), 1,
      sym_id,
    ACTIONS(9), 1,
      sym_grammar,
    ACTIONS(11), 1,
      sym_fragment,
    ACTIONS(13), 1,
      sym_ignore,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(207), 1,
      ts_builtin_sym_end,
    STATE(53), 1,
      aux_sym_program_repeat1,
    STATE(93), 1,
      sym_statement,
    STATE(81), 4,
      sym_grammar_statement,
      sym_fragment_statement,
      sym_ignore_statement,
      sym_assign_statement,
  [2117] = 10,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(209), 1,
      ts_builtin_sym_end,
    ACTIONS(211), 1,
      sym_id,
    ACTIONS(214), 1,
      sym_grammar,
    ACTIONS(217), 1,
      sym_fragment,
    ACTIONS(220), 1,
      sym_ignore,
    STATE(53), 1,
      aux_sym_program_repeat1,
    STATE(93), 1,
      sym_statement,
    STATE(81), 4,
      sym_grammar_statement,
      sym_fragment_statement,
      sym_ignore_statement,
      sym_assign_statement,
  [2151] = 11,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(203), 1,
      anon_sym_TILDE,
    ACTIONS(205), 1,
      anon_sym_LT_DASH,
    ACTIONS(223), 1,
      anon_sym_COMMA,
    ACTIONS(225), 1,
      anon_sym_RPAREN,
    STATE(41), 1,
      sym__suffix_op,
    STATE(127), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2187] = 8,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(203), 1,
      anon_sym_TILDE,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(181), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LT_DASH,
  [2216] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(227), 1,
      aux_sym_regex_long_token1,
    ACTIONS(159), 9,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
  [2237] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(203), 1,
      anon_sym_TILDE,
    ACTIONS(205), 1,
      anon_sym_LT_DASH,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(229), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2268] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(203), 1,
      anon_sym_TILDE,
    ACTIONS(205), 1,
      anon_sym_LT_DASH,
    ACTIONS(231), 1,
      anon_sym_RPAREN,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2298] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(77), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_POUND,
    ACTIONS(203), 1,
      anon_sym_TILDE,
    ACTIONS(205), 1,
      anon_sym_LT_DASH,
    ACTIONS(233), 1,
      anon_sym_RPAREN,
    STATE(41), 1,
      sym__suffix_op,
    ACTIONS(75), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2328] = 7,
    ACTIONS(235), 1,
      anon_sym_RBRACK,
    ACTIONS(237), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(240), 1,
      anon_sym_BSLASHp,
    STATE(60), 1,
      aux_sym_regex_range_repeat1,
    STATE(107), 1,
      sym_regex_range_item,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(106), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [2352] = 7,
    ACTIONS(243), 1,
      anon_sym_RBRACK,
    ACTIONS(245), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(247), 1,
      anon_sym_BSLASHp,
    STATE(60), 1,
      aux_sym_regex_range_repeat1,
    STATE(107), 1,
      sym_regex_range_item,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(106), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [2376] = 7,
    ACTIONS(245), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(247), 1,
      anon_sym_BSLASHp,
    ACTIONS(249), 1,
      anon_sym_RBRACK,
    STATE(61), 1,
      aux_sym_regex_range_repeat1,
    STATE(107), 1,
      sym_regex_range_item,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(106), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [2400] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(253), 1,
      sym_id,
    ACTIONS(255), 1,
      sym_eos,
    ACTIONS(251), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2419] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(259), 1,
      sym_id,
    ACTIONS(261), 1,
      sym_eos,
    ACTIONS(257), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2438] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(265), 1,
      sym_id,
    ACTIONS(267), 1,
      sym_eos,
    ACTIONS(263), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2457] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(265), 1,
      sym_id,
    ACTIONS(269), 1,
      sym_eos,
    ACTIONS(263), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2476] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(273), 1,
      sym_id,
    ACTIONS(275), 1,
      sym_eos,
    ACTIONS(271), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2495] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(273), 1,
      sym_id,
    ACTIONS(277), 1,
      sym_eos,
    ACTIONS(271), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2514] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(281), 1,
      sym_id,
    ACTIONS(283), 1,
      sym_eos,
    ACTIONS(279), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2533] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(287), 1,
      sym_id,
    ACTIONS(289), 1,
      sym_eos,
    ACTIONS(285), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2552] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(293), 1,
      sym_id,
    ACTIONS(295), 1,
      sym_eos,
    ACTIONS(291), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2571] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(299), 1,
      sym_id,
    ACTIONS(301), 1,
      sym_eos,
    ACTIONS(297), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2590] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(305), 1,
      sym_id,
    ACTIONS(307), 1,
      sym_eos,
    ACTIONS(303), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2609] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(311), 1,
      sym_id,
    ACTIONS(313), 1,
      sym_eos,
    ACTIONS(309), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2628] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(317), 1,
      sym_id,
    ACTIONS(319), 1,
      sym_eos,
    ACTIONS(315), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2647] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(323), 1,
      sym_id,
    ACTIONS(325), 1,
      sym_eos,
    ACTIONS(321), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2666] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(329), 1,
      sym_id,
    ACTIONS(331), 1,
      sym_eos,
    ACTIONS(327), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2685] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(293), 1,
      sym_id,
    ACTIONS(333), 1,
      sym_eos,
    ACTIONS(291), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2704] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(253), 1,
      sym_id,
    ACTIONS(335), 1,
      sym_eos,
    ACTIONS(251), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2723] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(339), 1,
      sym_id,
    ACTIONS(337), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2739] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(343), 1,
      sym_id,
    ACTIONS(341), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2755] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(347), 1,
      sym_id,
    ACTIONS(345), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2771] = 5,
    ACTIONS(245), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(247), 1,
      anon_sym_BSLASHp,
    STATE(113), 1,
      sym_regex_range_item,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(106), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [2789] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(351), 1,
      sym_id,
    ACTIONS(349), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2805] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(355), 1,
      sym_id,
    ACTIONS(353), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2821] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(359), 1,
      sym_id,
    ACTIONS(357), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2837] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(351), 1,
      sym_id,
    ACTIONS(349), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2853] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(293), 1,
      sym_id,
    ACTIONS(291), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2869] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(265), 1,
      sym_id,
    ACTIONS(263), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2885] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(363), 1,
      sym_id,
    ACTIONS(361), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2901] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(323), 1,
      sym_id,
    ACTIONS(321), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2917] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(367), 1,
      sym_id,
    ACTIONS(365), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2933] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(371), 1,
      sym_id,
    ACTIONS(369), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2949] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    STATE(7), 1,
      sym_eq,
    ACTIONS(373), 4,
      anon_sym_EQ,
      anon_sym__EQ,
      anon_sym_AT_EQ,
      anon_sym_CARET_EQ,
  [2965] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(377), 1,
      sym_id,
    ACTIONS(375), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2981] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(381), 1,
      sym_id,
    ACTIONS(379), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [2997] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(287), 1,
      sym_id,
    ACTIONS(285), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [3013] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(385), 1,
      sym_id,
    ACTIONS(383), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [3029] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(367), 1,
      sym_id,
    ACTIONS(365), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [3045] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(389), 1,
      sym_id,
    ACTIONS(387), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [3061] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(393), 1,
      sym_id,
    ACTIONS(391), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [3077] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(265), 1,
      sym_id,
    ACTIONS(263), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [3093] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(293), 1,
      sym_id,
    ACTIONS(291), 4,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      sym_ignore,
  [3109] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(395), 1,
      anon_sym_COMMA,
    STATE(104), 1,
      aux_sym_ignore_statement_repeat1,
    ACTIONS(398), 2,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [3126] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(400), 1,
      anon_sym_RBRACK,
    STATE(128), 1,
      sym_string,
  [3145] = 3,
    ACTIONS(404), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(402), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [3158] = 4,
    ACTIONS(408), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(410), 1,
      anon_sym_DASH,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(406), 2,
      anon_sym_RBRACK,
      anon_sym_BSLASHp,
  [3173] = 3,
    ACTIONS(141), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(139), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [3186] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(412), 1,
      anon_sym_RBRACE,
    STATE(128), 1,
      sym_string,
  [3205] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(414), 1,
      anon_sym_RBRACE,
    STATE(124), 1,
      sym_string,
  [3224] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(414), 1,
      anon_sym_RBRACK,
    STATE(125), 1,
      sym_string,
  [3243] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(416), 1,
      anon_sym_COMMA,
    STATE(112), 1,
      aux_sym_grammar_statement_repeat1,
    ACTIONS(419), 2,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [3260] = 3,
    ACTIONS(423), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(421), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [3273] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(425), 1,
      anon_sym_RBRACK,
    STATE(128), 1,
      sym_string,
  [3292] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    ACTIONS(400), 1,
      anon_sym_RBRACE,
    STATE(128), 1,
      sym_string,
  [3311] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(427), 3,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [3323] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(429), 1,
      anon_sym_COMMA,
    ACTIONS(431), 1,
      anon_sym_RBRACE,
    STATE(104), 1,
      aux_sym_ignore_statement_repeat1,
  [3339] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(433), 1,
      anon_sym_COMMA,
    ACTIONS(435), 1,
      anon_sym_RBRACK,
    STATE(112), 1,
      aux_sym_grammar_statement_repeat1,
  [3355] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(229), 1,
      anon_sym_RPAREN,
    ACTIONS(437), 1,
      anon_sym_COMMA,
    STATE(119), 1,
      aux_sym_macro_call_repeat1,
  [3371] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(440), 1,
      sym_id,
    ACTIONS(442), 1,
      anon_sym_LBRACE,
    ACTIONS(444), 1,
      anon_sym_LBRACK,
  [3387] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(446), 1,
      anon_sym_COMMA,
    ACTIONS(448), 1,
      anon_sym_RBRACE,
    STATE(112), 1,
      aux_sym_grammar_statement_repeat1,
  [3403] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(450), 1,
      anon_sym_COMMA,
    ACTIONS(452), 1,
      anon_sym_RBRACK,
    STATE(129), 1,
      aux_sym_ignore_statement_repeat1,
  [3419] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(29), 1,
      anon_sym_SQUOTE,
    ACTIONS(31), 1,
      anon_sym_DQUOTE,
    STATE(128), 1,
      sym_string,
  [3435] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(454), 1,
      anon_sym_COMMA,
    ACTIONS(456), 1,
      anon_sym_RBRACE,
    STATE(121), 1,
      aux_sym_grammar_statement_repeat1,
  [3451] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(456), 1,
      anon_sym_RBRACK,
    ACTIONS(458), 1,
      anon_sym_COMMA,
    STATE(118), 1,
      aux_sym_grammar_statement_repeat1,
  [3467] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(452), 1,
      anon_sym_RBRACE,
    ACTIONS(460), 1,
      anon_sym_COMMA,
    STATE(117), 1,
      aux_sym_ignore_statement_repeat1,
  [3483] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(23), 1,
      anon_sym_RPAREN,
    ACTIONS(462), 1,
      anon_sym_COMMA,
    STATE(119), 1,
      aux_sym_macro_call_repeat1,
  [3499] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(464), 3,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [3511] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(466), 1,
      anon_sym_COMMA,
    ACTIONS(468), 1,
      anon_sym_RBRACK,
    STATE(104), 1,
      aux_sym_ignore_statement_repeat1,
  [3527] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(43), 1,
      anon_sym_RPAREN,
    ACTIONS(470), 1,
      anon_sym_COMMA,
    STATE(119), 1,
      aux_sym_macro_call_repeat1,
  [3543] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(472), 1,
      sym_id,
    ACTIONS(474), 1,
      anon_sym_RBRACE,
  [3556] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(476), 1,
      sym_id,
    ACTIONS(478), 1,
      anon_sym_RBRACK,
  [3569] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(472), 1,
      sym_id,
    ACTIONS(474), 1,
      anon_sym_RBRACK,
  [3582] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(478), 1,
      anon_sym_RBRACE,
    ACTIONS(480), 1,
      sym_id,
  [3595] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(472), 1,
      sym_id,
    ACTIONS(482), 1,
      anon_sym_RBRACK,
  [3608] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(472), 1,
      sym_id,
    ACTIONS(484), 1,
      anon_sym_RBRACE,
  [3621] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(486), 1,
      anon_sym_LPAREN,
    ACTIONS(488), 1,
      anon_sym_DOT,
  [3634] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(490), 1,
      anon_sym_DQUOTE,
  [3644] = 2,
    ACTIONS(492), 1,
      aux_sym_string_token2,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
  [3652] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(494), 1,
      anon_sym_PIPE,
  [3662] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(496), 1,
      anon_sym_LBRACE,
  [3672] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(498), 1,
      anon_sym_SLASH,
  [3682] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(500), 1,
      sym_id,
  [3692] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(502), 1,
      sym_id,
  [3702] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(504), 1,
      anon_sym_LPAREN,
  [3712] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(490), 1,
      anon_sym_SQUOTE,
  [3722] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(506), 1,
      sym_id,
  [3732] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(508), 1,
      anon_sym_PIPE,
  [3742] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(510), 1,
      sym_id,
  [3752] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(512), 1,
      ts_builtin_sym_end,
  [3762] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(472), 1,
      sym_id,
  [3772] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(514), 1,
      anon_sym_SLASH,
  [3782] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(516), 1,
      sym_id,
  [3792] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(518), 1,
      anon_sym_RBRACE,
  [3802] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(520), 1,
      aux_sym_regex_set_token1,
  [3812] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(522), 1,
      sym_id,
  [3822] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(524), 1,
      anon_sym_RBRACE,
  [3832] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(526), 1,
      sym_id,
  [3842] = 2,
    ACTIONS(528), 1,
      aux_sym_string_token1,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
  [3850] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(530), 1,
      aux_sym_regex_set_token1,
  [3860] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(15), 1,
      sym_NEWLINE,
    ACTIONS(532), 1,
      anon_sym_LBRACE,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 65,
  [SMALL_STATE(4)] = 130,
  [SMALL_STATE(5)] = 195,
  [SMALL_STATE(6)] = 260,
  [SMALL_STATE(7)] = 325,
  [SMALL_STATE(8)] = 390,
  [SMALL_STATE(9)] = 452,
  [SMALL_STATE(10)] = 514,
  [SMALL_STATE(11)] = 576,
  [SMALL_STATE(12)] = 638,
  [SMALL_STATE(13)] = 700,
  [SMALL_STATE(14)] = 762,
  [SMALL_STATE(15)] = 824,
  [SMALL_STATE(16)] = 886,
  [SMALL_STATE(17)] = 948,
  [SMALL_STATE(18)] = 1010,
  [SMALL_STATE(19)] = 1072,
  [SMALL_STATE(20)] = 1134,
  [SMALL_STATE(21)] = 1196,
  [SMALL_STATE(22)] = 1224,
  [SMALL_STATE(23)] = 1256,
  [SMALL_STATE(24)] = 1285,
  [SMALL_STATE(25)] = 1316,
  [SMALL_STATE(26)] = 1347,
  [SMALL_STATE(27)] = 1382,
  [SMALL_STATE(28)] = 1408,
  [SMALL_STATE(29)] = 1434,
  [SMALL_STATE(30)] = 1460,
  [SMALL_STATE(31)] = 1486,
  [SMALL_STATE(32)] = 1512,
  [SMALL_STATE(33)] = 1538,
  [SMALL_STATE(34)] = 1564,
  [SMALL_STATE(35)] = 1590,
  [SMALL_STATE(36)] = 1616,
  [SMALL_STATE(37)] = 1642,
  [SMALL_STATE(38)] = 1668,
  [SMALL_STATE(39)] = 1694,
  [SMALL_STATE(40)] = 1720,
  [SMALL_STATE(41)] = 1746,
  [SMALL_STATE(42)] = 1772,
  [SMALL_STATE(43)] = 1798,
  [SMALL_STATE(44)] = 1824,
  [SMALL_STATE(45)] = 1850,
  [SMALL_STATE(46)] = 1877,
  [SMALL_STATE(47)] = 1916,
  [SMALL_STATE(48)] = 1955,
  [SMALL_STATE(49)] = 1990,
  [SMALL_STATE(50)] = 2013,
  [SMALL_STATE(51)] = 2047,
  [SMALL_STATE(52)] = 2083,
  [SMALL_STATE(53)] = 2117,
  [SMALL_STATE(54)] = 2151,
  [SMALL_STATE(55)] = 2187,
  [SMALL_STATE(56)] = 2216,
  [SMALL_STATE(57)] = 2237,
  [SMALL_STATE(58)] = 2268,
  [SMALL_STATE(59)] = 2298,
  [SMALL_STATE(60)] = 2328,
  [SMALL_STATE(61)] = 2352,
  [SMALL_STATE(62)] = 2376,
  [SMALL_STATE(63)] = 2400,
  [SMALL_STATE(64)] = 2419,
  [SMALL_STATE(65)] = 2438,
  [SMALL_STATE(66)] = 2457,
  [SMALL_STATE(67)] = 2476,
  [SMALL_STATE(68)] = 2495,
  [SMALL_STATE(69)] = 2514,
  [SMALL_STATE(70)] = 2533,
  [SMALL_STATE(71)] = 2552,
  [SMALL_STATE(72)] = 2571,
  [SMALL_STATE(73)] = 2590,
  [SMALL_STATE(74)] = 2609,
  [SMALL_STATE(75)] = 2628,
  [SMALL_STATE(76)] = 2647,
  [SMALL_STATE(77)] = 2666,
  [SMALL_STATE(78)] = 2685,
  [SMALL_STATE(79)] = 2704,
  [SMALL_STATE(80)] = 2723,
  [SMALL_STATE(81)] = 2739,
  [SMALL_STATE(82)] = 2755,
  [SMALL_STATE(83)] = 2771,
  [SMALL_STATE(84)] = 2789,
  [SMALL_STATE(85)] = 2805,
  [SMALL_STATE(86)] = 2821,
  [SMALL_STATE(87)] = 2837,
  [SMALL_STATE(88)] = 2853,
  [SMALL_STATE(89)] = 2869,
  [SMALL_STATE(90)] = 2885,
  [SMALL_STATE(91)] = 2901,
  [SMALL_STATE(92)] = 2917,
  [SMALL_STATE(93)] = 2933,
  [SMALL_STATE(94)] = 2949,
  [SMALL_STATE(95)] = 2965,
  [SMALL_STATE(96)] = 2981,
  [SMALL_STATE(97)] = 2997,
  [SMALL_STATE(98)] = 3013,
  [SMALL_STATE(99)] = 3029,
  [SMALL_STATE(100)] = 3045,
  [SMALL_STATE(101)] = 3061,
  [SMALL_STATE(102)] = 3077,
  [SMALL_STATE(103)] = 3093,
  [SMALL_STATE(104)] = 3109,
  [SMALL_STATE(105)] = 3126,
  [SMALL_STATE(106)] = 3145,
  [SMALL_STATE(107)] = 3158,
  [SMALL_STATE(108)] = 3173,
  [SMALL_STATE(109)] = 3186,
  [SMALL_STATE(110)] = 3205,
  [SMALL_STATE(111)] = 3224,
  [SMALL_STATE(112)] = 3243,
  [SMALL_STATE(113)] = 3260,
  [SMALL_STATE(114)] = 3273,
  [SMALL_STATE(115)] = 3292,
  [SMALL_STATE(116)] = 3311,
  [SMALL_STATE(117)] = 3323,
  [SMALL_STATE(118)] = 3339,
  [SMALL_STATE(119)] = 3355,
  [SMALL_STATE(120)] = 3371,
  [SMALL_STATE(121)] = 3387,
  [SMALL_STATE(122)] = 3403,
  [SMALL_STATE(123)] = 3419,
  [SMALL_STATE(124)] = 3435,
  [SMALL_STATE(125)] = 3451,
  [SMALL_STATE(126)] = 3467,
  [SMALL_STATE(127)] = 3483,
  [SMALL_STATE(128)] = 3499,
  [SMALL_STATE(129)] = 3511,
  [SMALL_STATE(130)] = 3527,
  [SMALL_STATE(131)] = 3543,
  [SMALL_STATE(132)] = 3556,
  [SMALL_STATE(133)] = 3569,
  [SMALL_STATE(134)] = 3582,
  [SMALL_STATE(135)] = 3595,
  [SMALL_STATE(136)] = 3608,
  [SMALL_STATE(137)] = 3621,
  [SMALL_STATE(138)] = 3634,
  [SMALL_STATE(139)] = 3644,
  [SMALL_STATE(140)] = 3652,
  [SMALL_STATE(141)] = 3662,
  [SMALL_STATE(142)] = 3672,
  [SMALL_STATE(143)] = 3682,
  [SMALL_STATE(144)] = 3692,
  [SMALL_STATE(145)] = 3702,
  [SMALL_STATE(146)] = 3712,
  [SMALL_STATE(147)] = 3722,
  [SMALL_STATE(148)] = 3732,
  [SMALL_STATE(149)] = 3742,
  [SMALL_STATE(150)] = 3752,
  [SMALL_STATE(151)] = 3762,
  [SMALL_STATE(152)] = 3772,
  [SMALL_STATE(153)] = 3782,
  [SMALL_STATE(154)] = 3792,
  [SMALL_STATE(155)] = 3802,
  [SMALL_STATE(156)] = 3812,
  [SMALL_STATE(157)] = 3822,
  [SMALL_STATE(158)] = 3832,
  [SMALL_STATE(159)] = 3842,
  [SMALL_STATE(160)] = 3850,
  [SMALL_STATE(161)] = 3860,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 0),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(94),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(62),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3),
  [55] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 3, .production_id = 18),
  [59] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 3, .production_id = 18),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 4, .production_id = 23),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 4, .production_id = 23),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_prefix, 2, .production_id = 13),
  [73] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_prefix, 2, .production_id = 13),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 1, .production_id = 7),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 1, .production_id = 7),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_concat_expression, 3, .production_id = 17),
  [83] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_concat_expression, 3, .production_id = 17),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range, 2, .production_id = 10),
  [89] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range, 2, .production_id = 10),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range, 3, .production_id = 10),
  [93] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range, 3, .production_id = 10),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_expression, 3, .production_id = 17),
  [97] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_expression, 3, .production_id = 17),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 7, .production_id = 27),
  [101] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 7, .production_id = 27),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_long, 3),
  [105] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_long, 3),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 9, .production_id = 27),
  [109] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 9, .production_id = 27),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 3),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 3),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 7, .production_id = 24),
  [117] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 7, .production_id = 24),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data, 1),
  [121] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_data, 1),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 6, .production_id = 26),
  [125] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 6, .production_id = 26),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 8, .production_id = 27),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 8, .production_id = 27),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 5, .production_id = 24),
  [133] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 5, .production_id = 24),
  [135] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1),
  [137] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 1),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_set, 4, .production_id = 22),
  [141] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_set, 4, .production_id = 22),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_suffix, 2, .production_id = 12),
  [145] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_suffix, 2, .production_id = 12),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 6, .production_id = 24),
  [149] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 6, .production_id = 24),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 5, .production_id = 25),
  [153] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 5, .production_id = 25),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 4),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 4),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_long, 2),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_long, 2),
  [163] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 4, .production_id = 11),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 4, .production_id = 11),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 3, .production_id = 8),
  [177] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 3, .production_id = 8),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_expr, 3, .production_id = 17),
  [183] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_expr, 3, .production_id = 17),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_eq, 1),
  [187] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_eq, 1),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 2, .production_id = 3),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 2, .production_id = 3),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [197] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [199] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [201] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 1, .production_id = 2),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5),
  [211] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(94),
  [214] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(147),
  [217] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(158),
  [220] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 5), SHIFT_REPEAT(120),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [225] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_macro_call_repeat1, 2),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [233] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 2),
  [237] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_regex_range_repeat1, 2), SHIFT_REPEAT(106),
  [240] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 2), SHIFT_REPEAT(161),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [245] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 5, .production_id = 15),
  [253] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 5, .production_id = 15),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [257] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 3, .production_id = 6),
  [259] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 3, .production_id = 6),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 6, .production_id = 15),
  [265] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 6, .production_id = 15),
  [267] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [269] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [271] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 6, .production_id = 20),
  [273] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 6, .production_id = 20),
  [275] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [277] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [279] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 2, .production_id = 4),
  [281] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 2, .production_id = 4),
  [283] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 6, .production_id = 14),
  [287] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 6, .production_id = 14),
  [289] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 7, .production_id = 20),
  [293] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 7, .production_id = 20),
  [295] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [297] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 5, .production_id = 14),
  [299] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 5, .production_id = 14),
  [301] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 3),
  [305] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 3),
  [307] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 4, .production_id = 3),
  [311] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 4, .production_id = 3),
  [313] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [315] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment_statement, 2, .production_id = 3),
  [317] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fragment_statement, 2, .production_id = 3),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [321] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 5, .production_id = 9),
  [323] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 5, .production_id = 9),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 4, .production_id = 9),
  [329] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 4, .production_id = 9),
  [331] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [337] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 4, .production_id = 6),
  [339] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 4, .production_id = 6),
  [341] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 1),
  [343] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 1),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 4),
  [347] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 4),
  [349] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 8, .production_id = 20),
  [351] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 8, .production_id = 20),
  [353] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 3, .production_id = 3),
  [355] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 3, .production_id = 3),
  [357] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 5, .production_id = 11),
  [359] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 5, .production_id = 11),
  [361] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 3, .production_id = 4),
  [363] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 3, .production_id = 4),
  [365] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 7, .production_id = 15),
  [367] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 7, .production_id = 15),
  [369] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 1, .production_id = 1),
  [371] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_program_repeat1, 1, .production_id = 1),
  [373] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [375] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment_statement, 3, .production_id = 3),
  [377] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fragment_statement, 3, .production_id = 3),
  [379] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 7, .production_id = 14),
  [381] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 7, .production_id = 14),
  [383] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 5, .production_id = 3),
  [385] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 5, .production_id = 3),
  [387] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ignore_statement, 6, .production_id = 9),
  [389] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ignore_statement, 6, .production_id = 9),
  [391] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 4, .production_id = 8),
  [393] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 4, .production_id = 8),
  [395] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ignore_statement_repeat1, 2, .production_id = 16), SHIFT_REPEAT(151),
  [398] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ignore_statement_repeat1, 2, .production_id = 16),
  [400] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range_item, 1),
  [404] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range_item, 1),
  [406] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 1),
  [408] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_regex_range_repeat1, 1),
  [410] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [412] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [414] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [416] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_grammar_statement_repeat1, 2, .production_id = 21), SHIFT_REPEAT(123),
  [419] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_grammar_statement_repeat1, 2, .production_id = 21),
  [421] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range_item_group, 3, .production_id = 17),
  [423] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range_item_group, 3, .production_id = 17),
  [425] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [427] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ignore_statement_repeat1, 2, .production_id = 4),
  [429] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [431] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [433] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [435] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [437] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_macro_call_repeat1, 2), SHIFT_REPEAT(10),
  [440] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [442] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [444] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [446] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [448] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [450] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [452] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [454] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [456] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [458] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [460] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [462] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [464] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_grammar_statement_repeat1, 2, .production_id = 19),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [468] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [470] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [472] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [474] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [476] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [478] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [480] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [482] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [484] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [486] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [488] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [490] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [492] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [494] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [496] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [498] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [500] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [502] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [504] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [508] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [510] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [512] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [516] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [518] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [520] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [522] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [524] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [526] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [528] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [530] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [532] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
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
