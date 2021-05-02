use tree_sitter_cli::generate::grammars::InputGrammar;
use super::*;

const UNARY: &str = r#"
grammar! unary
test1 = e1?;
test2 = e1+;
test3 = e1*;
test4 = ^e1;
"#;

#[test]
fn test_unary() -> Result<()> {
    let text = UNARY;
    let target = include_str!("unary.yaml");


    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let mut grammar = parser.traverse()?.build_grammar((*EXAMPLE_URL).clone())?.0;
    grammar.optimize_local()?;
    let out = grammar.build_input_grammar();
    assert_eq!(format!("{:#?}",out), target);
    Ok(())

}