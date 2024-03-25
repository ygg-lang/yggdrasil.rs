use crate::states::{ParseContext, ParseState};

use crate::helpers::TakeAnnotations;
use yggdrasil_error::Result;

pub trait AstBuilder<'i> {
    type Output;
    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> Result<Self::Output>;
}

pub trait WithAnnotation {
    fn with_annotation(self, extra: TakeAnnotations) -> Self;
}
