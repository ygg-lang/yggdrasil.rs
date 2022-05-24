use super::*;

#[derive(Debug, Clone)]
pub struct SequenceHelper<'a, T> {
    state: ParseState<'a>,
    result: Option<Parsed<'a, T>>,
}

impl<'a, T> SequenceHelper<'a, T> {
    #[inline]
    pub fn new(state: ParseState<'a>) -> Self {
        Self { state, result: None }
    }

    #[inline]
    pub fn and_then(mut self, parse_fn: impl FnOnce(ParseState<'a>) -> IResult<'a, T>) -> Self {
        if self.result.is_none() {
            match parse_fn(self.state.clone()) {
                Ok(ok_result) => self.result = Some(ok_result),
                Err(err) => self.state.set_error(err),
            }
        }
        self
    }

    #[inline]
    pub fn end_choice(self) -> IResult<'a, T> {
        match self.result {
            Some(ok) => Ok(ok),
            None => Err(self.state.get_error()),
        }
    }
}
