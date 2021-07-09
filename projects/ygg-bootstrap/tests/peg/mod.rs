use yggdrasil_bootstrap::cst::CSTBuilder;

pub fn peg_assert(input: &str, target: &str) {
    let mut parser = CSTBuilder::default();
    let out = parser.parse(input).unwrap();
    assert_eq!(format!("{:#?}", out), target)
}

#[test]
fn grammar() {
    let input = r#"
grammar! ygg
grammar! ygg "*.ygg";
grammar! ygg { }
grammar! ygg {"*.ygg",};
fragment! ygg_ex;
"#;
    peg_assert(input, include_str!("grammar.yaml"))
}

#[test]
fn ignore() {
    let input = r#"
ignore! a;
ignore! {a, b, c};
"#;
    peg_assert(input, include_str!("ignore.yaml"))
}

#[test]
fn import() {
    let input = r#"
import! ">root/"
import! "@root/" {a, b as c, d}
"#;
    peg_assert(input, include_str!("import.yaml"))
}

#[test]
fn atom() {
    let input = r#"
x = symbol
x = 12345
x = "\"string"
x = '"string"'
"#;
    peg_assert(input, include_str!("atom.yaml"))
}

#[test]
fn assign() {
    let input = r#"
x = / 1 ~ 2 ~ 3
x = a: T <- x
x = a | b | c
x = ^!field*?
"#;
    peg_assert(input, include_str!("assign.yaml"))
}

#[test]
fn space() {
    let input = r#"
x = a b | c d
"#;
    peg_assert(input, include_str!("space.yaml"))
}
