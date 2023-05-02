#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

use core::str::FromStr;
use std::{borrow::Cow, ops::Range, sync::OnceLock};
use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, BootstrapRule>>;
type Output<'i> = Result<Box<State<'i, BootstrapRule>>, Box<State<'i, BootstrapRule>>>;

#[doc = include_str!("railway.min.svg")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BootstrapParser {}

impl YggdrasilParser for BootstrapParser {
    type Rule = BootstrapRule;
    fn parse_cst(input: &str, rule: Self::Rule) -> OutputResult<BootstrapRule> {
        self::parse_cst::parse_cst(input, rule)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BootstrapRule {
    Root,
    Statement,
    GrammarStatement,
    GrammarBlock,
    ClassStatement,
    ClassBlock,
    UnionStatement,
    UnionBlock,
    UnionBranch,
    BranchTag,
    RightAssociativity,
    GroupStatement,
    GroupBlock,
    GroupPair,
    Annotation,
    AnnotationCall,
    AnnotationName,
    FunctionCall,
    FunctionName,
    CallBody,
    Expression,
    Term,
    Infix,
    Prefix,
    Suffix,
    Atomic,
    String,
    RegexEmbed,
    RegexRange,
    RegexNegative,
    NamepathFree,
    Namepath,
    Identifier,
    Boolean,
    ModifierCall,
    KW_GRAMMAR,
    KW_IMPORT,
    KW_CLASS,
    KW_UNION,
    KW_GROUP,
    KW_CLIMB,
    KW_MACRO,
    WhiteSpace,
    Comment,
    /// Label for text literal
    IgnoreText,
    /// Label for regex literal
    IgnoreRegex,
}

impl YggdrasilRule for BootstrapRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::IgnoreText | Self::IgnoreRegex | Self::WhiteSpace | Self::Comment)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Root => "",
            Self::Statement => "",
            Self::GrammarStatement => "",
            Self::GrammarBlock => "",
            Self::ClassStatement => "",
            Self::ClassBlock => "",
            Self::UnionStatement => "",
            Self::UnionBlock => "",
            Self::UnionBranch => "",
            Self::BranchTag => "",
            Self::RightAssociativity => "",
            Self::GroupStatement => "",
            Self::GroupBlock => "",
            Self::GroupPair => "",
            Self::Annotation => "",
            Self::AnnotationCall => "",
            Self::AnnotationName => "",
            Self::FunctionCall => "",
            Self::FunctionName => "",
            Self::CallBody => "",
            Self::Expression => "",
            Self::Term => "",
            Self::Infix => "",
            Self::Prefix => "",
            Self::Suffix => "",
            Self::Atomic => "",
            Self::String => "",
            Self::RegexEmbed => "",
            Self::RegexRange => "",
            Self::RegexNegative => "",
            Self::NamepathFree => "",
            Self::Namepath => "",
            Self::Identifier => "",
            Self::Boolean => "",
            Self::ModifierCall => "",
            Self::KW_GRAMMAR => "",
            Self::KW_IMPORT => "",
            Self::KW_CLASS => "",
            Self::KW_UNION => "",
            Self::KW_GROUP => "",
            Self::KW_CLIMB => "",
            Self::KW_MACRO => "",
            Self::WhiteSpace => "",
            Self::Comment => "",
            _ => "",
        }
    }
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RootNode {
    pub statement: StatementNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementNode {
    ClassStatement(ClassStatementNode),
    GrammarStatement(GrammarStatementNode),
    GroupStatement(GroupStatementNode),
    UnionStatement(UnionStatementNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarStatementNode {
    pub grammar_block: GrammarBlockNode,
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarBlockNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassStatementNode {
    pub annotation: AnnotationNode,
    pub class_block: ClassBlockNode,
    pub cast: IdentifierNode,
    pub name: IdentifierNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassBlockNode {
    pub expression: Vec<ExpressionNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionStatementNode {
    pub annotation: AnnotationNode,
    pub union_block: UnionBlockNode,
    pub name: IdentifierNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionBlockNode {
    pub union_branch: UnionBranchNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionBranchNode {
    pub branch_tag: Option<BranchTagNode>,
    pub expression: ExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BranchTagNode {
    pub identifier: IdentifierNode,
    pub right_associativity: Option<RightAssociativityNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RightAssociativityNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupStatementNode {
    pub annotation: AnnotationNode,
    pub group_block: GroupBlockNode,
    pub identifier: Option<IdentifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupBlockNode {
    pub group_pair: Vec<GroupPairNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupPairNode {
    pub atomic: AtomicNode,
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationNode {
    pub annotation_call: AnnotationCallNode,
    pub modifier_call: ModifierCallNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationCallNode {
    pub annotation_name: AnnotationNameNode,
    pub call_body: CallBodyNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationNameNode {
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCallNode {
    pub call_body: CallBodyNode,
    pub function_name: FunctionNameNode,
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionNameNode {
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallBodyNode {
    pub expression: Vec<ExpressionNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode {
    pub infix: InfixNode,
    pub term: Vec<TermNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TermNode {
    pub atomic: AtomicNode,
    pub prefix: PrefixNode,
    pub suffix: SuffixNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InfixNode {
    Infix0,
    Infix1,
    Infix2,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrefixNode {
    Prefix0,
    Prefix1,
    Prefix2,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SuffixNode {
    Suffix0,
    Suffix1,
    Suffix2,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtomicNode {
    Atomic0(ExpressionNode),
    Boolean(BooleanNode),
    FunctionCall(FunctionCallNode),
    Identifier(IdentifierNode),
    RegexEmbed(RegexEmbedNode),
    RegexRange(RegexRangeNode),
    String(StringNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringNode {
    String0,
    String1,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexEmbedNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexRangeNode {
    pub regex_negative: Option<RegexNegativeNode>,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexNegativeNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathFreeNode {
    pub identifier: Vec<IdentifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathNode {
    pub identifier: Vec<IdentifierNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BooleanNode {
    Boolean0,
    Boolean1,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifierCallNode {
    pub identifier: IdentifierNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwGrammarNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwImportNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwClassNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwUnionNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwGroupNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwClimbNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwMacroNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommentNode {
    Comment0,
    Comment1,
}
