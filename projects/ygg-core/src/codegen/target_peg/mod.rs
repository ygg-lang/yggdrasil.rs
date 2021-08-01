use self::build_symbol::build_symbol;
use crate::frontend::{
    rule::{ExpressionNode, RefinedChoice, RefinedConcat, RefinedData, RefinedExpression},
    GrammarState,
};
use lrpeg::{
    ast::{BareExpression, Definition, Expression, Grammar},
    Generator,
};
mod build_symbol;
mod test;

impl GrammarState {
    pub fn build_peg(&self) -> String {
        let mut gen = Generator::new();
        let mut grammar = Grammar { lookup: Default::default(), definitions: vec![] };

        for (name, rule) in &self.rule_map {
            grammar.definitions.push(Definition { name: name.to_owned(), sequence: rule.expression.build_peg() });
        }

        // pre populate builtins
        gen.insert_symbol("Dot");
        gen.insert_symbol("WHITESPACE");
        gen.insert_symbol("EOI");
        gen.insert_symbol("XID_IDENTIFIER");

        // collect all symbols
        for def in &grammar.definitions {
            gen.insert_symbol(def.name.to_string());
        }

        for rule in &grammar.definitions {
            gen.collect_terminals_recursive(&rule.sequence);
        }
        let mut res = String::with_capacity(10000);
        gen.emit(&grammar, &mut res);
        return res;
    }
}

impl ExpressionNode {
    pub fn build_peg(&self) -> Expression {
        let label = self.node_tag.as_ref().map(|e| e.data.to_owned());
        let alt = self.node_tag.as_ref().map(|e| e.data.to_owned());
        Expression { label, alt, expr: self.node.build_peg() }
    }
}

impl RefinedExpression {
    pub fn build_peg(&self) -> BareExpression {
        match self {
            Self::Data(v) => v.build_peg(),
            Self::Unary(_) => {
                unimplemented!()
            }
            Self::Choice(v) => v.build_peg(),
            Self::Concat(v) => v.build_peg(),
        }
    }
}

impl RefinedData {
    pub fn build_peg(&self) -> BareExpression {
        match self {
            Self::Symbol(s) => build_symbol(s),
            Self::String(s) => BareExpression::StringLiteral(s.to_owned()),
            Self::Regex(s) => BareExpression::Regex(s.to_owned()),
            Self::Integer(i) => BareExpression::StringLiteral(i.to_string()),
        }
    }
}

impl RefinedChoice {
    pub fn build_peg(&self) -> BareExpression {
        unimplemented!()
    }
}

impl RefinedConcat {
    pub fn build_peg(&self) -> BareExpression {
        unimplemented!()
    }
}
