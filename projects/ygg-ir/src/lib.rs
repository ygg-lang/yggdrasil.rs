pub use self::{
    data::{rule_ref::RuleReference, symbol::SymbolAlias, DataKind},
    grammar::GrammarInfo,
    rule::{
        node::{choice::ChoiceExpression, concat::ConcatExpression, unary::UnaryExpression, ExpressionKind, Operator},
        GrammarRule,
    },
};

// mod optimize;
mod data;
mod grammar;
mod rule;
mod traits;
mod typing;
