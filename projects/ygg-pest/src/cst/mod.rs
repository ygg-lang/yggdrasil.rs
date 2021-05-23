use crate::ygg::{Rule, YGGParser};
use crate::{Pair, Pairs};
// use crate::{Rule, YGGParser};
use pest::error::Error;
use pest::Parser;

use yggdrasil_cst_shared::position_system::{get_position, OffsetRange};

mod marker;

#[derive(Debug)]
pub struct CSTNode<'input> {
    pub text: &'input str,
    pub mark: Option<&'static str>,
    pub position: OffsetRange,
    pub children: Vec<CSTNode<'input>>,
}

pub type RuleResult<T> = Result<T, Error<Rule>>;

pub struct YGGMarker {}

impl YGGMarker {
    pub fn parse_program<'i>(&mut self, input: &'i str) -> RuleResult<CSTNode<'i>> {
        let parsed = YGGParser::parse(Rule::program, input)?;
        marker::mark_program(parsed)
    }
}



#[test]
fn test() {
    let mut parser = YGGMarker {};
    let out = parser.parse_program(r#"
    ignore! k; ///2
    ignore! [ s ];
    "#);
    println!("{:#?}", out.unwrap());
}
