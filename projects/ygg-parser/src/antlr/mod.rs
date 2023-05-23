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
    OP_REMARK,
    UnionStatement,
    UnionBlock,
    UnionBranch,
    BranchTag,
    RightAssociativity,
    GroupStatement,
    GroupBlock,
    GroupPair,
    DecoratorCall,
    DecoratorName,
    FunctionCall,
    FunctionName,
    CallBody,
    Expression,
    ExpressionHard,
    ExpressionSoft,
    ExpressionTag,
    Term,
    Prefix,
    Suffix,
    Atomic,
    GroupExpression,
    String,
    StringRaw,
    /// Label for text literal
    IgnoreText,
    /// Label for regex literal
    IgnoreRegex,
}

impl YggdrasilRule for BootstrapRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::IgnoreText | Self::IgnoreRegex)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Root => "",
            Self::Statement => "",
            Self::GrammarStatement => "",
            Self::GrammarBlock => "",
            Self::ClassStatement => "",
            Self::ClassBlock => "",
            Self::OP_REMARK => "",
            Self::UnionStatement => "",
            Self::UnionBlock => "",
            Self::UnionBranch => "",
            Self::BranchTag => "",
            Self::RightAssociativity => "",
            Self::GroupStatement => "",
            Self::GroupBlock => "",
            Self::GroupPair => "",
            Self::DecoratorCall => "",
            Self::DecoratorName => "",
            Self::FunctionCall => "",
            Self::FunctionName => "",
            Self::CallBody => "",
            Self::Expression => "",
            Self::ExpressionHard => "",
            Self::ExpressionSoft => "",
            Self::ExpressionTag => "",
            Self::Term => "",
            Self::Prefix => "",
            Self::Suffix => "",
            Self::Atomic => "",
            Self::GroupExpression => "",
            Self::String => "",
            Self::StringRaw => "",
            _ => "",
        }
    }
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RootNode {
    pub statement: Vec<StatementNode>,
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
    // Missing rule Identifier
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
    pub class_block: ClassBlockNode,
    pub decorator_call: Vec<DecoratorCallNode>,
    // Missing rule ModifierCall
    pub op_remark: Option<OpRemarkNode>,
    // Missing rule Identifier
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassBlockNode {
    pub expression: ExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpRemarkNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionStatementNode {
    pub decorator_call: Vec<DecoratorCallNode>,
    // Missing rule ModifierCall
    pub op_remark: Option<OpRemarkNode>,
    pub union_block: UnionBlockNode,
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionBlockNode {
    pub union_branch: Vec<UnionBranchNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionBranchNode {
    pub branch_tag: Option<BranchTagNode>,
    pub expression_hard: ExpressionHardNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BranchTagNode {
    // Missing rule Identifier
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
    pub decorator_call: Vec<DecoratorCallNode>,
    pub group_block: GroupBlockNode,
    // Missing rule Identifier
    // Missing rule ModifierCall
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
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecoratorCallNode {
    pub call_body: CallBodyNode,
    pub decorator_name: DecoratorNameNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecoratorNameNode {
    // Missing rule Identifier
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCallNode {
    pub call_body: CallBodyNode,
    pub function_name: FunctionNameNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionNameNode {
    // Missing rule Identifier
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
    pub expression_hard: Vec<ExpressionHardNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionHardNode {
    pub expression_soft: Vec<ExpressionSoftNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionSoftNode {
    pub expression_tag: Vec<ExpressionTagNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionTagNode {
    // Missing rule Identifier
    pub term: TermNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TermNode {
    pub atomic: AtomicNode,
    pub prefix: Vec<PrefixNode>,
    pub suffix: Vec<SuffixNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrefixNode {
    Negative,
    Positive,
    Remark,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SuffixNode {
    Many,
    Many1,
    Optional,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtomicNode {
    Boolean(),
    FunctionCall(FunctionCallNode),
    GroupExpression(GroupExpressionNode),
    Identifier(),
    RegexEmbed(),
    RegexRange(),
    String(StringNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupExpressionNode {
    pub expression: ExpressionNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringNode {
    Escaped(),
    Raw(StringRawNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringRawNode {
    pub text: String,
    pub span: Range<u32>,
}
