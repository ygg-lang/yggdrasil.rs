use super::*;

#[derive(Clone, Debug)]
pub struct Program {
    pub statement: Vec<Statement>,
    pub range: Range,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Expression {
    ErrorNode,
    Priority(Box<Expression>),
    Identifier(Box<Identifier>),
    Integer(Box<Unsigned>),
    String(Box<StringLiteral>),
    UnarySuffix(Box<UnarySuffix>),
    UnaryPrefix(Box<UnaryPrefix>),
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

impl Default for Expression {
    fn default() -> Self {
        Self::ErrorNode
    }
}
