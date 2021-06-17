use super::*;

pub enum Associativity {
    Left,
    Right,
    Dynamic,
}

#[derive(Clone, Debug)]
pub struct ConcatExpressionResolver<'i> {
    pub base: Pair<'i, Rule>,
    pub rest: Vec<ConcatExpressionRest<'i>>,
}

pub struct ConcatExpressionRest<'i> {
    pub expr: Pair<'i, Rule>,
}

impl ConcatExpressionResolver {
    pub fn fold_left() {}
    pub fn right() {}
}
