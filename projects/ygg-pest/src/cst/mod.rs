use crate::ygg::{Rule, YGGParser};
// use crate::{Rule, YGGParser};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
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
        match parsed.into_iter().next() {
            None => {
                unimplemented!()
            }
            Some(p) => self.mark_program(p, None),
        }
    }
    pub fn mark_program<'i>(&self, pairs: Pair<'i, Rule>, mark: Option<&'static str>) -> RuleResult<CSTNode<'i>> {
        let mut node = Self::new_node(&pairs, mark);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::WHITESPACE => self.split_whitespace(&mut node.children, pair)?,
                Rule::statement => node.children.push(self.mark_statement(pair, Some("statement"))?),
                _ => {
                    unreachable!("{:#?}", pair);
                    Self::unreachable(&pair)?
                }
            }
        }
        return Ok(node);
    }

    pub fn mark_statement<'i>(&self, pairs: Pair<'i, Rule>, mark: Option<&'static str>) -> RuleResult<CSTNode<'i>> {
        let mut node = Self::new_node(&pairs, mark);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => self.split_whitespace(&mut node.children, pair)?,
                _ => node.children.push(self.mark_any(pair)?),
            }
        }
        return Ok(node);
    }

    pub fn mark_any<'i>(&self, pairs: Pair<'i, Rule>) -> RuleResult<CSTNode<'i>> {
        let mut node = Self::new_node(&pairs, None);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => self.split_whitespace(&mut node.children, pair)?,
                _ => node.children.push(self.mark_any(pair)?),
            }
        }
        return Ok(node);
    }
}

#[test]
fn test() {
    let mut parser = YGGMarker {};
    let out = parser.parse_program(
        r#"
    ignore! k; ///2
    ignore! [ s ];
    "#,
    );
    println!("{:#?}", out.unwrap());
}
