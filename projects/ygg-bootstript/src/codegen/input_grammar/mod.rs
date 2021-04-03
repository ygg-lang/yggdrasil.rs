// use tree_sitter_cli::generate::{
//     generate_parser_for_grammar_with_opts, generate_parser_in_directory,
//     grammars::{InputGrammar, PrecedenceEntry, Variable, VariableType},
//     prepare_grammar::prepare_grammar,
//     rules::{Alias, Associativity, MetadataParams, Rule, Symbol, SymbolType},
//     GeneratedParser,
// };

use std::path::PathBuf;
use tree_sitter_cli::generate::{generate_parser_for_grammar};
use tree_sitter_cli::generate::parse_grammar::{GrammarJSON, parse_grammar};
use tree_sitter_cli::generate::grammars::{InputGrammar, Variable, VariableType};
use tree_sitter_cli::generate::grammars::VariableType::Named;
use tree_sitter_cli::generate::rules::{MetadataParams, Precedence, Rule};
use tree_sitter_cli::generate::rules::Rule::{Blank, Choice, Metadata, NamedSymbol, Repeat};

#[test]
pub fn test() {
    let grammar: GrammarJSON = serde_json::from_str(include_str!("../../../../tree-sitter-ygg/src/grammar.json")).unwrap();
    let grammar = parse_grammar(grammar).unwrap();

    // let (_name, _c_code) = generate_parser_for_grammar(
    //     grammar
    // ).unwrap();
    let extra_symbols = vec![
        Rule::NamedSymbol(String::from("NEWLINE")),
        Rule::NamedSymbol(String::from("WHITESPACE")),
    ];
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
                        field_name: Some(
                            String::from("statement"),
                        ),
                    },
                    rule: Box::new(NamedSymbol(
                        String::from("statement"),
                    )),
                })),
                Blank,
            ],

            ),
        },
        Variable {
            name: String::from("statement"),
            kind: Named,
            rule: Choice(
                vec![
                    NamedSymbol(
                        String::from("grammar_statement"),
                    ),
                    NamedSymbol(
                        String::from("fragment_statement"),
                    ),
                    NamedSymbol(
                        String::from("assign_statement"),
                    ),
                ],
            ),
        },
    ];
    debug_assert_eq!(
        format!("{:#?}", variables),
        format!("{:#?}", grammar.variables),
    );
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
    //println!("{:#?}",grammar);
}

