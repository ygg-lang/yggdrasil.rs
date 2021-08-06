use self::build_symbol::build_symbol;
use crate::frontend::{
    rule::{ExpressionNode, RefinedChoice, RefinedConcat, RefinedData, RefinedExpression, RefinedUnary},
    GrammarInfo,
};
use lrpeg::{
    ast::{BareExpression, Definition, Expression, Grammar},
    Generator,
};

mod build_symbol;
mod test;

impl GrammarInfo {
    pub fn build_peg(&self) -> (Grammar, Generator) {
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
        return (grammar, gen);
    }
    pub fn build_peg_code(&self) -> String {
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
        Expression { label, alt, expr: self.node.build_peg().expr }
    }
}

impl RefinedExpression {
    pub fn build_peg(&self) -> Expression {
        match self {
            Self::Data(e) => e.build_peg(),
            Self::Unary(e) => e.build_peg(),
            Self::Choice(e) => e.build_peg(),
            Self::Concat(e) => e.build_peg(),
        }
    }
}

impl RefinedUnary {
    pub fn build_peg(&self) -> Expression {
        self.base.build_peg()
    }
}

impl RefinedData {
    pub fn build_peg(&self) -> Expression {
        let bare = match self {
            Self::Symbol(s) => build_symbol(s),
            Self::String(s) => BareExpression::StringLiteral(s.to_owned()),
            Self::Regex(s) => BareExpression::Regex(s.to_owned()),
            Self::Integer(i) => BareExpression::StringLiteral(i.to_string()),
        };
        Expression { label: None, alt: None, expr: bare }
    }
}

impl RefinedChoice {
    pub fn build_peg(&self) -> Expression {
        unimplemented!()
    }
}

impl RefinedConcat {
    pub fn build_peg(&self) -> Expression {
        unimplemented!()
    }
}
