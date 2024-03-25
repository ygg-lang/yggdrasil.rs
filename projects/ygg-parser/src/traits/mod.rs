use crate::states::{ParseContext, ParseState};

use yggdrasil_error::Result;

pub trait AstBuilder<'i> {
    type Output;
    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output>;
}
