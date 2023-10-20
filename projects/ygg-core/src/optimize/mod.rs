use itertools::Itertools;
use std::mem::take;
use yggdrasil_error::YggdrasilError;
pub use yggdrasil_ir::traits::{CodeGenerator, CodeOptimizer};
use yggdrasil_ir::{
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionBody, UnaryExpression, YggdrasilExpression},
    rule::{FunctionRule, GrammarAtomic, GrammarRule},
    IndexMap,
};
mod emit_function;
mod fuse_rule;
mod insert_ignore;
mod remark_tag;

mod refine;

pub use self::{
    emit_function::EmitFunction, fuse_rule::FusionRules, insert_ignore::InsertIgnore, refine::RefineRules,
    remark_tag::RemarkTags,
};
