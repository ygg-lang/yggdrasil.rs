use super::*;

#[derive(Clone)]
pub struct Program {
    pub statement: Vec<Statement>,
    pub range: Range,
}

#[derive(Clone)]
pub enum Statement {
    GrammarStatement(Box<GrammarStatement>),
    FragmentStatement(Box<FragmentStatement>),
    AssignStatement(Box<AssignStatement>),
    IgnoreStatement(Box<IgnoreStatement>),
    EmptyStatement(Box<Eos>),
}

#[derive(Clone, Debug)]
pub struct GrammarStatement {
    pub id: Identifier,
    pub ext: Vec<String>,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct FragmentStatement {
    pub id: Identifier,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Identifier,
    pub eq: String,
    pub rhs: Expression,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct IgnoreStatement {
    pub rules: Vec<String>,
    pub range: Range,
}

#[derive(Clone)]
pub enum Expression {
    //ErrorNode,
    Data(Box<Data>),
    //Priority(Box<Expression>),
    UnarySuffix(Box<UnarySuffix>),
    UnaryPrefix(Box<UnaryPrefix>),
    ConcatExpression(Box<ConcatExpression>),
    ChoiceExpression(Box<ChoiceExpression>),
    FieldExpression(Box<FieldExpression>),
}

#[derive(Clone, Debug)]
pub struct ConcatExpression {
    pub base: Expression,
    pub op: Vec<String>,
    pub expr: Vec<Expression>,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct ChoiceExpression {
    pub base: TaggedExpression,
    pub op: Vec<String>,
    pub expr: Vec<TaggedExpression>,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct TaggedExpression {
    pub expr: Expression,
    pub tag: String,
    pub tag_mode: String,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct FieldExpression {
    pub lhs: Identifier,
    pub op: String,
    pub rhs: Expression,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct UnarySuffix {
    pub suffix: String,
    pub base: Expression,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct UnaryPrefix {
    pub prefix: String,
    pub base: Expression,
    pub range: Range,
}

#[derive(Clone)]
pub enum Data {
    Identifier(Box<Identifier>),
    Integer(Box<Unsigned>),
    String(Box<StringLiteral>),
    Regex,
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub data: String,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct Unsigned {
    pub data: usize,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct StringLiteral {
    pub data: String,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub struct Eos {
    pub data: bool,
    pub range: Range,
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            data: "".to_string(),
            range: Range { start_byte: 0, end_byte: 0, start_point: Default::default(), end_point: Default::default() },
        }
    }
}
