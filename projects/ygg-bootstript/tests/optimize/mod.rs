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
    assert_optimize(UNARY, include_str!("unary.yaml"))
}

#[test]
fn test_bootstrap() -> Result<()> {
    assert_optimize(include_str!("../bootstrap.ygg"), include_str!("bootstrap.yaml"))
}
