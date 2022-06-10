use yggdrasil_parser::{parse_program, ClassStatementNode};
use yggdrasil_rt::ast_mode::YState;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn test_bootstrap2() {
    let state = YState::new("entry class {}");
    let out = ClassStatementNode::consume(state);
    println!("{:#?}", out)
}

#[test]
fn test_bootstrap() {
    let out = parse_program(include_str!("../src/bootstrap.ygg")).unwrap();
    println!("{:#?}", out)
}
