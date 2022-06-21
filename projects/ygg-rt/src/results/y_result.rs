use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Try},
};

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

impl<'i, T> FromResidual for SResult<'i, T> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        match residual {
            SResult::Pending(_, _) => {
                unreachable!()
            }
            SResult::Stop(e) => SResult::Stop(e),
        }
    }
}

impl<'i, T> FromResidual<Result<Infallible, StopBecause>> for SResult<'_, T> {
    fn from_residual(residual: Result<Infallible, StopBecause>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => SResult::Stop(e),
        }
    }
}

impl<'i, T> Try for SResult<'i, T> {
    type Output = (YState<'i>, T);
    type Residual = SResult<'i, Infallible>;

    fn from_output(output: Self::Output) -> Self {
        SResult::Pending(output.0, output.1)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            SResult::Pending(state, value) => ControlFlow::Continue((state, value)),
            SResult::Stop(e) => ControlFlow::Break(SResult::Stop(e)),
        }
    }
}
