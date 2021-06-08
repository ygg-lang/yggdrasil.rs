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
    FragmentStatement(Box<FragmentStatement>),
    AssignStatement(Box<AssignStatement>),
    Ignore(Box<IgnoreStatement>),
    CommentDocument(Box<CommentDocument>),
}

#[derive(Clone, Debug)]
pub struct GrammarStatement {
    pub id: Identifier,
    pub ext: Vec<StringLiteral>,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct FragmentStatement {
    pub id: Identifier,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Identifier,
    pub eq: String,
    pub rhs: Expression,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct IgnoreStatement {
    pub rules: Vec<Identifier>,
    pub position: OffsetRange,
}

#[derive(Clone)]
pub enum Expression {
    Data(Box<Data>),
    // Priority(Box<Expression>),
    UnarySuffix(Box<UnarySuffix>),
    UnaryPrefix(Box<UnaryPrefix>),
    ConcatExpression(Box<ConcatExpression>),
    ChoiceExpression(Box<ChoiceExpression>),
    FieldExpression(Box<FieldExpression>),
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
    pub tag: Option<Identifier>,
    pub mode: Option<String>,
    pub ty: Option<Identifier>,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct FieldExpression {
    pub lhs: Identifier,
    pub op: String,
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
    Identifier(Box<Identifier>),
    Integer(Box<Unsigned>),
    String(Box<StringLiteral>),
    Macro,
    Regex,
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub data: String,
    pub position: OffsetRange,
}

#[derive(Clone, Debug)]
pub struct Unsigned {
    pub data: usize,
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
