use std::ops::{Add, AddAssign};
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct HintItems {
    pub diagnostic: Vec<Diagnostic>,
    pub code_lens: Vec<CodeLens>,
}


impl Default  for HintItems {
    fn default() -> Self {
        Self{
            diagnostic: vec![],
            code_lens: vec![]
        }
    }
}

impl Add<Self> for HintItems {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Self> for HintItems {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}