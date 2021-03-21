use super::*;


const CONCAT_SIMPLE: &str = r#"
test1 = e1 ~ e2;
test2 = e1 ~ e2 ~ e3 ~ e4;
test3 = e1 ~ ((e2 ~ e3) ~ e4);
test4 = (e1 ~ e2) ~ (e3 ~ e4);
"#;


#[test]
fn test_concat_simple() -> Result<()> {
    assert_ast(CONCAT_SIMPLE, include_str!("concat_simple.yaml"))
}
