use tree_sitter_cli::{
    generate::grammars::{InputGrammar, Variable, PrecedenceEntry, VariableType},
    generate::rules::{Rule, MetadataParams, Associativity, Alias, Symbol, SymbolType},
    generate::generate_parser_in_directory,
    generate::prepare_grammar::prepare_grammar,
    generate::{GeneratedParser, generate_parser_for_grammar_with_opts},
};

///
use tree_sitter_cli::generate::parse_grammar::parse_grammar;

pub fn test_input_grammar() -> InputGrammar {
    let r1 = Rule::Choice(
        vec![
            Rule::NamedSymbol(
                String::from("grammar_statement"),
            ),
            Rule::NamedSymbol(
                String::from("fragment_statement"),
            ),
            Rule::NamedSymbol(
                String::from("assign_statement"),
            ),
        ],
    );

    let v1 = Variable {
        name: "".to_string(),
        kind: VariableType::Hidden,
        rule: r1,
    };


    InputGrammar {
        name: "yg".to_string(),
        variables: vec![v1],
        extra_symbols: vec![],
        expected_conflicts: vec![],
        precedence_orderings: vec![],
        external_tokens: vec![],
        variables_to_inline: vec![],
        supertype_symbols: vec![],
        word_token: None,
    }
}

#[test]
fn test() {
    let grammar_json = include_str!("../../../tree-sitter-yg/src/grammar.json");
    let input_grammar = parse_grammar(&grammar_json).unwrap();
    let (syntax_grammar, lexical_grammar, inlines, simple_aliases) = prepare_grammar(&input_grammar).unwrap();
    let language_name = input_grammar.name;

    // Generate the parser and related files.
    let GeneratedParser {
        c_code: _,
        node_types_json,
    } = generate_parser_for_grammar_with_opts(
        &language_name,
        syntax_grammar,
        lexical_grammar,
        inlines,
        simple_aliases,
        true,
        None,
    ).unwrap();
    // println!("{}", c_code);
    println!("{}", node_types_json);
}
