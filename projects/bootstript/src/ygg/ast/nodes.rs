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
    pub ext: Vec<String>,
    pub range: Range,
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


impl Default for Identifier {
    fn default() -> Self {
        Self {
            data: "".to_string(),
            range: Range {
                start_byte: 0,
                end_byte: 0,
                start_point: Default::default(),
                end_point: Default::default()
            }
        }
    }
}