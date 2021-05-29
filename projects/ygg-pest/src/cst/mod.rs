use crate::ygg::{Rule, YGGParser};
use std::fmt::{Debug, Formatter};
// use crate::{Rule, YGGParser};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use pest::Parser;

use yggdrasil_cst_shared::position_system::{get_position, OffsetRange};

mod marker;

pub type RuleResult<T> = Result<T, Error<Rule>>;

pub struct YGGMarker {}

pub struct CSTNode<'input> {
    pub rule: Rule,
    pub text: &'input str,
    pub mark: Option<&'static str>,
    pub position: OffsetRange,
    pub children: Vec<CSTNode<'input>>,
}

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

impl<'i> Debug for CSTNode<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let node = &mut f.debug_struct("Node");
        node.field("rule", &self.rule);
        if let Some(s) = self.mark {
            node.field("mark", &s);
        }
        match self.children.len() {
            0 => node.field("text", &self.text),
            _ => node.field("children", &self.children),
        };
        node.field("position", &self.position);
        node.finish()
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
