use super::*;

#[derive(Clone, Debug)]
pub enum RefinedExpression {
    Data(Box<RefinedData>),
    Choice(Box<RefinedChoice>),
    Concat(Box<RefinedConcat>)
}

#[derive(Clone, Debug)]
pub struct RefinedChoice {
    pub  inner: Vec<RefinedTag>
}

#[derive(Clone, Debug)]
pub struct RefinedTag {
   pub expr: RefinedExpression
}


#[derive(Clone, Debug)]
pub struct RefinedConcat {
    pub inner: Vec<RefinedExpression>
}

#[derive(Clone, Debug)]
pub enum RefinedData {
    Identifier {
        id: String,
        inline: bool
    },
    String(String),
    Regex(String),
    Integer(usize),
}