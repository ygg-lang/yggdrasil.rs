use super::*;

#[derive(Clone, Debug)]
pub struct Program {
    pub children: Vec<Statement>,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub enum Statement {
    GrammarStatement(Box<GrammarStatement>),
    FragmentStatement(Box<FragmentStatement>),
    AssignStatement(Box<AssignStatement>),
}

#[derive(Clone, Debug)]
pub struct GrammarStatement {
    pub id: Identifier,
    pub eos: Eos,
    pub range: Range,
    pub children: Vec<AuxNode2>,
}

#[derive(Clone, Debug)]
pub enum AuxNode2 {
    GrammarStatement(String),
    OPCOMMA,
    OpLeft,
    OpRight,
}

#[derive(Clone, Debug)]
pub struct FragmentStatement {
    pub id: Identifier,
    pub range: Range
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Identifier,
    pub eos: Eos,
    pub children: Vec<AuxNode2>,
}

#[derive(Clone, Debug)]
pub struct Identifier {
   pub data: String,
   pub range: Range
}

#[derive(Clone, Debug)]
pub struct Eos {
   pub data: bool,
   pub range: Range
}
