use crate::ygg::{Rule, YGGParser};
use crate::{Pair, Pairs};
// use crate::{Rule, YGGParser};
use pest::error::Error;
use pest::Parser;

mod marker;



#[derive(Debug)]
pub struct CSTNode<'a> {
    pub text: &'a str,
    pub mark: Option<&'static str>,
    pub range: OffsetRange,
    pub children: Vec<CSTNode<'a>>,
}

pub type RuleResult<T> = Result<T, Error<Rule>>;

pub struct YGGMarker {}

impl YGGMarker {
    pub fn parse_program<'a>(&mut self, input: &'a str) -> RuleResult<CSTNode<'a>>  {
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
