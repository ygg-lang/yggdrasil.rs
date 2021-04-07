use super::*;

#[derive(Clone, Debug)]
pub enum RefinedExpression {
    Data(),
    Choice(Box<RefinedChoice>),
    Concat(Box<RefinedConcat>)
}

#[derive(Clone, Debug)]
pub struct RefinedChoice {
    inner: Vec<ChoiceItem>
}

#[derive(Clone, Debug)]
pub struct ChoiceItem {
    expr: RefinedExpression
}


#[derive(Clone, Debug)]
pub struct RefinedConcat {
    pub inner: Vec<RefinedExpression>
}