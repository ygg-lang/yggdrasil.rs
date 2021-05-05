#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 115
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 69
#define ALIAS_COUNT 0
#define TOKEN_COUNT 42
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 18
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 23

enum {
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
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_id] = "id",
  [anon_sym_LBRACE] = "{",
  [anon_sym_COMMA] = ",",
  [anon_sym_RBRACE] = "}",
  [sym_grammar] = "grammar",
  [sym_fragment] = "fragment",
  [anon_sym_PIPE] = "|",
  [anon_sym_EQ] = "=",
  [anon_sym__EQ] = "_=",
  [anon_sym_AT_EQ] = "@=",
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
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
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
  [aux_sym__grammar_exts_repeat1] = "_grammar_exts_repeat1",
  [aux_sym_macro_call_repeat1] = "macro_call_repeat1",
  [aux_sym_regex_range_repeat1] = "regex_range_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_id] = sym_id,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [sym_grammar] = sym_grammar,
  [sym_fragment] = sym_fragment,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym__EQ] = anon_sym__EQ,
  [anon_sym_AT_EQ] = anon_sym_AT_EQ,
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
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
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
  [aux_sym__grammar_exts_repeat1] = aux_sym__grammar_exts_repeat1,
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
  [sym_grammar] = {
    .visible = true,
    .named = true,
  },
  [sym_fragment] = {
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
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
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
  [aux_sym__grammar_exts_repeat1] = {
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
  field_lhs = 8,
  field_mode = 9,
  field_name = 10,
  field_op = 11,
  field_prefix = 12,
  field_rhs = 13,
  field_set = 14,
  field_statement = 15,
  field_suffix = 16,
  field_tag = 17,
  field_ty = 18,
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
  [4] = {.index = 3, .length = 2},
  [5] = {.index = 5, .length = 1},
  [6] = {.index = 6, .length = 3},
  [7] = {.index = 9, .length = 3},
  [8] = {.index = 12, .length = 1},
  [9] = {.index = 13, .length = 2},
  [10] = {.index = 15, .length = 2},
  [11] = {.index = 17, .length = 3},
  [12] = {.index = 20, .length = 2},
  [13] = {.index = 22, .length = 2},
  [14] = {.index = 24, .length = 1},
  [15] = {.index = 25, .length = 2},
  [16] = {.index = 27, .length = 1},
  [17] = {.index = 28, .length = 3},
  [18] = {.index = 31, .length = 3},
  [19] = {.index = 34, .length = 3},
  [20] = {.index = 37, .length = 1},
  [21] = {.index = 38, .length = 4},
  [22] = {.index = 42, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_statement, 0},
  [1] =
    {field_statement, 0, .inherited = true},
  [2] =
    {field_id, 1},
  [3] =
    {field_statement, 0, .inherited = true},
    {field_statement, 1, .inherited = true},
  [5] =
    {field_expression, 0},
  [6] =
    {field_eq, 1},
    {field_id, 0},
    {field_rhs, 2},
  [9] =
    {field_eq, 1},
    {field_id, 0},
    {field_rhs, 3},
  [12] =
    {field_is_neg, 0},
  [13] =
    {field_base, 0},
    {field_suffix, 1},
  [15] =
    {field_base, 1},
    {field_prefix, 0},
  [17] =
    {field_lhs, 0},
    {field_op, 1},
    {field_rhs, 2},
  [20] =
    {field_expression, 0},
    {field_tag, 2},
  [22] =
    {field_ext, 3},
    {field_id, 1},
  [24] =
    {field_ext, 1},
  [25] =
    {field_ext, 0, .inherited = true},
    {field_ext, 1, .inherited = true},
  [27] =
    {field_set, 2},
  [28] =
    {field_expression, 0},
    {field_mode, 3},
    {field_tag, 2},
  [31] =
    {field_ext, 3},
    {field_ext, 4, .inherited = true},
    {field_id, 1},
  [34] =
    {field_expression, 0},
    {field_tag, 2},
    {field_ty, 4},
  [37] =
    {field_name, 1},
  [38] =
    {field_expression, 0},
    {field_mode, 3},
    {field_tag, 2},
    {field_ty, 5},
  [42] =
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
      if (eof) ADVANCE(13);
      if (lookahead == '\n') ADVANCE(69);
      if (lookahead == '\r') ADVANCE(68);
      if (lookahead == '!') ADVANCE(32);
      if (lookahead == '"') ADVANCE(59);
      if (lookahead == '#') ADVANCE(31);
      if (lookahead == '\'') ADVANCE(55);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == ')') ADVANCE(24);
      if (lookahead == '*') ADVANCE(27);
      if (lookahead == '+') ADVANCE(28);
      if (lookahead == ',') ADVANCE(15);
      if (lookahead == '-') ADVANCE(72);
      if (lookahead == '.') ADVANCE(36);
      if (lookahead == '/') ADVANCE(63);
      if (lookahead == '0') ADVANCE(53);
      if (lookahead == ':') ADVANCE(33);
      if (lookahead == ';') ADVANCE(75);
      if (lookahead == '<') ADVANCE(70);
      if (lookahead == '=') ADVANCE(20);
      if (lookahead == '?') ADVANCE(26);
      if (lookahead == '@') ADVANCE(35);
      if (lookahead == '[') ADVANCE(66);
      if (lookahead == '\\') ADVANCE(71);
      if (lookahead == ']') ADVANCE(67);
      if (lookahead == '^') ADVANCE(25);
      if (lookahead == 'f') ADVANCE(48);
      if (lookahead == 'g') ADVANCE(50);
      if (lookahead == 'i') ADVANCE(52);
      if (lookahead == '{') ADVANCE(14);
      if (lookahead == '|') ADVANCE(19);
      if (lookahead == '}') ADVANCE(16);
      if (lookahead == '~') ADVANCE(29);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(68);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(54);
      if (sym_id_character_set_1(lookahead)) ADVANCE(52);
      if (lookahead != 0) ADVANCE(68);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(69);
      if (lookahead == '\r') ADVANCE(68);
      if (lookahead == '-') ADVANCE(72);
      if (lookahead == '\\') ADVANCE(71);
      if (lookahead == ']') ADVANCE(67);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(68);
      if (lookahead != 0) ADVANCE(68);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(69);
      if (lookahead == '\r') ADVANCE(68);
      if (lookahead == '\\') ADVANCE(71);
      if (lookahead == ']') ADVANCE(67);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(68);
      if (lookahead != 0) ADVANCE(68);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(78);
      if (lookahead == '\r') ADVANCE(76);
      if (lookahead == '"') ADVANCE(59);
      if (lookahead == '\'') ADVANCE(55);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == ')') ADVANCE(24);
      if (lookahead == '/') ADVANCE(63);
      if (lookahead == '0') ADVANCE(53);
      if (lookahead == '@') ADVANCE(34);
      if (lookahead == '[') ADVANCE(66);
      if (lookahead == '\\') ADVANCE(9);
      if (lookahead == '^') ADVANCE(25);
      if (lookahead == '|') ADVANCE(19);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(77);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(54);
      if (sym_id_character_set_2(lookahead)) ADVANCE(52);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(78);
      if (lookahead == '\r') ADVANCE(76);
      if (lookahead == '#') ADVANCE(31);
      if (lookahead == ')') ADVANCE(24);
      if (lookahead == '*') ADVANCE(27);
      if (lookahead == '+') ADVANCE(28);
      if (lookahead == ',') ADVANCE(15);
      if (lookahead == '<') ADVANCE(6);
      if (lookahead == '=') ADVANCE(20);
      if (lookahead == '?') ADVANCE(26);
      if (lookahead == '@') ADVANCE(7);
      if (lookahead == '_') ADVANCE(8);
      if (lookahead == 'g' ||
          lookahead == 'i') ADVANCE(64);
      if (lookahead == '|') ADVANCE(19);
      if (lookahead == '~') ADVANCE(29);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(77);
      END_STATE();
    case 5:
      if (lookahead == '\n') ADVANCE(78);
      if (lookahead == '\r') ADVANCE(76);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(77);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(74);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(30);
      END_STATE();
    case 7:
      if (lookahead == '=') ADVANCE(22);
      END_STATE();
    case 8:
      if (lookahead == '=') ADVANCE(21);
      END_STATE();
    case 9:
      if (lookahead == 'p') ADVANCE(73);
      END_STATE();
    case 10:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(58);
      END_STATE();
    case 11:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(62);
      END_STATE();
    case 12:
      if (eof) ADVANCE(13);
      if (lookahead == '\n') ADVANCE(78);
      if (lookahead == '\r') ADVANCE(76);
      if (lookahead == '"') ADVANCE(59);
      if (lookahead == '#') ADVANCE(31);
      if (lookahead == '\'') ADVANCE(55);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == ')') ADVANCE(24);
      if (lookahead == '*') ADVANCE(27);
      if (lookahead == '+') ADVANCE(28);
      if (lookahead == ',') ADVANCE(15);
      if (lookahead == '.') ADVANCE(36);
      if (lookahead == '/') ADVANCE(63);
      if (lookahead == ':') ADVANCE(33);
      if (lookahead == ';') ADVANCE(75);
      if (lookahead == '<') ADVANCE(6);
      if (lookahead == '?') ADVANCE(26);
      if (lookahead == 'f') ADVANCE(48);
      if (lookahead == 'g') ADVANCE(50);
      if (lookahead == 'i') ADVANCE(52);
      if (lookahead == '{') ADVANCE(14);
      if (lookahead == '|') ADVANCE(19);
      if (lookahead == '}') ADVANCE(16);
      if (lookahead == '~') ADVANCE(29);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(77);
      if (lookahead == '!' ||
          lookahead == '^') ADVANCE(32);
      if (sym_id_character_set_2(lookahead)) ADVANCE(52);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_grammar);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_fragment);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym__EQ);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_AT_EQ);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_QMARK);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_LT_DASH);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(aux_sym_choice_tag_token1);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead == '=') ADVANCE(22);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == '!') ADVANCE(17);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == '!') ADVANCE(18);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(43);
      if (sym_id_character_set_4(lookahead)) ADVANCE(52);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(46);
      if (sym_id_character_set_4(lookahead)) ADVANCE(52);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'a') ADVANCE(49);
      if (sym_id_character_set_4(lookahead)) ADVANCE(52);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'e') ADVANCE(47);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'g') ADVANCE(44);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(42);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(41);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'm') ADVANCE(45);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'n') ADVANCE(51);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(39);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(37);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 'r') ADVANCE(40);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(sym_id);
      if (lookahead == 't') ADVANCE(38);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_id);
      if (sym_id_character_set_3(lookahead)) ADVANCE(52);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_unsigned);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_unsigned);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(54);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\n') ADVANCE(57);
      if (lookahead == '\r') ADVANCE(58);
      if (lookahead == '\\') ADVANCE(10);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(58);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(58);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\r') ADVANCE(58);
      if (lookahead == '\\') ADVANCE(10);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(58);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead == '\\') ADVANCE(10);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(58);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\n') ADVANCE(61);
      if (lookahead == '\r') ADVANCE(62);
      if (lookahead == '\\') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(62);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(62);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\r') ADVANCE(62);
      if (lookahead == '\\') ADVANCE(11);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(62);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(aux_sym_string_token2);
      if (lookahead == '\\') ADVANCE(11);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(62);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(aux_sym_regex_long_token1);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_LBRACK_CARET);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      if (lookahead == '^') ADVANCE(65);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == '\r') ADVANCE(76);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == '-') ADVANCE(30);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(aux_sym_regex_range_item_token1);
      if (lookahead == 'p') ADVANCE(73);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_BSLASHp);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(aux_sym_regex_set_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(74);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_eos);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_NEWLINE);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_WHITESPACE);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_WHITESPACE);
      if (lookahead == '\r') ADVANCE(76);
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
  [1] = {.lex_state = 12},
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
  [21] = {.lex_state = 12},
  [22] = {.lex_state = 12},
  [23] = {.lex_state = 12},
  [24] = {.lex_state = 12},
  [25] = {.lex_state = 12},
  [26] = {.lex_state = 12},
  [27] = {.lex_state = 12},
  [28] = {.lex_state = 12},
  [29] = {.lex_state = 12},
  [30] = {.lex_state = 12},
  [31] = {.lex_state = 12},
  [32] = {.lex_state = 12},
  [33] = {.lex_state = 12},
  [34] = {.lex_state = 12},
  [35] = {.lex_state = 12},
  [36] = {.lex_state = 12},
  [37] = {.lex_state = 12},
  [38] = {.lex_state = 12},
  [39] = {.lex_state = 12},
  [40] = {.lex_state = 12},
  [41] = {.lex_state = 12},
  [42] = {.lex_state = 12},
  [43] = {.lex_state = 12},
  [44] = {.lex_state = 12},
  [45] = {.lex_state = 12},
  [46] = {.lex_state = 12},
  [47] = {.lex_state = 3},
  [48] = {.lex_state = 12},
  [49] = {.lex_state = 12},
  [50] = {.lex_state = 4},
  [51] = {.lex_state = 12},
  [52] = {.lex_state = 12},
  [53] = {.lex_state = 12},
  [54] = {.lex_state = 12},
  [55] = {.lex_state = 12},
  [56] = {.lex_state = 12},
  [57] = {.lex_state = 2},
  [58] = {.lex_state = 2},
  [59] = {.lex_state = 2},
  [60] = {.lex_state = 12},
  [61] = {.lex_state = 12},
  [62] = {.lex_state = 12},
  [63] = {.lex_state = 2},
  [64] = {.lex_state = 12},
  [65] = {.lex_state = 12},
  [66] = {.lex_state = 12},
  [67] = {.lex_state = 12},
  [68] = {.lex_state = 12},
  [69] = {.lex_state = 12},
  [70] = {.lex_state = 4},
  [71] = {.lex_state = 12},
  [72] = {.lex_state = 12},
  [73] = {.lex_state = 1},
  [74] = {.lex_state = 1},
  [75] = {.lex_state = 12},
  [76] = {.lex_state = 12},
  [77] = {.lex_state = 1},
  [78] = {.lex_state = 12},
  [79] = {.lex_state = 12},
  [80] = {.lex_state = 1},
  [81] = {.lex_state = 12},
  [82] = {.lex_state = 12},
  [83] = {.lex_state = 12},
  [84] = {.lex_state = 12},
  [85] = {.lex_state = 12},
  [86] = {.lex_state = 12},
  [87] = {.lex_state = 12},
  [88] = {.lex_state = 12},
  [89] = {.lex_state = 12},
  [90] = {.lex_state = 12},
  [91] = {.lex_state = 3},
  [92] = {.lex_state = 5},
  [93] = {.lex_state = 56},
  [94] = {.lex_state = 3},
  [95] = {.lex_state = 60},
  [96] = {.lex_state = 12},
  [97] = {.lex_state = 12},
  [98] = {.lex_state = 3},
  [99] = {.lex_state = 3},
  [100] = {.lex_state = 3},
  [101] = {.lex_state = 12},
  [102] = {.lex_state = 12},
  [103] = {.lex_state = 12},
  [104] = {.lex_state = 3},
  [105] = {.lex_state = 12},
  [106] = {.lex_state = 12},
  [107] = {.lex_state = 12},
  [108] = {.lex_state = 12},
  [109] = {.lex_state = 3},
  [110] = {.lex_state = 12},
  [111] = {.lex_state = 12},
  [112] = {.lex_state = 12},
  [113] = {.lex_state = 5},
  [114] = {.lex_state = 12},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_id] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [sym_grammar] = ACTIONS(1),
    [sym_fragment] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_AT_EQ] = ACTIONS(1),
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
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [aux_sym_regex_range_item_token1] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_BSLASHp] = ACTIONS(1),
    [sym_eos] = ACTIONS(1),
    [sym_NEWLINE] = ACTIONS(3),
    [sym_WHITESPACE] = ACTIONS(3),
  },
  [1] = {
    [sym_program] = STATE(107),
    [sym_statement] = STATE(71),
    [sym_grammar_statement] = STATE(75),
    [sym_fragment_statement] = STATE(75),
    [sym_assign_statement] = STATE(75),
    [aux_sym_program_repeat1] = STATE(52),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_id] = ACTIONS(7),
    [sym_grammar] = ACTIONS(9),
    [sym_fragment] = ACTIONS(11),
    [sym_NEWLINE] = ACTIONS(13),
    [sym_WHITESPACE] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(17), 1,
      anon_sym_PIPE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(21), 1,
      anon_sym_CARET,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(29), 1,
      anon_sym_SLASH,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    STATE(18), 1,
      sym__prefix_op,
    STATE(46), 1,
      sym_expression,
    STATE(102), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [65] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(37), 1,
      anon_sym_PIPE,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(56), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [130] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    ACTIONS(43), 1,
      anon_sym_RPAREN,
    STATE(9), 1,
      sym__prefix_op,
    STATE(55), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [195] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    ACTIONS(45), 1,
      anon_sym_RPAREN,
    STATE(9), 1,
      sym__prefix_op,
    STATE(55), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [260] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    ACTIONS(47), 1,
      anon_sym_RPAREN,
    STATE(9), 1,
      sym__prefix_op,
    STATE(55), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [325] = 18,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    ACTIONS(49), 1,
      anon_sym_RPAREN,
    STATE(9), 1,
      sym__prefix_op,
    STATE(55), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [390] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(55), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [452] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(25), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [514] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(48), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [576] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(21), 1,
      anon_sym_CARET,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(29), 1,
      anon_sym_SLASH,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    STATE(18), 1,
      sym__prefix_op,
    STATE(24), 1,
      sym_expression,
    STATE(30), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [638] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(23), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [700] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(49), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [762] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(21), 1,
      anon_sym_CARET,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(29), 1,
      anon_sym_SLASH,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    STATE(18), 1,
      sym__prefix_op,
    STATE(44), 1,
      sym_expression,
    STATE(102), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [824] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(51), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [886] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(24), 1,
      sym_expression,
    STATE(30), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [948] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    ACTIONS(39), 1,
      anon_sym_CARET,
    ACTIONS(41), 1,
      anon_sym_SLASH,
    STATE(9), 1,
      sym__prefix_op,
    STATE(54), 1,
      sym_expression,
    STATE(112), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1010] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(21), 1,
      anon_sym_CARET,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(29), 1,
      anon_sym_SLASH,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    STATE(18), 1,
      sym__prefix_op,
    STATE(25), 1,
      sym_expression,
    STATE(102), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1072] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(21), 1,
      anon_sym_CARET,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(29), 1,
      anon_sym_SLASH,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    STATE(18), 1,
      sym__prefix_op,
    STATE(45), 1,
      sym_expression,
    STATE(102), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1134] = 17,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(19), 1,
      anon_sym_LPAREN,
    ACTIONS(21), 1,
      anon_sym_CARET,
    ACTIONS(23), 1,
      anon_sym_AT,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(29), 1,
      anon_sym_SLASH,
    ACTIONS(31), 1,
      anon_sym_LBRACK_CARET,
    ACTIONS(33), 1,
      anon_sym_LBRACK,
    ACTIONS(35), 1,
      anon_sym_BSLASHp,
    STATE(18), 1,
      sym__prefix_op,
    STATE(23), 1,
      sym_expression,
    STATE(102), 1,
      sym_choice_tag,
    ACTIONS(15), 2,
      sym_id,
      sym_unsigned,
    STATE(36), 5,
      sym_macro_call,
      sym_string,
      sym_regex_long,
      sym_regex_range,
      sym_regex_set,
    STATE(33), 6,
      sym_unary_prefix,
      sym_unary_suffix,
      sym_concat_expression,
      sym_choice_expression,
      sym_field_expr,
      sym_data,
  [1196] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(53), 1,
      sym_id,
    ACTIONS(55), 1,
      aux_sym_choice_tag_token1,
    ACTIONS(57), 1,
      anon_sym_COLON,
    ACTIONS(51), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1227] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(61), 1,
      sym_id,
    ACTIONS(63), 1,
      anon_sym_COLON,
    ACTIONS(59), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1255] = 8,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(67), 1,
      sym_id,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(65), 8,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      sym_eos,
  [1289] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(75), 1,
      sym_id,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(69), 10,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1319] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(79), 1,
      sym_id,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(77), 10,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1349] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(83), 1,
      sym_id,
    ACTIONS(81), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1374] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(87), 1,
      sym_id,
    ACTIONS(85), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1399] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(91), 1,
      sym_id,
    ACTIONS(89), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1424] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(95), 1,
      sym_id,
    ACTIONS(93), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1449] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(99), 1,
      sym_id,
    ACTIONS(97), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1474] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(103), 1,
      sym_id,
    ACTIONS(101), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1499] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(107), 1,
      sym_id,
    ACTIONS(105), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1524] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(111), 1,
      sym_id,
    ACTIONS(109), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1549] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(115), 1,
      sym_id,
    ACTIONS(113), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1574] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(119), 1,
      sym_id,
    ACTIONS(117), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1599] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(123), 1,
      sym_id,
    ACTIONS(121), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1624] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(127), 1,
      sym_id,
    ACTIONS(125), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1649] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(131), 1,
      sym_id,
    ACTIONS(129), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1674] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(135), 1,
      sym_id,
    ACTIONS(133), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1699] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(139), 1,
      sym_id,
    ACTIONS(137), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1724] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(143), 1,
      sym_id,
    ACTIONS(141), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1749] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(147), 1,
      sym_id,
    ACTIONS(145), 13,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1774] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(151), 1,
      sym_id,
    ACTIONS(153), 1,
      aux_sym_regex_long_token1,
    ACTIONS(149), 11,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      anon_sym_PIPE,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_LT_DASH,
      anon_sym_POUND,
      sym_eos,
  [1800] = 11,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(157), 1,
      sym_id,
    ACTIONS(159), 1,
      anon_sym_TILDE,
    ACTIONS(161), 1,
      anon_sym_LT_DASH,
    ACTIONS(163), 1,
      sym_eos,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(155), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [1838] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(159), 1,
      anon_sym_TILDE,
    ACTIONS(167), 1,
      sym_id,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(165), 5,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
      anon_sym_LT_DASH,
      sym_eos,
  [1872] = 11,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(159), 1,
      anon_sym_TILDE,
    ACTIONS(161), 1,
      anon_sym_LT_DASH,
    ACTIONS(171), 1,
      sym_id,
    ACTIONS(173), 1,
      sym_eos,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(169), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [1910] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(177), 1,
      anon_sym_LBRACK,
    ACTIONS(175), 11,
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
  [1933] = 10,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(179), 1,
      anon_sym_COMMA,
    ACTIONS(181), 1,
      anon_sym_TILDE,
    ACTIONS(183), 1,
      anon_sym_LT_DASH,
    STATE(28), 1,
      sym__suffix_op,
    STATE(89), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [1966] = 10,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(181), 1,
      anon_sym_TILDE,
    ACTIONS(183), 1,
      anon_sym_LT_DASH,
    ACTIONS(185), 1,
      anon_sym_COMMA,
    STATE(28), 1,
      sym__suffix_op,
    STATE(85), 1,
      aux_sym_macro_call_repeat1,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [1999] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(187), 1,
      aux_sym_regex_long_token1,
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
  [2020] = 8,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(181), 1,
      anon_sym_TILDE,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(165), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LT_DASH,
  [2049] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(7), 1,
      sym_id,
    ACTIONS(9), 1,
      sym_grammar,
    ACTIONS(11), 1,
      sym_fragment,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(189), 1,
      ts_builtin_sym_end,
    STATE(53), 1,
      aux_sym_program_repeat1,
    STATE(71), 1,
      sym_statement,
    STATE(75), 3,
      sym_grammar_statement,
      sym_fragment_statement,
      sym_assign_statement,
  [2079] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(191), 1,
      ts_builtin_sym_end,
    ACTIONS(193), 1,
      sym_id,
    ACTIONS(196), 1,
      sym_grammar,
    ACTIONS(199), 1,
      sym_fragment,
    STATE(53), 1,
      aux_sym_program_repeat1,
    STATE(71), 1,
      sym_statement,
    STATE(75), 3,
      sym_grammar_statement,
      sym_fragment_statement,
      sym_assign_statement,
  [2109] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(181), 1,
      anon_sym_TILDE,
    ACTIONS(183), 1,
      anon_sym_LT_DASH,
    ACTIONS(202), 1,
      anon_sym_RPAREN,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2139] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(181), 1,
      anon_sym_TILDE,
    ACTIONS(183), 1,
      anon_sym_LT_DASH,
    ACTIONS(204), 1,
      anon_sym_COMMA,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2169] = 9,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(69), 1,
      anon_sym_PIPE,
    ACTIONS(73), 1,
      anon_sym_POUND,
    ACTIONS(181), 1,
      anon_sym_TILDE,
    ACTIONS(183), 1,
      anon_sym_LT_DASH,
    ACTIONS(206), 1,
      anon_sym_RPAREN,
    STATE(28), 1,
      sym__suffix_op,
    ACTIONS(71), 3,
      anon_sym_QMARK,
      anon_sym_STAR,
      anon_sym_PLUS,
  [2199] = 7,
    ACTIONS(208), 1,
      anon_sym_RBRACK,
    ACTIONS(210), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(212), 1,
      anon_sym_BSLASHp,
    STATE(58), 1,
      aux_sym_regex_range_repeat1,
    STATE(73), 1,
      sym_regex_range_item,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(80), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [2223] = 7,
    ACTIONS(210), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(212), 1,
      anon_sym_BSLASHp,
    ACTIONS(214), 1,
      anon_sym_RBRACK,
    STATE(59), 1,
      aux_sym_regex_range_repeat1,
    STATE(73), 1,
      sym_regex_range_item,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(80), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [2247] = 7,
    ACTIONS(216), 1,
      anon_sym_RBRACK,
    ACTIONS(218), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(221), 1,
      anon_sym_BSLASHp,
    STATE(59), 1,
      aux_sym_regex_range_repeat1,
    STATE(73), 1,
      sym_regex_range_item,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(80), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [2271] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(226), 1,
      sym_id,
    ACTIONS(228), 1,
      anon_sym_LBRACE,
    ACTIONS(230), 1,
      sym_eos,
    ACTIONS(224), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2292] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(234), 1,
      sym_id,
    ACTIONS(236), 1,
      sym_eos,
    ACTIONS(232), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2310] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(240), 1,
      sym_id,
    ACTIONS(242), 1,
      sym_eos,
    ACTIONS(238), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2328] = 5,
    ACTIONS(210), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(212), 1,
      anon_sym_BSLASHp,
    STATE(77), 1,
      sym_regex_range_item,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    STATE(80), 2,
      sym_regex_range_item_group,
      sym_regex_set,
  [2346] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(246), 1,
      sym_id,
    ACTIONS(248), 1,
      sym_eos,
    ACTIONS(244), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2364] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(252), 1,
      sym_id,
    ACTIONS(254), 1,
      sym_eos,
    ACTIONS(250), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2382] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(258), 1,
      sym_id,
    ACTIONS(256), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2397] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(260), 1,
      anon_sym_RBRACE,
    STATE(108), 1,
      sym_string,
  [2416] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(264), 1,
      sym_id,
    ACTIONS(262), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2431] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(268), 1,
      sym_id,
    ACTIONS(266), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2446] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    STATE(2), 1,
      sym_eq,
    ACTIONS(270), 3,
      anon_sym_EQ,
      anon_sym__EQ,
      anon_sym_AT_EQ,
  [2461] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(274), 1,
      sym_id,
    ACTIONS(272), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2476] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(276), 1,
      anon_sym_RBRACE,
    STATE(108), 1,
      sym_string,
  [2495] = 4,
    ACTIONS(280), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(282), 1,
      anon_sym_DASH,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(278), 2,
      anon_sym_RBRACK,
      anon_sym_BSLASHp,
  [2510] = 3,
    ACTIONS(135), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(133), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [2523] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(286), 1,
      sym_id,
    ACTIONS(284), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2538] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(290), 1,
      sym_id,
    ACTIONS(288), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2553] = 3,
    ACTIONS(294), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(292), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [2566] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(298), 1,
      sym_id,
    ACTIONS(296), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2581] = 6,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    ACTIONS(300), 1,
      anon_sym_RBRACE,
    STATE(87), 1,
      sym_string,
  [2600] = 3,
    ACTIONS(304), 1,
      aux_sym_regex_range_item_token1,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
    ACTIONS(302), 3,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BSLASHp,
  [2613] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(308), 1,
      sym_id,
    ACTIONS(306), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2628] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(312), 1,
      sym_id,
    ACTIONS(310), 3,
      ts_builtin_sym_end,
      sym_grammar,
      sym_fragment,
  [2643] = 5,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(25), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_DQUOTE,
    STATE(108), 1,
      sym_string,
  [2659] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(314), 1,
      anon_sym_COMMA,
    STATE(84), 1,
      aux_sym__grammar_exts_repeat1,
  [2672] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(317), 1,
      anon_sym_COMMA,
    STATE(90), 1,
      aux_sym_macro_call_repeat1,
  [2685] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(319), 1,
      anon_sym_COMMA,
    STATE(84), 1,
      aux_sym__grammar_exts_repeat1,
  [2698] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(321), 1,
      anon_sym_COMMA,
    STATE(86), 1,
      aux_sym__grammar_exts_repeat1,
  [2711] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(323), 1,
      anon_sym_LPAREN,
    ACTIONS(325), 1,
      anon_sym_DOT,
  [2724] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(327), 1,
      anon_sym_COMMA,
    STATE(90), 1,
      aux_sym_macro_call_repeat1,
  [2737] = 4,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(329), 1,
      anon_sym_COMMA,
    STATE(90), 1,
      aux_sym_macro_call_repeat1,
  [2750] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(332), 1,
      sym_id,
  [2760] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(334), 1,
      aux_sym_regex_set_token1,
  [2770] = 2,
    ACTIONS(336), 1,
      aux_sym_string_token1,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
  [2778] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(338), 1,
      sym_id,
  [2788] = 2,
    ACTIONS(340), 1,
      aux_sym_string_token2,
    ACTIONS(3), 2,
      sym_NEWLINE,
      sym_WHITESPACE,
  [2796] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(342), 1,
      anon_sym_DQUOTE,
  [2806] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(342), 1,
      anon_sym_SQUOTE,
  [2816] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(344), 1,
      sym_id,
  [2826] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(346), 1,
      sym_id,
  [2836] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(348), 1,
      sym_id,
  [2846] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(350), 1,
      anon_sym_LPAREN,
  [2856] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(352), 1,
      anon_sym_PIPE,
  [2866] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(354), 1,
      anon_sym_LBRACE,
  [2876] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(356), 1,
      sym_id,
  [2886] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(358), 1,
      anon_sym_SLASH,
  [2896] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(360), 1,
      anon_sym_RBRACE,
  [2906] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(362), 1,
      ts_builtin_sym_end,
  [2916] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(364), 1,
      anon_sym_COMMA,
  [2926] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(366), 1,
      sym_id,
  [2936] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(368), 1,
      anon_sym_RBRACE,
  [2946] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(370), 1,
      anon_sym_SLASH,
  [2956] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(372), 1,
      anon_sym_PIPE,
  [2966] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(374), 1,
      aux_sym_regex_set_token1,
  [2976] = 3,
    ACTIONS(3), 1,
      sym_WHITESPACE,
    ACTIONS(13), 1,
      sym_NEWLINE,
    ACTIONS(376), 1,
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
  [SMALL_STATE(22)] = 1227,
  [SMALL_STATE(23)] = 1255,
  [SMALL_STATE(24)] = 1289,
  [SMALL_STATE(25)] = 1319,
  [SMALL_STATE(26)] = 1349,
  [SMALL_STATE(27)] = 1374,
  [SMALL_STATE(28)] = 1399,
  [SMALL_STATE(29)] = 1424,
  [SMALL_STATE(30)] = 1449,
  [SMALL_STATE(31)] = 1474,
  [SMALL_STATE(32)] = 1499,
  [SMALL_STATE(33)] = 1524,
  [SMALL_STATE(34)] = 1549,
  [SMALL_STATE(35)] = 1574,
  [SMALL_STATE(36)] = 1599,
  [SMALL_STATE(37)] = 1624,
  [SMALL_STATE(38)] = 1649,
  [SMALL_STATE(39)] = 1674,
  [SMALL_STATE(40)] = 1699,
  [SMALL_STATE(41)] = 1724,
  [SMALL_STATE(42)] = 1749,
  [SMALL_STATE(43)] = 1774,
  [SMALL_STATE(44)] = 1800,
  [SMALL_STATE(45)] = 1838,
  [SMALL_STATE(46)] = 1872,
  [SMALL_STATE(47)] = 1910,
  [SMALL_STATE(48)] = 1933,
  [SMALL_STATE(49)] = 1966,
  [SMALL_STATE(50)] = 1999,
  [SMALL_STATE(51)] = 2020,
  [SMALL_STATE(52)] = 2049,
  [SMALL_STATE(53)] = 2079,
  [SMALL_STATE(54)] = 2109,
  [SMALL_STATE(55)] = 2139,
  [SMALL_STATE(56)] = 2169,
  [SMALL_STATE(57)] = 2199,
  [SMALL_STATE(58)] = 2223,
  [SMALL_STATE(59)] = 2247,
  [SMALL_STATE(60)] = 2271,
  [SMALL_STATE(61)] = 2292,
  [SMALL_STATE(62)] = 2310,
  [SMALL_STATE(63)] = 2328,
  [SMALL_STATE(64)] = 2346,
  [SMALL_STATE(65)] = 2364,
  [SMALL_STATE(66)] = 2382,
  [SMALL_STATE(67)] = 2397,
  [SMALL_STATE(68)] = 2416,
  [SMALL_STATE(69)] = 2431,
  [SMALL_STATE(70)] = 2446,
  [SMALL_STATE(71)] = 2461,
  [SMALL_STATE(72)] = 2476,
  [SMALL_STATE(73)] = 2495,
  [SMALL_STATE(74)] = 2510,
  [SMALL_STATE(75)] = 2523,
  [SMALL_STATE(76)] = 2538,
  [SMALL_STATE(77)] = 2553,
  [SMALL_STATE(78)] = 2566,
  [SMALL_STATE(79)] = 2581,
  [SMALL_STATE(80)] = 2600,
  [SMALL_STATE(81)] = 2613,
  [SMALL_STATE(82)] = 2628,
  [SMALL_STATE(83)] = 2643,
  [SMALL_STATE(84)] = 2659,
  [SMALL_STATE(85)] = 2672,
  [SMALL_STATE(86)] = 2685,
  [SMALL_STATE(87)] = 2698,
  [SMALL_STATE(88)] = 2711,
  [SMALL_STATE(89)] = 2724,
  [SMALL_STATE(90)] = 2737,
  [SMALL_STATE(91)] = 2750,
  [SMALL_STATE(92)] = 2760,
  [SMALL_STATE(93)] = 2770,
  [SMALL_STATE(94)] = 2778,
  [SMALL_STATE(95)] = 2788,
  [SMALL_STATE(96)] = 2796,
  [SMALL_STATE(97)] = 2806,
  [SMALL_STATE(98)] = 2816,
  [SMALL_STATE(99)] = 2826,
  [SMALL_STATE(100)] = 2836,
  [SMALL_STATE(101)] = 2846,
  [SMALL_STATE(102)] = 2856,
  [SMALL_STATE(103)] = 2866,
  [SMALL_STATE(104)] = 2876,
  [SMALL_STATE(105)] = 2886,
  [SMALL_STATE(106)] = 2896,
  [SMALL_STATE(107)] = 2906,
  [SMALL_STATE(108)] = 2916,
  [SMALL_STATE(109)] = 2926,
  [SMALL_STATE(110)] = 2936,
  [SMALL_STATE(111)] = 2946,
  [SMALL_STATE(112)] = 2956,
  [SMALL_STATE(113)] = 2966,
  [SMALL_STATE(114)] = 2976,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 0),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(70),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(57),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 3, .production_id = 12),
  [53] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 3, .production_id = 12),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 4, .production_id = 17),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 4, .production_id = 17),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_concat_expression, 3, .production_id = 11),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_concat_expression, 3, .production_id = 11),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 1, .production_id = 5),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [75] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 1, .production_id = 5),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_prefix, 2, .production_id = 10),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_prefix, 2, .production_id = 10),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 3),
  [83] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 3),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3),
  [87] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_suffix, 2, .production_id = 9),
  [91] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_suffix, 2, .production_id = 9),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 5, .production_id = 19),
  [95] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 5, .production_id = 19),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_expression, 3, .production_id = 11),
  [99] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_expression, 3, .production_id = 11),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 9, .production_id = 22),
  [103] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 9, .production_id = 22),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 6, .production_id = 20),
  [107] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 6, .production_id = 20),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1),
  [111] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 1),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 8, .production_id = 22),
  [115] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 8, .production_id = 22),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range, 3, .production_id = 8),
  [119] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range, 3, .production_id = 8),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_data, 1),
  [123] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_data, 1),
  [125] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 4),
  [127] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 4),
  [129] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_long, 3),
  [131] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_long, 3),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_set, 4, .production_id = 16),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_set, 4, .production_id = 16),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_macro_call, 7, .production_id = 20),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_macro_call, 7, .production_id = 20),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice_tag, 6, .production_id = 21),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_choice_tag, 6, .production_id = 21),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range, 2, .production_id = 8),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range, 2, .production_id = 8),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_long, 2),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_long, 2),
  [153] = {.entry = {.count = 1, .reusable = false}}, SHIFT(38),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 4, .production_id = 7),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 4, .production_id = 7),
  [159] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [161] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_expr, 3, .production_id = 11),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_expr, 3, .production_id = 11),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 3, .production_id = 6),
  [171] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 3, .production_id = 6),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_eq, 1),
  [177] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_eq, 1),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 1, .production_id = 2),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 4),
  [193] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 4), SHIFT_REPEAT(70),
  [196] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 4), SHIFT_REPEAT(100),
  [199] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, .production_id = 4), SHIFT_REPEAT(109),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [204] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_macro_call_repeat1, 2),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [210] = {.entry = {.count = 1, .reusable = false}}, SHIFT(80),
  [212] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [214] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 2),
  [218] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_regex_range_repeat1, 2), SHIFT_REPEAT(80),
  [221] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 2), SHIFT_REPEAT(114),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 2, .production_id = 3),
  [226] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 2, .production_id = 3),
  [228] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [230] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [232] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 7, .production_id = 18),
  [234] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 7, .production_id = 18),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [238] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment_statement, 2, .production_id = 3),
  [240] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fragment_statement, 2, .production_id = 3),
  [242] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [244] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 6, .production_id = 13),
  [246] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 6, .production_id = 13),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [250] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 4, .production_id = 3),
  [252] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 4, .production_id = 3),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [256] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 4, .production_id = 6),
  [258] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 4, .production_id = 6),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [262] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment_statement, 3, .production_id = 3),
  [264] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fragment_statement, 3, .production_id = 3),
  [266] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 3, .production_id = 3),
  [268] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 3, .production_id = 3),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [272] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 1, .production_id = 1),
  [274] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_program_repeat1, 1, .production_id = 1),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [278] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_regex_range_repeat1, 1),
  [280] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_regex_range_repeat1, 1),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [284] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 1),
  [286] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 1),
  [288] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 8, .production_id = 18),
  [290] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 8, .production_id = 18),
  [292] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range_item_group, 3, .production_id = 11),
  [294] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range_item_group, 3, .production_id = 11),
  [296] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 5, .production_id = 3),
  [298] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 5, .production_id = 3),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [302] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_regex_range_item, 1),
  [304] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_regex_range_item, 1),
  [306] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grammar_statement, 7, .production_id = 13),
  [308] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grammar_statement, 7, .production_id = 13),
  [310] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assign_statement, 5, .production_id = 7),
  [312] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assign_statement, 5, .production_id = 7),
  [314] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__grammar_exts_repeat1, 2, .production_id = 15), SHIFT_REPEAT(83),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [323] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [327] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [329] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_macro_call_repeat1, 2), SHIFT_REPEAT(8),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [344] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [346] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [348] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [352] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [356] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [358] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [360] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [362] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__grammar_exts_repeat1, 2, .production_id = 14),
  [366] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [368] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [374] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [376] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
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
