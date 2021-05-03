use super::*;

const DUPLICATE1: &str = r#"
grammar! test

rule = a ~ b

fragment! test
"#;

#[test]
fn test_duplicate1() -> Result<()> {
    assert_diagnostic(DUPLICATE1, include_str!("duplicate1.yaml"))
}

const DUPLICATE2: &str = r#"
rule = a ~ b

rule = a ~ b
"#;

#[test]
fn test_duplicate2() -> Result<()> {
    assert_diagnostic(DUPLICATE2, include_str!("duplicate2.yaml"))
}


#[test]
fn test_bootstrap() -> Result<()> {
    assert_diagnostic(include_str!("../bootstrap.ygg"), include_str!("bootstrap.yaml"))
}