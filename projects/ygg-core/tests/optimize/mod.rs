use super::*;

const UNARY_FUSION: &str = r#"
grammar! unary
test1 = e1?;
test2 = e1+;
test3 = e1*;
test4 = ^e1;
test5 = ^e1+;

test6 = e1+?;
test7 = e1+*;
test8 = e1?+;
test9 = (^e1?)+;

test10 = (^e1? ~ ^e2*)+
test11 = (^e1? ~ ^e2+)*
"#;

#[test]
fn test_unary() -> Result<()> {
    assert_optimize(UNARY_FUSION, include_str!("unary_fusion.yaml"))
}

const OR_FUSION: &str = r#"
grammar! or
test1 = a | a
test2 = a | b | b
test3 = 0 | 0
test4 = 0 | 1 | 1
test5 = "a" | "a"
test6 = "a" | "a" | "b"
"#;

#[test]
fn test_or_fusion() -> Result<()> {
    assert_optimize(OR_FUSION, include_str!("or_fusion.yaml"))
}

const STRING_FUSION: &str = r#"
grammar! string
test1 = 0 | "0" | "1"
test2 = "a" | "a" | "b"
"#;

#[test]
fn test_string_fusion() -> Result<()> {
    assert_optimize(STRING_FUSION, include_str!("string_fusion.yaml"))
}

#[test]
fn test_bootstrap() -> Result<()> {
    assert_optimize(include_str!("../bootstrap.ygg"), include_str!("bootstrap.yaml"))
}
