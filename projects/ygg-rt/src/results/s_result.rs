use super::*;

impl<'i, T> SResult<'i, T> {
    #[inline(always)]
    pub fn map_inner<F, U>(self, f: F) -> SResult<'i, U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Pending(state, value) => SResult::Pending(state, f(value)),
            Self::Stop(reason) => SResult::Stop(reason),
        }
    }
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    pub fn as_result(self) -> Result<Parsed<'i, T>, StopBecause> {
        match self {
            Self::Pending(state, value) => Ok((state, value)),
            Self::Stop(reason) => Err(reason),
        }
    }
}
