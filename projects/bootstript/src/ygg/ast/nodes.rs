use super::*;

#[derive(Clone, Debug)]
pub struct Program {
    pub children: Vec<AuxNode1>,
    pub range: Range,
}

#[derive(Clone, Debug)]
pub enum AuxNode1 {
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
    pub eos: Eos,
    pub children: Vec<AuxNode2>,
}

#[derive(Clone, Debug)]
pub struct AssignStatement {
    pub id: Identifier,
    pub eos: Eos,
    pub children: Vec<AuxNode2>,
}


#[derive(Clone, Debug)]
pub struct Identifier(pub String);

#[derive(Copy, Clone, Debug)]
pub struct Eos(pub bool);
