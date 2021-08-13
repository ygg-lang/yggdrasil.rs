use super::*;

const SIMPLE: &str = r#"
a = b | c?;
b = 2? ~ "3"+;
"#;

#[test]
fn test_simple() -> Result<()> {
    assert_codegen(SIMPLE, include_str!("./simple.svg"))
}
