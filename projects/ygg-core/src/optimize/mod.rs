use itertools::Itertools;
use std::collections::HashSet;
use yggdrasil_ir::{
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionKind, UnaryExpression, YggdrasilExpression},
    rule::{GrammarAtomic, GrammarRule},
    traits::{CodeOptimizer, FieldDescriptor},
    IndexMap, QError, Validation,
};

mod dead_code;
mod inlining;
mod insert_ignore;

mod refine;

pub use self::{dead_code::DeadCodeEliminator, inlining::InlineRules, insert_ignore::InsertIgnore, refine::RefineRules};
