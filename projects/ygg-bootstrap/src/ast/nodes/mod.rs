use super::*;

mod nodes_custom;

#[derive(Clone)]
pub struct Program {
    pub statement: Vec<Statement>,
    pub position: OffsetRange,
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
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct FragmentStatement {
    pub id: Symbol,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Symbol,
    pub eq: String,
    pub rhs: Expression,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct IgnoreStatement {
    pub rules: Vec<Symbol>,
    pub position: OffsetRange,
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
    pub lhs: Expression,
    pub rhs: Expression,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct ChoiceExpression {
    pub lhs: ChoiceTag,
    pub op: String,
    pub rhs: ChoiceTag,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct CommentDocument {
    pub doc: String,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct ChoiceTag {
    pub expr: Expression,
    pub tag: Option<Symbol>,
    pub mode: Option<String>,
    pub ty: Option<Symbol>,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct MarkExpression {
    pub lhs: Symbol,
    pub ty: Option<SymbolPath>,
    pub rhs: Expression,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct UnarySuffix {
    pub suffix: String,
    pub base: Expression,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct UnaryPrefix {
    pub prefix: String,
    pub base: Expression,
    pub position: OffsetRange,
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
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub data: String,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct Integer {
    pub data: isize,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct StringLiteral {
    pub data: String,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct Eos {
    pub data: bool,
    pub position: OffsetRange,
}
