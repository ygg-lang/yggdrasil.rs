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
    GrammarTerm,
    GrammarPair,
    GrammarValue,
    GrammarDict,
    GrammarList,
    GrammarListTerms,
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
    StringRaw,
    StringRawText,
    StringNormal,
    StringItem,
    EscapedUnicode,
    EscapedCharacter,
    HEX,
    TextAny,
    RegexEmbed,
    RegexItem,
    RegexCharacter,
    RegexRange,
    RegexNegative,
    Category,
    NamepathFree,
    Namepath,
    Identifier,
    Boolean,
    Integer,
    RangeExact,
    Range,
    ModifierCall,
    OP_CATEGORY,
    KW_EXTERNAL,
    KW_GRAMMAR,
    KW_IMPORT,
    KW_CLASS,
    KW_UNION,
    KW_GROUP,
    KW_CLIMB,
    KW_MACRO,
    WhiteSpace,
    Comment,
    /// Label for unnamed text literal
    HiddenText,
}

impl YggdrasilRule for BootstrapRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::HiddenText | Self::WhiteSpace | Self::Comment)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Root => "",
            Self::Statement => "",
            Self::GrammarStatement => "",
            Self::GrammarTerm => "",
            Self::GrammarPair => "",
            Self::GrammarValue => "",
            Self::GrammarDict => "",
            Self::GrammarList => "",
            Self::GrammarListTerms => "",
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
            Self::StringRaw => "",
            Self::StringRawText => "",
            Self::StringNormal => "",
            Self::StringItem => "",
            Self::EscapedUnicode => "",
            Self::EscapedCharacter => "",
            Self::HEX => "",
            Self::TextAny => "",
            Self::RegexEmbed => "",
            Self::RegexItem => "",
            Self::RegexCharacter => "",
            Self::RegexRange => "",
            Self::RegexNegative => "",
            Self::Category => "",
            Self::NamepathFree => "",
            Self::Namepath => "",
            Self::Identifier => "",
            Self::Boolean => "",
            Self::Integer => "",
            Self::RangeExact => "",
            Self::Range => "",
            Self::ModifierCall => "",
            Self::OP_CATEGORY => "",
            Self::KW_EXTERNAL => "",
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
pub struct RootNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementNode<'i> {
    ClassStatement(ClassStatementNode<'i>),
    GrammarStatement(GrammarStatementNode<'i>),
    GroupStatement(GroupStatementNode<'i>),
    UnionStatement(UnionStatementNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarStatementNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrammarTermNode<'i> {
    GrammarPair(GrammarPairNode<'i>),
    GrammarTerm1,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarPairNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrammarValueNode<'i> {
    GrammarDict(GrammarDictNode<'i>),
    GrammarList(GrammarListNode<'i>),
    Namepath(NamepathNode<'i>),
    StringNormal(StringNormalNode<'i>),
    StringRaw(StringRawNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarDictNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarListNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarListTermsNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassStatementNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassBlockNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpRemarkNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionStatementNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionBlockNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionBranchNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BranchTagNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RightAssociativityNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupStatementNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupBlockNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupPairNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecoratorCallNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecoratorNameNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCallNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionNameNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallBodyNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionHardNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionSoftNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionTagNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TermNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrefixNode<'i> {
    Negative,
    Positive,
    Remark,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SuffixNode<'i> {
    Many,
    Many1,
    Optional,
    Range(RangeNode<'i>),
    RangeExact(RangeExactNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtomicNode<'i> {
    Boolean(BooleanNode<'i>),
    Category(CategoryNode<'i>),
    EscapedUnicode(EscapedUnicodeNode<'i>),
    FunctionCall(FunctionCallNode<'i>),
    GroupExpression(GroupExpressionNode<'i>),
    Identifier(IdentifierNode<'i>),
    Integer(IntegerNode<'i>),
    RegexEmbed(RegexEmbedNode<'i>),
    RegexRange(RegexRangeNode<'i>),
    StringNormal(StringNormalNode<'i>),
    StringRaw(StringRawNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupExpressionNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringRawNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringRawTextNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringNormalNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringItemNode<'i> {
    EscapedCharacter(EscapedCharacterNode<'i>),
    EscapedUnicode(EscapedUnicodeNode<'i>),
    TextAny(TextAnyNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EscapedUnicodeNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EscapedCharacterNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextAnyNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexEmbedNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RegexItemNode<'i> {
    EscapedCharacter(EscapedCharacterNode<'i>),
    RegexCharacter(RegexCharacterNode<'i>),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexCharacterNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexRangeNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexNegativeNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CategoryNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathFreeNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamepathNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BooleanNode<'i> {
    False,
    True,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeExactNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RangeNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifierCallNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpCategoryNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KwExternalNode<'i> {
    External,
    Inspector,
    Parser,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwGrammarNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwImportNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwClassNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwUnionNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwGroupNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwClimbNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KwMacroNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentNode<'i> {
    pair: TokenPair<'i, BootstrapRule>,
}
