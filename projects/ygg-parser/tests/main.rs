use yggdrasil_parser::{parse_program, ClassStatementNode, IgnoredNode, ProgramNode};
use yggdrasil_rt::ast_mode::YState;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn test_bootstrap2() {
    let state = YState::new("entry class Aaa { \n }  ");
    let out = ClassStatementNode::consume(state);
    println!("{:#?}", out)
}

#[test]
fn test_bootstrap() {
    let state = YState::new(include_str!("../src/bootstrap.ygg"));
    let (out, _) = IgnoredNode::consume(state).unwrap();
    let out = ProgramNode::consume(out);
    println!("{:#?}", out)
}
