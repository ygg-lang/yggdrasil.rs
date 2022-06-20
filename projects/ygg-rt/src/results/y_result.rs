use super::*;

impl<'i, T> YYResult<'i, T> {
    #[inline(always)]
    pub fn map_inner<F, U>(self, f: F) -> YYResult<'i, U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            YYResult::Pending(state, value) => YYResult::Pending(state, f(value)),
            YYResult::Stop(reason) => YYResult::Stop(reason),
        }
    }
}

#[cfg(nightly)]
mod nightly {
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
}
