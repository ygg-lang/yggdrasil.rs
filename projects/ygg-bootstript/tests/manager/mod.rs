use super::*;


const INCOMPLETE: &str = r#"
grammar! test1

rule1 = a ~ b

fragment! test2
"#;

pub fn test_grammar(text: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let out = parser.traverse()?;
    let diag = out.build_grammar(None)?.show_diagnostic();
    println!("{:?}", diag);
    Ok(())
}

#[test]
fn test_incomplete() -> Result<()> {
    test_grammar(INCOMPLETE)
}
