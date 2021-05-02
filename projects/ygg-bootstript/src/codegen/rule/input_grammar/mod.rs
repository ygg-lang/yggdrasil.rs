// use tree_sitter_cli::generate::{
//     generate_parser_for_grammar_with_opts, generate_parser_in_directory,
//     grammars::{InputGrammar, PrecedenceEntry, Variable, VariableType},
//     prepare_grammar::prepare_grammar,
//     rules::{Alias, Associativity, MetadataParams, Rule, Symbol, SymbolType},
//     GeneratedParser,
// };

use super::*;

use std::path::PathBuf;
use tree_sitter_cli::generate::{
    generate_parser_for_grammar,
    grammars::{InputGrammar, Variable, VariableType, VariableType::Named},
    parse_grammar::{parse_grammar, GrammarJSON},
    rules::{
        MetadataParams, Precedence, Rule,
        Rule::{Blank, Choice, Metadata, NamedSymbol, Repeat},
    },
};

impl GrammarState {
    pub fn build_input_grammar(&self) -> InputGrammar {
        InputGrammar {
            name: self.name.data.to_owned(),
            variables: vec![],
            extra_symbols: vec![],
            expected_conflicts: vec![],
            precedence_orderings: vec![],
            external_tokens: vec![],
            variables_to_inline: vec![],
            supertype_symbols: vec![],
            word_token: None,
        }
    }
}

impl YGGRule {
    pub fn build_variable(&self) -> Variable {
        Variable { name: self.name.to_owned(), kind: VariableType::Named, rule: Rule::Blank }
    }
}

impl From<RefinedExpression> for Rule {
    fn from(e: RefinedExpression) -> Self {
        match e {
            RefinedExpression::Data(r) => Self::from(*r),
            RefinedExpression::Choice(r) => Self::from(*r),
            RefinedExpression::Concat(r) => Self::from(*r),
            RefinedExpression::Unary(r) => Self::from(*r),
        }
    }
}

impl From<RefinedUnary> for Rule {
    fn from(_: RefinedUnary) -> Self {
        unimplemented!()
    }
}

impl From<RefinedConcat> for Rule {
    fn from(e: RefinedConcat) -> Self {
        Self::Choice(e.inner.iter().cloned().map(|e| e.into()).collect())
    }
}

impl From<RefinedData> for Rule {
    fn from(data: RefinedData) -> Self {
        match data {
            RefinedData::String(s) => Self::String(s),
            RefinedData::Regex(s) => Self::Pattern(s),
            RefinedData::Integer(s) => Self::String(s.to_string()),
            RefinedData::Identifier(s) => Self::NamedSymbol(s.data),
        }
    }
}

impl From<RefinedChoice> for Rule {
    fn from(e: RefinedChoice) -> Self {
        todo!("{:?}", e)
    }
}

#[test]
pub fn test() {
    let grammar: GrammarJSON =
        serde_json::from_str(include_str!("../../../../../tree-sitter-ygg/src/tree_sitter/grammar.json")).unwrap();
    let grammar = parse_grammar(grammar).unwrap();
    // let (_name, _c_code) = generate_parser_for_grammar(
    //     grammar
    // ).unwrap();
    let extra_symbols = vec![Rule::NamedSymbol(String::from("NEWLINE")), Rule::NamedSymbol(String::from("WHITESPACE"))];
    let variables = vec![
        Variable {
            name: String::from("program"),
            kind: VariableType::Named,
            rule: Rule::Choice(vec![
                Rule::Repeat(Box::from(Metadata {
                    params: MetadataParams {
                        precedence: Precedence::None,
                        dynamic_precedence: 0,
                        associativity: None,
                        is_token: false,
                        is_string: false,
                        is_active: false,
                        is_main_token: false,
                        alias: None,
                        field_name: Some(String::from("statement")),
                    },
                    rule: Box::new(NamedSymbol(String::from("statement"))),
                })),
                Blank,
            ]),
        },
        Variable {
            name: String::from("statement"),
            kind: Named,
            rule: Choice(vec![
                NamedSymbol(String::from("grammar_statement")),
                NamedSymbol(String::from("fragment_statement")),
                NamedSymbol(String::from("assign_statement")),
            ]),
        },
    ];
    debug_assert_eq!(format!("{:#?}", variables), format!("{:#?}", grammar.variables),);
    let _rhs = InputGrammar {
        name: String::from("ygg"),
        variables,
        extra_symbols,
        expected_conflicts: vec![],
        precedence_orderings: vec![],
        external_tokens: vec![],
        variables_to_inline: vec![],
        supertype_symbols: vec![],
        word_token: Some(String::from("id")),
    };
    // println!("{:#?}",grammar);
}
