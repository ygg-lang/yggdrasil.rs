grammar YggdrasilAntlr;
import YggdrasilBasic;

// $antlr-format useTab false, columnLimit 144
// $antlr-format alignColons hanging, alignSemicolons hanging, alignFirstTokens true
program
    : (
        define_grammar
        | import_statement
        | define_class
        | define_union
        | define_climb
        | define_token
        | SEMICOLON
    )* EOF
    ;
// =================================================================================================
import_statement: KW_IMPORT (identifier | string) import_block?;
import_block:     BRACE_L identifier* BRACE_R;
// =================================================================================================
define_grammar: KW_GRAMMAR identifier grammar_block;
grammar_block:  BRACE_L BRACE_R;
// =================================================================================================
define_class: macro_call* (mods += identifier)* KW_CLASS name = identifier class_block;
class_block:  BRACE_L OP_OR? class_expression* BRACE_R;
class_expression
    : class_expression suffix                                 # CSuffix
    | identifier COLON class_expression                       # CETag
    | OP_UNTAG class_expression                               # CUntag
    | OP_NOT class_expression                                 # CNot
    | lhs = class_expression OP_CONCAT rhs = class_expression # CHard
    | lhs = class_expression rhs = class_expression           # CSoft
    | lhs = class_expression OP_OR rhs = class_expression     # CPattern
    | PARENTHESES_L OP_OR? class_expression PARENTHESES_R     # CGroup
    | tuple_call                                              # CCall
    | atomic                                                  # Atom
    ;
// =================================================================================================
define_union: macro_call* (mods += identifier)* KW_UNION name = identifier union_block;
union_block:  BRACE_L union_term* BRACE_R;
union_term:   OP_OR union_expression* tag_branch?;
union_expression
    : union_expression suffix                                 # USuffix
    | identifier COLON union_expression                       # UETag
    | OP_UNTAG union_expression                               # UUntag
    | OP_NOT union_expression                                 # UNot
    | lhs = union_expression OP_CONCAT rhs = union_expression # UHard
    | lhs = union_expression rhs = union_expression           # USoft
    | PARENTHESES_L OP_OR? class_expression PARENTHESES_R     # UGroup
    | tuple_call                                              # UCall
    | atomic                                                  # Utom
    ;
// =================================================================================================
define_climb: macro_call* (mods += identifier)* KW_CLIMB name = identifier union_block;
tag_branch:   OP_HASH identifier OP_GT?;
// =================================================================================================
define_token: macro_call* (mods += identifier)* KW_TOKEN name = identifier? token_block;
token_block:  BRACE_L (token_pair | SEMICOLON)* BRACE_R;
token_pair:   macro_call* identifier COLON token_expression;

token_expression: token_expression OP_OR token_expression # TOr | atomic # TAtom;
// =================================================================================================
macro_call: (OP_HASH | OP_AT) namepath tuple_block?;
// =================================================================================================
tuple_call:  OP_AT namepath tuple_block?;
tuple_block: PARENTHESES_L (class_expression (COMMA class_expression)* COMMA?)? PARENTHESES_R;
// =================================================================================================
suffix
    : MATCH_OPTIONAL             # Optional
    | MATCH_MAYBE                # MaybeGreedy
    | MATCH_MAYBE MATCH_OPTIONAL # Maybe
    | MATCH_MANY                 # ManyGreedy
    | MATCH_MANY MATCH_OPTIONAL  # Many
    ;
// =================================================================================================
atomic
    : tuple_call # ATuple
    | string     # AString
    | identifier # AId
    | regex      # ARe
    | INTEGER    # AInt
    | SPECIAL    # ASpecial
    | ESCAPED    # AChar
    ;
regex:      REGEX_RANGE | REGEX_FREE;
namepath:   identifier ((OP_PROPORTION | DOT) identifier)*;
string:     STRING_SINGLE | STRING_DOUBLE;
identifier: RAW_ID | UNICODE_ID;