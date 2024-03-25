use crate::{helpers::ParseContext, TakeAnnotations};
use yggdrasil_error::Result;

pub trait AstBuilder<'i> {
    type Output;
    type Context;
    fn build(&self, cfg: &Self::Context, state: &mut ParseContext) -> Result<Self::Output>;
}

pub trait WithAnnotation {
    fn with_annotation(self, extra: TakeAnnotations) -> Self;
}
