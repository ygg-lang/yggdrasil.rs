use super::*;

#[derive(Debug, Clone)]
pub struct SequenceHelper<'a, T> {
    state: YState<'a>,
    result: Option<Parsed<'a, T>>,
}

impl<'a, T> SequenceHelper<'a, T> {
    #[inline]
    pub fn new(state: YState<'a>) -> Self {
        Self { state, result: None }
    }

    #[inline]
    pub fn and_then(mut self, parse_fn: impl FnOnce(YState<'a>) -> YResult<'a, T>) -> Self {
        if self.result.is_none() {
            match parse_fn(self.state.clone()) {
                Ok(ok_result) => self.result = Some(ok_result),
                Err(err) => self.state.set_error(err),
            }
        }
        self
    }

    #[inline]
    pub fn end_choice(self) -> YResult<'a, T> {
        match self.result {
            Some(ok) => Ok(ok),
            None => Err(self.state.get_error()),
        }
    }
}
