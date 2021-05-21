// use crate::ygg::{Rule, YGGParser};
use crate::{Pair, Pairs};
use crate::{Rule, YGGParser};
use pest::error::Error;
use pest::Parser;

#[derive(Debug)]
pub struct OffsetRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct CSTNode<'a> {
    pub text: &'a str,
    pub mark: Option<&'static str>,
    pub range: OffsetRange,
    pub children: Vec<CSTNode<'a>>,
}

pub struct ParserConfig {}

impl ParserConfig {
    pub fn parse_program<'a>(&mut self, input: &'a str) -> Result<CSTNode<'a>, Error<Rule>> {
        let parsed = YGGParser::parse(Rule::program, input)?;
        self.parse_program_inner(parsed)
    }
    fn parse_program_inner<'a>(&self, pairs: Pairs<'a, Rule>) -> Result<CSTNode<'a>, Error<Rule>> {
        let mut node = CSTNode {
            text: pairs.as_str(),
            mark: None,
            range: OffsetRange { start: 0, end: 0 },
            children: vec![],
        };
        for pair in pairs.into_iter() {
            println!("{:#?}", pair)
        }
        return Ok(node);
    }
}

#[test]
fn test() {
    let mut parser = ParserConfig {};
    println!("{:#?}", parser.parse_program("grammar! s; import! k;"))
}
