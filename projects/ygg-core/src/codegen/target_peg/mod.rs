use peginator::{codegen::Grammar, grammar::{Grammar_rules, CharRule}};

use crate::frontend::{
    rule::{DataExpression, Term},
    GrammarInfo, Rule,
};

mod build_symbol;
mod test;

impl GrammarInfo {
    pub fn as_peg(&self) -> Grammar {
        let mut rules = vec![];
        for (_, rule) in &self.rules {
            rules.push(rule.as_peg(self))
        }
        return Grammar { rules };
    }
}

impl Rule {
    pub fn as_peg(&self, info: &GrammarInfo) -> Grammar_rules {
        match self.body.node {
            Term::Unary(expr) => {}
            Term::Choice(expr) => {}
            Term::Concat(expr) => {}
            Term::Data(expr) => expr.as_peg(),
        }
    }
}

impl DataExpression {
    pub fn as_peg(&self, info: &GrammarInfo) -> Grammar_rules {
        match self {
            DataExpression::String(data) => {}
            DataExpression::Regex(data) => {}
            DataExpression::Integer(data) => {}
            DataExpression::Character(data) => {
                CharRule {

                }


                Grammar_rules::CharRule()

            }
            DataExpression::CharacterRange(data) => {}
            DataExpression::CharacterSet(data) => {}
        }
    }
}
