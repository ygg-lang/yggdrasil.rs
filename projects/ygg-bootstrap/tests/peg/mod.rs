use yggdrasil_bootstrap::{flatten, PEG};

pub fn peg_assert(input: &str, target: &str) {
    let mut p = PEG::new();
    let out = flatten(p.parse(input).unwrap());
    assert_eq!(format!("{:#?}", out), target)
}

#[test]
fn grammar() {
    let input = r#"
grammar! ygg
grammar! ygg "*.ygg";
grammar! ygg { }
grammar! ygg {"*.ygg",};
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
fn assign() {
    peg_assert("x = | 1 ~ 2 | 3", include_str!("assign.yaml"))
}
// a | b ~ c | d <- e


#[test]
fn import() {
    let input = r#"
import! ">root/"
import! ">root/" {a, b as c, d}
"#;
    peg_assert(input, include_str!("import.yaml"))
}