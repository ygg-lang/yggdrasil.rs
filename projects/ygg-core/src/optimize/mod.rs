use itertools::Itertools;
use std::mem::take;
use yggdrasil_error::{Validate, Validation, YggdrasilError};
pub use yggdrasil_ir::traits::{CodeGenerator, CodeOptimizer};
use yggdrasil_ir::{
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionBody, UnaryExpression, YggdrasilExpression},
    rule::{FunctionRule, GrammarAtomic, GrammarBody, GrammarRule},
    IndexMap,
};
mod emit_function;
mod fuse_rule;
mod inline_rules;
mod insert_ignore;
mod refine_rules;
mod remark_tag;

pub use self::{
    emit_function::EmitFunction, fuse_rule::FusionRules, inline_rules::InlineRules, insert_ignore::InsertIgnore,
    refine_rules::RefineRules, remark_tag::RemarkTags,
};
