#![allow(nonstandard_style)]
// Generated from YggdrasilAntlr.g4 by ANTLR 4.8
use super::yggdrasilantlrparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait YggdrasilAntlrListener<'input>: ParseTreeListener<'input, YggdrasilAntlrParserContextType> {
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#program}.
    /// @param ctx the parse tree
    fn enter_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#program}.
    /// @param ctx the parse tree
    fn exit_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#import_statement}.
    /// @param ctx the parse tree
    fn enter_import_statement(&mut self, _ctx: &Import_statementContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#import_statement}.
    /// @param ctx the parse tree
    fn exit_import_statement(&mut self, _ctx: &Import_statementContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#import_block}.
    /// @param ctx the parse tree
    fn enter_import_block(&mut self, _ctx: &Import_blockContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#import_block}.
    /// @param ctx the parse tree
    fn exit_import_block(&mut self, _ctx: &Import_blockContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#define_grammar}.
    /// @param ctx the parse tree
    fn enter_define_grammar(&mut self, _ctx: &Define_grammarContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#define_grammar}.
    /// @param ctx the parse tree
    fn exit_define_grammar(&mut self, _ctx: &Define_grammarContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#grammar_block}.
    /// @param ctx the parse tree
    fn enter_grammar_block(&mut self, _ctx: &Grammar_blockContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#grammar_block}.
    /// @param ctx the parse tree
    fn exit_grammar_block(&mut self, _ctx: &Grammar_blockContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#define_class}.
    /// @param ctx the parse tree
    fn enter_define_class(&mut self, _ctx: &Define_classContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#define_class}.
    /// @param ctx the parse tree
    fn exit_define_class(&mut self, _ctx: &Define_classContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#class_block}.
    /// @param ctx the parse tree
    fn enter_class_block(&mut self, _ctx: &Class_blockContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#class_block}.
    /// @param ctx the parse tree
    fn exit_class_block(&mut self, _ctx: &Class_blockContext<'input>) {}
    /// Enter a parse tree produced by the {@code CSuffix}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CSuffix(&mut self, _ctx: &CSuffixContext<'input>) {}
    /// Exit a parse tree produced by the {@code CSuffix}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CSuffix(&mut self, _ctx: &CSuffixContext<'input>) {}
    /// Enter a parse tree produced by the {@code CCall}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CCall(&mut self, _ctx: &CCallContext<'input>) {}
    /// Exit a parse tree produced by the {@code CCall}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CCall(&mut self, _ctx: &CCallContext<'input>) {}
    /// Enter a parse tree produced by the {@code CETag}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CETag(&mut self, _ctx: &CETagContext<'input>) {}
    /// Exit a parse tree produced by the {@code CETag}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CETag(&mut self, _ctx: &CETagContext<'input>) {}
    /// Enter a parse tree produced by the {@code CUntag}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CUntag(&mut self, _ctx: &CUntagContext<'input>) {}
    /// Exit a parse tree produced by the {@code CUntag}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CUntag(&mut self, _ctx: &CUntagContext<'input>) {}
    /// Enter a parse tree produced by the {@code CSoft}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CSoft(&mut self, _ctx: &CSoftContext<'input>) {}
    /// Exit a parse tree produced by the {@code CSoft}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CSoft(&mut self, _ctx: &CSoftContext<'input>) {}
    /// Enter a parse tree produced by the {@code CHard}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CHard(&mut self, _ctx: &CHardContext<'input>) {}
    /// Exit a parse tree produced by the {@code CHard}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CHard(&mut self, _ctx: &CHardContext<'input>) {}
    /// Enter a parse tree produced by the {@code CPattern}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CPattern(&mut self, _ctx: &CPatternContext<'input>) {}
    /// Exit a parse tree produced by the {@code CPattern}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CPattern(&mut self, _ctx: &CPatternContext<'input>) {}
    /// Enter a parse tree produced by the {@code CGroup}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CGroup(&mut self, _ctx: &CGroupContext<'input>) {}
    /// Exit a parse tree produced by the {@code CGroup}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CGroup(&mut self, _ctx: &CGroupContext<'input>) {}
    /// Enter a parse tree produced by the {@code Atom}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_Atom(&mut self, _ctx: &AtomContext<'input>) {}
    /// Exit a parse tree produced by the {@code Atom}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_Atom(&mut self, _ctx: &AtomContext<'input>) {}
    /// Enter a parse tree produced by the {@code CNot}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn enter_CNot(&mut self, _ctx: &CNotContext<'input>) {}
    /// Exit a parse tree produced by the {@code CNot}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn exit_CNot(&mut self, _ctx: &CNotContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#define_union}.
    /// @param ctx the parse tree
    fn enter_define_union(&mut self, _ctx: &Define_unionContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#define_union}.
    /// @param ctx the parse tree
    fn exit_define_union(&mut self, _ctx: &Define_unionContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#union_block}.
    /// @param ctx the parse tree
    fn enter_union_block(&mut self, _ctx: &Union_blockContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#union_block}.
    /// @param ctx the parse tree
    fn exit_union_block(&mut self, _ctx: &Union_blockContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#union_term}.
    /// @param ctx the parse tree
    fn enter_union_term(&mut self, _ctx: &Union_termContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#union_term}.
    /// @param ctx the parse tree
    fn exit_union_term(&mut self, _ctx: &Union_termContext<'input>) {}
    /// Enter a parse tree produced by the {@code UHard}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_UHard(&mut self, _ctx: &UHardContext<'input>) {}
    /// Exit a parse tree produced by the {@code UHard}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_UHard(&mut self, _ctx: &UHardContext<'input>) {}
    /// Enter a parse tree produced by the {@code UUntag}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_UUntag(&mut self, _ctx: &UUntagContext<'input>) {}
    /// Exit a parse tree produced by the {@code UUntag}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_UUntag(&mut self, _ctx: &UUntagContext<'input>) {}
    /// Enter a parse tree produced by the {@code USuffix}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_USuffix(&mut self, _ctx: &USuffixContext<'input>) {}
    /// Exit a parse tree produced by the {@code USuffix}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_USuffix(&mut self, _ctx: &USuffixContext<'input>) {}
    /// Enter a parse tree produced by the {@code UGroup}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_UGroup(&mut self, _ctx: &UGroupContext<'input>) {}
    /// Exit a parse tree produced by the {@code UGroup}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_UGroup(&mut self, _ctx: &UGroupContext<'input>) {}
    /// Enter a parse tree produced by the {@code UETag}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_UETag(&mut self, _ctx: &UETagContext<'input>) {}
    /// Exit a parse tree produced by the {@code UETag}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_UETag(&mut self, _ctx: &UETagContext<'input>) {}
    /// Enter a parse tree produced by the {@code Utom}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_Utom(&mut self, _ctx: &UtomContext<'input>) {}
    /// Exit a parse tree produced by the {@code Utom}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_Utom(&mut self, _ctx: &UtomContext<'input>) {}
    /// Enter a parse tree produced by the {@code UNot}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_UNot(&mut self, _ctx: &UNotContext<'input>) {}
    /// Exit a parse tree produced by the {@code UNot}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_UNot(&mut self, _ctx: &UNotContext<'input>) {}
    /// Enter a parse tree produced by the {@code USoft}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_USoft(&mut self, _ctx: &USoftContext<'input>) {}
    /// Exit a parse tree produced by the {@code USoft}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_USoft(&mut self, _ctx: &USoftContext<'input>) {}
    /// Enter a parse tree produced by the {@code UCall}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn enter_UCall(&mut self, _ctx: &UCallContext<'input>) {}
    /// Exit a parse tree produced by the {@code UCall}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn exit_UCall(&mut self, _ctx: &UCallContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#define_climb}.
    /// @param ctx the parse tree
    fn enter_define_climb(&mut self, _ctx: &Define_climbContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#define_climb}.
    /// @param ctx the parse tree
    fn exit_define_climb(&mut self, _ctx: &Define_climbContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#tag_branch}.
    /// @param ctx the parse tree
    fn enter_tag_branch(&mut self, _ctx: &Tag_branchContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#tag_branch}.
    /// @param ctx the parse tree
    fn exit_tag_branch(&mut self, _ctx: &Tag_branchContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#define_token}.
    /// @param ctx the parse tree
    fn enter_define_token(&mut self, _ctx: &Define_tokenContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#define_token}.
    /// @param ctx the parse tree
    fn exit_define_token(&mut self, _ctx: &Define_tokenContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#token_block}.
    /// @param ctx the parse tree
    fn enter_token_block(&mut self, _ctx: &Token_blockContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#token_block}.
    /// @param ctx the parse tree
    fn exit_token_block(&mut self, _ctx: &Token_blockContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#token_pair}.
    /// @param ctx the parse tree
    fn enter_token_pair(&mut self, _ctx: &Token_pairContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#token_pair}.
    /// @param ctx the parse tree
    fn exit_token_pair(&mut self, _ctx: &Token_pairContext<'input>) {}
    /// Enter a parse tree produced by the {@code TOr}
    /// labeled alternative in {@link YggdrasilAntlrParser#token_expression}.
    /// @param ctx the parse tree
    fn enter_TOr(&mut self, _ctx: &TOrContext<'input>) {}
    /// Exit a parse tree produced by the {@code TOr}
    /// labeled alternative in {@link YggdrasilAntlrParser#token_expression}.
    /// @param ctx the parse tree
    fn exit_TOr(&mut self, _ctx: &TOrContext<'input>) {}
    /// Enter a parse tree produced by the {@code TAtom}
    /// labeled alternative in {@link YggdrasilAntlrParser#token_expression}.
    /// @param ctx the parse tree
    fn enter_TAtom(&mut self, _ctx: &TAtomContext<'input>) {}
    /// Exit a parse tree produced by the {@code TAtom}
    /// labeled alternative in {@link YggdrasilAntlrParser#token_expression}.
    /// @param ctx the parse tree
    fn exit_TAtom(&mut self, _ctx: &TAtomContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#macro_call}.
    /// @param ctx the parse tree
    fn enter_macro_call(&mut self, _ctx: &Macro_callContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#macro_call}.
    /// @param ctx the parse tree
    fn exit_macro_call(&mut self, _ctx: &Macro_callContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#tuple_call}.
    /// @param ctx the parse tree
    fn enter_tuple_call(&mut self, _ctx: &Tuple_callContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#tuple_call}.
    /// @param ctx the parse tree
    fn exit_tuple_call(&mut self, _ctx: &Tuple_callContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#tuple_block}.
    /// @param ctx the parse tree
    fn enter_tuple_block(&mut self, _ctx: &Tuple_blockContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#tuple_block}.
    /// @param ctx the parse tree
    fn exit_tuple_block(&mut self, _ctx: &Tuple_blockContext<'input>) {}
    /// Enter a parse tree produced by the {@code Optional}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn enter_Optional(&mut self, _ctx: &OptionalContext<'input>) {}
    /// Exit a parse tree produced by the {@code Optional}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn exit_Optional(&mut self, _ctx: &OptionalContext<'input>) {}
    /// Enter a parse tree produced by the {@code MaybeGreedy}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn enter_MaybeGreedy(&mut self, _ctx: &MaybeGreedyContext<'input>) {}
    /// Exit a parse tree produced by the {@code MaybeGreedy}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn exit_MaybeGreedy(&mut self, _ctx: &MaybeGreedyContext<'input>) {}
    /// Enter a parse tree produced by the {@code Maybe}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn enter_Maybe(&mut self, _ctx: &MaybeContext<'input>) {}
    /// Exit a parse tree produced by the {@code Maybe}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn exit_Maybe(&mut self, _ctx: &MaybeContext<'input>) {}
    /// Enter a parse tree produced by the {@code ManyGreedy}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn enter_ManyGreedy(&mut self, _ctx: &ManyGreedyContext<'input>) {}
    /// Exit a parse tree produced by the {@code ManyGreedy}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn exit_ManyGreedy(&mut self, _ctx: &ManyGreedyContext<'input>) {}
    /// Enter a parse tree produced by the {@code Many}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn enter_Many(&mut self, _ctx: &ManyContext<'input>) {}
    /// Exit a parse tree produced by the {@code Many}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn exit_Many(&mut self, _ctx: &ManyContext<'input>) {}
    /// Enter a parse tree produced by the {@code ATuple}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn enter_ATuple(&mut self, _ctx: &ATupleContext<'input>) {}
    /// Exit a parse tree produced by the {@code ATuple}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn exit_ATuple(&mut self, _ctx: &ATupleContext<'input>) {}
    /// Enter a parse tree produced by the {@code AString}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn enter_AString(&mut self, _ctx: &AStringContext<'input>) {}
    /// Exit a parse tree produced by the {@code AString}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn exit_AString(&mut self, _ctx: &AStringContext<'input>) {}
    /// Enter a parse tree produced by the {@code AId}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn enter_AId(&mut self, _ctx: &AIdContext<'input>) {}
    /// Exit a parse tree produced by the {@code AId}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn exit_AId(&mut self, _ctx: &AIdContext<'input>) {}
    /// Enter a parse tree produced by the {@code ARe}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn enter_ARe(&mut self, _ctx: &AReContext<'input>) {}
    /// Exit a parse tree produced by the {@code ARe}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn exit_ARe(&mut self, _ctx: &AReContext<'input>) {}
    /// Enter a parse tree produced by the {@code AInt}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn enter_AInt(&mut self, _ctx: &AIntContext<'input>) {}
    /// Exit a parse tree produced by the {@code AInt}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn exit_AInt(&mut self, _ctx: &AIntContext<'input>) {}
    /// Enter a parse tree produced by the {@code ASpecial}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn enter_ASpecial(&mut self, _ctx: &ASpecialContext<'input>) {}
    /// Exit a parse tree produced by the {@code ASpecial}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn exit_ASpecial(&mut self, _ctx: &ASpecialContext<'input>) {}
    /// Enter a parse tree produced by the {@code AChar}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn enter_AChar(&mut self, _ctx: &ACharContext<'input>) {}
    /// Exit a parse tree produced by the {@code AChar}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn exit_AChar(&mut self, _ctx: &ACharContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#regex}.
    /// @param ctx the parse tree
    fn enter_regex(&mut self, _ctx: &RegexContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#regex}.
    /// @param ctx the parse tree
    fn exit_regex(&mut self, _ctx: &RegexContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#namepath}.
    /// @param ctx the parse tree
    fn enter_namepath(&mut self, _ctx: &NamepathContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#namepath}.
    /// @param ctx the parse tree
    fn exit_namepath(&mut self, _ctx: &NamepathContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#string}.
    /// @param ctx the parse tree
    fn enter_string(&mut self, _ctx: &StringContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#string}.
    /// @param ctx the parse tree
    fn exit_string(&mut self, _ctx: &StringContext<'input>) {}
    /// Enter a parse tree produced by {@link YggdrasilAntlrParser#identifier}.
    /// @param ctx the parse tree
    fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}
    /// Exit a parse tree produced by {@link YggdrasilAntlrParser#identifier}.
    /// @param ctx the parse tree
    fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : YggdrasilAntlrListener<'input> }
