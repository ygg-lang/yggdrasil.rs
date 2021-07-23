mod nodes_custom;
mod custom_debug;

// pub use self::nodes_resolver::*;

#[derive(Clone)]
pub struct Program {
    pub statement: Vec<Statement>,
    pub range: (usize, usize),
}

#[derive(Clone)]
pub enum Statement {
    Grammar(Box<GrammarStatement>),
    Import(Box<ImportStatement>),
    Assign(Box<AssignStatement>),
    Ignore(Box<IgnoreStatement>),
    Fragment(Box<FragmentStatement>),
    CommentDocument(Box<CommentDocument>),
}

#[derive(Clone, Debug)]
pub struct GrammarStatement {
    pub id: Symbol,
    pub ext: Vec<StringLiteral>,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct FragmentStatement {
    pub id: Symbol,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Symbol,
    pub ty: Option<Symbol>,
    pub eq: String,
    pub rhs: Expression,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct IgnoreStatement {
    pub rules: Vec<Symbol>,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct ImportStatement {
    pub path: StringLiteral,
    pub symbol_alias: Vec<SymbolAlias>,
    pub range: (usize, usize),
}

#[derive(Clone)]
pub struct SymbolAlias {
    pub from: Symbol,
    pub into: Option<Symbol>,
    pub range: (usize, usize),
}

#[derive(Clone)]
pub enum Expression {
    Data(Data),
    /// lhs ~ rhs
    Concat {
        is_soft: bool,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// lhs | rhs
    Choice {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// lhs <- rhs
    MarkNode {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// ^rhs
    MarkNodeShort(Box<Expression>),
    /// lhs: rhs
    MarkType {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// !rhs
    MustNot(Box<Expression>),
    MustOne(Box<Expression>),
    Maybe(Box<Expression>),
    Many(Box<Expression>),
    ManyNonNull(Box<Expression>),
}

#[derive(Debug)]
pub enum Term {
    Prefix(char),
    Suffix(char),
    Infix(char),
    Atom(Expression),
}

#[derive(Clone, Debug)]
pub struct Prefix {
    pub data: char,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct Suffix {
    pub data: char,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct ConcatExpression {
    pub is_soft: bool,
    pub lhs: Expression,
    pub rhs: Expression,
}

#[derive(Clone, Debug)]
pub struct ChoiceExpression {
    pub lhs: Expression,
    pub rhs: Expression,
}

#[derive(Clone, Debug)]
pub struct ChoiceTag {
    pub expr: Expression,
    pub tag: Option<Symbol>,
    pub mode: Option<String>,
    pub ty: Option<Symbol>,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct MarkExpression {
    pub lhs: Symbol,
    pub ty: Option<SymbolPath>,
    pub rhs: Expression,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct UnarySuffix {
    pub suffix: String,
    pub base: Expression,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct UnaryPrefix {
    pub prefix: String,
    pub base: Expression,
    pub range: (usize, usize),
}

#[derive(Clone)]
pub enum Data {
    Symbol(Box<SymbolPath>),
    Integer(Box<Integer>),
    String(Box<StringLiteral>),
    Macro,
    Regex,
}

#[derive(Clone)]
pub struct SymbolPath {
    pub symbol: Vec<Symbol>,
    pub range: (usize, usize),
}

#[derive(Clone)]
pub struct Symbol {
    pub data: String,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct Integer {
    pub data: isize,
    pub range: (usize, usize),
}

#[derive(Clone)]
pub struct StringLiteral {
    pub data: String,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct Eos {
    pub data: bool,
    pub range: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct CommentDocument {
    pub doc: String,
    pub range: (usize, usize),
}
