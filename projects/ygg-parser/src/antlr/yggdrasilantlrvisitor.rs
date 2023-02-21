#![allow(nonstandard_style)]
// Generated from YggdrasilAntlr.g4 by ANTLR 4.8
use super::yggdrasilantlrparser::*;
use antlr_rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/// This interface defines a complete generic visitor for a parse tree produced
/// by {@link YggdrasilAntlrParser}.
pub trait YggdrasilAntlrVisitor<'input>: ParseTreeVisitor<'input, YggdrasilAntlrParserContextType> {
    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#program}.
    /// @param ctx the parse tree
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#import_statement}.
    /// @param ctx the parse tree
    fn visit_import_statement(&mut self, ctx: &Import_statementContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#import_block}.
    /// @param ctx the parse tree
    fn visit_import_block(&mut self, ctx: &Import_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_grammar}.
    /// @param ctx the parse tree
    fn visit_define_grammar(&mut self, ctx: &Define_grammarContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#grammar_block}.
    /// @param ctx the parse tree
    fn visit_grammar_block(&mut self, ctx: &Grammar_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_class}.
    /// @param ctx the parse tree
    fn visit_define_class(&mut self, ctx: &Define_classContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#class_block}.
    /// @param ctx the parse tree
    fn visit_class_block(&mut self, ctx: &Class_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CSuffix}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CSuffix(&mut self, ctx: &CSuffixContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CCall}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CCall(&mut self, ctx: &CCallContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CETag}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CETag(&mut self, ctx: &CETagContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CUntag}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CUntag(&mut self, ctx: &CUntagContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CSoft}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CSoft(&mut self, ctx: &CSoftContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CHard}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CHard(&mut self, ctx: &CHardContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CPattern}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CPattern(&mut self, ctx: &CPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CGroup}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CGroup(&mut self, ctx: &CGroupContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Atom}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_Atom(&mut self, ctx: &AtomContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CNot}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CNot(&mut self, ctx: &CNotContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_union}.
    /// @param ctx the parse tree
    fn visit_define_union(&mut self, ctx: &Define_unionContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#union_block}.
    /// @param ctx the parse tree
    fn visit_union_block(&mut self, ctx: &Union_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#union_term}.
    /// @param ctx the parse tree
    fn visit_union_term(&mut self, ctx: &Union_termContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UHard}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UHard(&mut self, ctx: &UHardContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UUntag}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UUntag(&mut self, ctx: &UUntagContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code USuffix}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_USuffix(&mut self, ctx: &USuffixContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UGroup}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UGroup(&mut self, ctx: &UGroupContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UETag}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UETag(&mut self, ctx: &UETagContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Utom}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_Utom(&mut self, ctx: &UtomContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UNot}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UNot(&mut self, ctx: &UNotContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code USoft}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_USoft(&mut self, ctx: &USoftContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UCall}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UCall(&mut self, ctx: &UCallContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_climb}.
    /// @param ctx the parse tree
    fn visit_define_climb(&mut self, ctx: &Define_climbContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#tag_branch}.
    /// @param ctx the parse tree
    fn visit_tag_branch(&mut self, ctx: &Tag_branchContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_token}.
    /// @param ctx the parse tree
    fn visit_define_token(&mut self, ctx: &Define_tokenContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#token_block}.
    /// @param ctx the parse tree
    fn visit_token_block(&mut self, ctx: &Token_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#token_pair}.
    /// @param ctx the parse tree
    fn visit_token_pair(&mut self, ctx: &Token_pairContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code TOr}
    /// labeled alternative in {@link YggdrasilAntlrParser#token_expression}.
    /// @param ctx the parse tree
    fn visit_TOr(&mut self, ctx: &TOrContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code TAtom}
    /// labeled alternative in {@link YggdrasilAntlrParser#token_expression}.
    /// @param ctx the parse tree
    fn visit_TAtom(&mut self, ctx: &TAtomContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#macro_call}.
    /// @param ctx the parse tree
    fn visit_macro_call(&mut self, ctx: &Macro_callContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#tuple_call}.
    /// @param ctx the parse tree
    fn visit_tuple_call(&mut self, ctx: &Tuple_callContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#tuple_block}.
    /// @param ctx the parse tree
    fn visit_tuple_block(&mut self, ctx: &Tuple_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Optional}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_Optional(&mut self, ctx: &OptionalContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code MaybeGreedy}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_MaybeGreedy(&mut self, ctx: &MaybeGreedyContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Maybe}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_Maybe(&mut self, ctx: &MaybeContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code ManyGreedy}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_ManyGreedy(&mut self, ctx: &ManyGreedyContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Many}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_Many(&mut self, ctx: &ManyContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code ATuple}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_ATuple(&mut self, ctx: &ATupleContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code AString}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_AString(&mut self, ctx: &AStringContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code AId}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_AId(&mut self, ctx: &AIdContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code ARe}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_ARe(&mut self, ctx: &AReContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code AInt}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_AInt(&mut self, ctx: &AIntContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code ASpecial}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_ASpecial(&mut self, ctx: &ASpecialContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code AChar}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_AChar(&mut self, ctx: &ACharContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#regex}.
    /// @param ctx the parse tree
    fn visit_regex(&mut self, ctx: &RegexContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#namepath}.
    /// @param ctx the parse tree
    fn visit_namepath(&mut self, ctx: &NamepathContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#string}.
    /// @param ctx the parse tree
    fn visit_string(&mut self, ctx: &StringContext<'input>) {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#identifier}.
    /// @param ctx the parse tree
    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait YggdrasilAntlrVisitorCompat<'input>: ParseTreeVisitorCompat<'input, Node = YggdrasilAntlrParserContextType> {
    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#program}.
    /// @param ctx the parse tree
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#import_statement}.
    /// @param ctx the parse tree
    fn visit_import_statement(&mut self, ctx: &Import_statementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#import_block}.
    /// @param ctx the parse tree
    fn visit_import_block(&mut self, ctx: &Import_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_grammar}.
    /// @param ctx the parse tree
    fn visit_define_grammar(&mut self, ctx: &Define_grammarContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#grammar_block}.
    /// @param ctx the parse tree
    fn visit_grammar_block(&mut self, ctx: &Grammar_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_class}.
    /// @param ctx the parse tree
    fn visit_define_class(&mut self, ctx: &Define_classContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#class_block}.
    /// @param ctx the parse tree
    fn visit_class_block(&mut self, ctx: &Class_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CSuffix}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CSuffix(&mut self, ctx: &CSuffixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CCall}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CCall(&mut self, ctx: &CCallContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CETag}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CETag(&mut self, ctx: &CETagContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CUntag}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CUntag(&mut self, ctx: &CUntagContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CSoft}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CSoft(&mut self, ctx: &CSoftContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CHard}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CHard(&mut self, ctx: &CHardContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CPattern}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CPattern(&mut self, ctx: &CPatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CGroup}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CGroup(&mut self, ctx: &CGroupContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Atom}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_Atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code CNot}
    /// labeled alternative in {@link YggdrasilAntlrParser#class_expression}.
    /// @param ctx the parse tree
    fn visit_CNot(&mut self, ctx: &CNotContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_union}.
    /// @param ctx the parse tree
    fn visit_define_union(&mut self, ctx: &Define_unionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#union_block}.
    /// @param ctx the parse tree
    fn visit_union_block(&mut self, ctx: &Union_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#union_term}.
    /// @param ctx the parse tree
    fn visit_union_term(&mut self, ctx: &Union_termContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UHard}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UHard(&mut self, ctx: &UHardContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UUntag}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UUntag(&mut self, ctx: &UUntagContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code USuffix}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_USuffix(&mut self, ctx: &USuffixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UGroup}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UGroup(&mut self, ctx: &UGroupContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UETag}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UETag(&mut self, ctx: &UETagContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Utom}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_Utom(&mut self, ctx: &UtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UNot}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UNot(&mut self, ctx: &UNotContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code USoft}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_USoft(&mut self, ctx: &USoftContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code UCall}
    /// labeled alternative in {@link YggdrasilAntlrParser#union_expression}.
    /// @param ctx the parse tree
    fn visit_UCall(&mut self, ctx: &UCallContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_climb}.
    /// @param ctx the parse tree
    fn visit_define_climb(&mut self, ctx: &Define_climbContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#tag_branch}.
    /// @param ctx the parse tree
    fn visit_tag_branch(&mut self, ctx: &Tag_branchContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#define_token}.
    /// @param ctx the parse tree
    fn visit_define_token(&mut self, ctx: &Define_tokenContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#token_block}.
    /// @param ctx the parse tree
    fn visit_token_block(&mut self, ctx: &Token_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#token_pair}.
    /// @param ctx the parse tree
    fn visit_token_pair(&mut self, ctx: &Token_pairContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code TOr}
    /// labeled alternative in {@link YggdrasilAntlrParser#token_expression}.
    /// @param ctx the parse tree
    fn visit_TOr(&mut self, ctx: &TOrContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code TAtom}
    /// labeled alternative in {@link YggdrasilAntlrParser#token_expression}.
    /// @param ctx the parse tree
    fn visit_TAtom(&mut self, ctx: &TAtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#macro_call}.
    /// @param ctx the parse tree
    fn visit_macro_call(&mut self, ctx: &Macro_callContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#tuple_call}.
    /// @param ctx the parse tree
    fn visit_tuple_call(&mut self, ctx: &Tuple_callContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#tuple_block}.
    /// @param ctx the parse tree
    fn visit_tuple_block(&mut self, ctx: &Tuple_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Optional}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_Optional(&mut self, ctx: &OptionalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code MaybeGreedy}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_MaybeGreedy(&mut self, ctx: &MaybeGreedyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Maybe}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_Maybe(&mut self, ctx: &MaybeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code ManyGreedy}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_ManyGreedy(&mut self, ctx: &ManyGreedyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code Many}
    /// labeled alternative in {@link YggdrasilAntlrParser#suffix}.
    /// @param ctx the parse tree
    fn visit_Many(&mut self, ctx: &ManyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code ATuple}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_ATuple(&mut self, ctx: &ATupleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code AString}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_AString(&mut self, ctx: &AStringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code AId}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_AId(&mut self, ctx: &AIdContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code ARe}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_ARe(&mut self, ctx: &AReContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code AInt}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_AInt(&mut self, ctx: &AIntContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code ASpecial}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_ASpecial(&mut self, ctx: &ASpecialContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by the {@code AChar}
    /// labeled alternative in {@link YggdrasilAntlrParser#atomic}.
    /// @param ctx the parse tree
    fn visit_AChar(&mut self, ctx: &ACharContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#regex}.
    /// @param ctx the parse tree
    fn visit_regex(&mut self, ctx: &RegexContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#namepath}.
    /// @param ctx the parse tree
    fn visit_namepath(&mut self, ctx: &NamepathContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#string}.
    /// @param ctx the parse tree
    fn visit_string(&mut self, ctx: &StringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /// Visit a parse tree produced by {@link YggdrasilAntlrParser#identifier}.
    /// @param ctx the parse tree
    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> YggdrasilAntlrVisitor<'input> for T
where
    T: YggdrasilAntlrVisitorCompat<'input>,
{
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_statement(&mut self, ctx: &Import_statementContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_import_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_block(&mut self, ctx: &Import_blockContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_import_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_grammar(&mut self, ctx: &Define_grammarContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_define_grammar(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_grammar_block(&mut self, ctx: &Grammar_blockContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_grammar_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_class(&mut self, ctx: &Define_classContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_define_class(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_class_block(&mut self, ctx: &Class_blockContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_class_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CSuffix(&mut self, ctx: &CSuffixContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CSuffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CCall(&mut self, ctx: &CCallContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CETag(&mut self, ctx: &CETagContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CETag(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CUntag(&mut self, ctx: &CUntagContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CUntag(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CSoft(&mut self, ctx: &CSoftContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CSoft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CHard(&mut self, ctx: &CHardContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CHard(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CPattern(&mut self, ctx: &CPatternContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CGroup(&mut self, ctx: &CGroupContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_Atom(&mut self, ctx: &AtomContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_Atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_CNot(&mut self, ctx: &CNotContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_CNot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_union(&mut self, ctx: &Define_unionContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_define_union(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_union_block(&mut self, ctx: &Union_blockContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_union_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_union_term(&mut self, ctx: &Union_termContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_union_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_UHard(&mut self, ctx: &UHardContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_UHard(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_UUntag(&mut self, ctx: &UUntagContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_UUntag(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_USuffix(&mut self, ctx: &USuffixContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_USuffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_UGroup(&mut self, ctx: &UGroupContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_UGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_UETag(&mut self, ctx: &UETagContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_UETag(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_Utom(&mut self, ctx: &UtomContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_Utom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_UNot(&mut self, ctx: &UNotContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_UNot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_USoft(&mut self, ctx: &USoftContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_USoft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_UCall(&mut self, ctx: &UCallContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_UCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_climb(&mut self, ctx: &Define_climbContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_define_climb(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_tag_branch(&mut self, ctx: &Tag_branchContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_tag_branch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_define_token(&mut self, ctx: &Define_tokenContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_define_token(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_token_block(&mut self, ctx: &Token_blockContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_token_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_token_pair(&mut self, ctx: &Token_pairContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_token_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TOr(&mut self, ctx: &TOrContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_TOr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_TAtom(&mut self, ctx: &TAtomContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_TAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_macro_call(&mut self, ctx: &Macro_callContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_macro_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_tuple_call(&mut self, ctx: &Tuple_callContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_tuple_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_tuple_block(&mut self, ctx: &Tuple_blockContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_tuple_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_Optional(&mut self, ctx: &OptionalContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_Optional(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_MaybeGreedy(&mut self, ctx: &MaybeGreedyContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_MaybeGreedy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_Maybe(&mut self, ctx: &MaybeContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_Maybe(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ManyGreedy(&mut self, ctx: &ManyGreedyContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_ManyGreedy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_Many(&mut self, ctx: &ManyContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_Many(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ATuple(&mut self, ctx: &ATupleContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_ATuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_AString(&mut self, ctx: &AStringContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_AString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_AId(&mut self, ctx: &AIdContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_AId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ARe(&mut self, ctx: &AReContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_ARe(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_AInt(&mut self, ctx: &AIntContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_AInt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ASpecial(&mut self, ctx: &ASpecialContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_ASpecial(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_AChar(&mut self, ctx: &ACharContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_AChar(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_regex(&mut self, ctx: &RegexContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_regex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_namepath(&mut self, ctx: &NamepathContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_namepath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_string(&mut self, ctx: &StringContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) {
        let result = <Self as YggdrasilAntlrVisitorCompat>::visit_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}
