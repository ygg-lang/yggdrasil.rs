use super::*;

#[derive(Clone, Debug)]
pub enum RefinedExpression {
    Choice(Box<RefinedChoice>)
}

#[derive(Clone, Debug)]
pub struct RefinedChoice {
    inner: Vec<RefinedExpression>
}