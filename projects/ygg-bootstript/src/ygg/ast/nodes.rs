use super::*;

#[derive(Clone)]
pub struct Program {
    pub statement: Vec<Statement>,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
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
    pub ext: Vec<StringRanged>,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct FragmentStatement {
    pub id: Identifier,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Identifier,
    pub eq: String,
    pub rhs: Expression,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct IgnoreStatement {
    pub rules: Vec<StringRanged>,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone)]
pub enum Expression {
    // ErrorNode,
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
    pub op: String,
    pub rhs: Expression,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct ChoiceExpression {
    pub lhs: ChoiceTag,
    pub op: String,
    pub rhs: ChoiceTag,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct ChoiceTag {
    pub expr: Expression,
    pub tag: Option<Identifier>,
    pub mode: Option<String>,
    pub ty: Option<Identifier>,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct FieldExpression {
    pub lhs: Identifier,
    pub op: String,
    pub rhs: Expression,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct UnarySuffix {
    pub suffix: String,
    pub base: Expression,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct UnaryPrefix {
    pub prefix: String,
    pub base: Expression,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
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
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct Unsigned {
    pub data: usize,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct StringLiteral {
    pub data: String,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct StringRanged {
    pub data: String,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}

#[derive(Clone, Debug)]
pub struct Eos {
    pub data: bool,
    #[cfg(feature = "lsp")]
    pub range: lsp_types::Range,
    #[cfg(not(feature = "lsp"))]
    pub range: tree_sitter::Range,
}
