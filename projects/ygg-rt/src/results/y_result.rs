use super::*;

impl<'i, T> SResult<'i, T> {
    #[inline(always)]
    pub fn map_inner<F, U>(self, f: F) -> SResult<'i, U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            SResult::Pending(state, value) => SResult::Pending(state, f(value)),
            SResult::Stop(reason) => SResult::Stop(reason),
        }
    }
}

use std::ops::{ControlFlow, Try};

impl<'i, T> Try for YYResult<'i, T> {
    type Output = ();
    type Residual = ();

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        todo!()
    }
}
