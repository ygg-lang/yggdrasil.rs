use yggdrasil_shared::records::OffsetRange;
use super::*;

mod nodes_custom;
// mod nodes_resolver;

// pub use self::nodes_resolver::*;

#[derive(Clone)]
pub struct Program {
    pub statement: Vec<Statement>,
    pub range: OffsetRange,
}

#[derive(Clone)]
pub enum Statement {
    GrammarStatement(Box<GrammarStatement>),
    Fragment(Box<FragmentStatement>),
    Assign(Box<AssignStatement>),
    Ignore(Box<IgnoreStatement>),
    CommentDocument(Box<CommentDocument>),
}

#[derive(Clone, Debug)]
pub struct GrammarStatement {
    pub id: Symbol,
    pub ext: Vec<StringLiteral>,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct FragmentStatement {
    pub id: Symbol,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Symbol,
    pub eq: String,
    pub rhs: Expression,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct IgnoreStatement {
    pub rules: Vec<Symbol>,
    pub range: OffsetRange,
}

#[derive(Clone)]
pub enum Expression {
    Data(Box<Data>),
    // Priority(Box<Expression>),
    UnarySuffix(Box<UnarySuffix>),
    UnaryPrefix(Box<UnaryPrefix>),
    Concat(Box<ConcatExpression>),
    Choice(Box<ChoiceExpression>),
    Mark(Box<MarkExpression>),
}

#[derive(Clone, Debug)]
pub struct ConcatExpression {
    pub base: Expression,
    pub rest: Vec<Expression>,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct ChoiceExpression {
    pub lhs: ChoiceTag,
    pub rhs: ChoiceTag,
    pub range: OffsetRange,
}


#[derive(Clone, Debug)]
pub struct ChoiceTag {
    pub expr: Expression,
    pub tag: Option<Symbol>,
    pub mode: Option<String>,
    pub ty: Option<Symbol>,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct MarkExpression {
    pub lhs: Symbol,
    pub ty: Option<SymbolPath>,
    pub rhs: Expression,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct UnarySuffix {
    pub suffix: String,
    pub base: Expression,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct UnaryPrefix {
    pub prefix: String,
    pub base: Expression,
    pub range: OffsetRange,
}

#[derive(Clone)]
pub enum Data {
    SymbolPath(Box<SymbolPath>),
    Integer(Box<Integer>),
    String(Box<StringLiteral>),
    Macro,
    Regex,
}

#[derive(Clone, Debug)]
pub struct SymbolPath {
    pub symbol: Vec<Symbol>,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub data: String,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct Integer {
    pub data: isize,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct StringLiteral {
    pub data: String,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct Eos {
    pub data: bool,
    pub range: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct CommentDocument {
    pub doc: String,
    pub range: OffsetRange,
}
