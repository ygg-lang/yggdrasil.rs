use super::*;

#[derive(Debug, Clone)]
pub struct ChoiceHelper<'a, T> {
    state: YState<'a>,
    result: Option<Parsed<'a, T>>,
}

impl<'i> YState<'i> {
    #[inline]
    pub fn start_choice<T>(self) -> ChoiceHelper<'i, T> {
        ChoiceHelper { state: self, result: None }
    }
}

impl<'a, T> ChoiceHelper<'a, T> {
    #[inline]
    pub fn new(state: YState<'a>) -> Self {
        Self { state, result: None }
    }

    #[inline]
    pub fn or_else(mut self, parse_fn: impl FnOnce(YState<'a>) -> YResult<'a, T>) -> Self {
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
