use std::convert::TryFrom;
use yggdrasil_bootstript::codegen::GrammarManager;
use super::*;

const INCOMPLETE:&str = r#"
grammar! test1

rule1 = a ~ b

fragment! test2
"#;

pub fn test_grammar(text: &str) -> Result<()> {
    let mut parser = YGGBuilder::new()?;
    parser.update_by_text(text)?;
    let out = parser.traverse()?;
    GrammarManager::try_from(out)?;
    Ok(())
}

#[test]
fn test_incomplete() -> Result<()>  {
    test_grammar(INCOMPLETE)
}