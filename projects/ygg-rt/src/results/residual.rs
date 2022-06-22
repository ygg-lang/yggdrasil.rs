use super::*;

impl<'i, T> Try for SResult<'i, T> {
    type Output = Parsed<'i, T>;
    type Residual = SResult<'i, Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Self::Pending(output.0, output.1)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Pending(state, value) => ControlFlow::Continue((state, value)),
            Self::Stop(e) => ControlFlow::Break(SResult::Stop(e)),
        }
    }
}

impl<'i, T> FromResidual for SResult<'i, T> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        match residual {
            SResult::Pending(_, _) => unreachable!(),
            SResult::Stop(e) => Self::Stop(e),
        }
    }
}

impl<'i, T> FromResidual<Result<Infallible, StopBecause>> for SResult<'_, T> {
    fn from_residual(residual: Result<Infallible, StopBecause>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => Self::Stop(e),
        }
    }
}
