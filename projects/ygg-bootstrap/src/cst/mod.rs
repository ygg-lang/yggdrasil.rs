#![allow(non_snake_case, non_camel_case_types)]
#![allow(unused_variables, dead_code)]

// #[rustfmt::skip]
mod parse;
mod rule;

use crate::cst::parse::Node;
use pest::error::Error;
use pest::iterators::Pairs;
use pest::Atomicity;
use pest::Parser;
use pest::ParserState;
use yggdrasil_shared::{ignore_terms, match_charset, tag_branch, tag_node};

pub use self::rule::Rule;

pub type RuleState<'a> = Box<ParserState<'a, Rule>>;
pub type RuleResult<'a> = Result<RuleState<'a>, RuleState<'a>>;

pub struct CSTBuilder {}

pub fn flatten(node: Node) -> Node {
    let mut buffer = vec![];
    for node in node.children {
        flatten_rec(node, &mut buffer)
    }
    Node {
        rule: node.rule,
        start: node.start,
        end: node.end,
        children: buffer,
        label: node.label,
        alternative: node.alternative,
    }
}

pub fn flatten_rec(node: Node, buffer: &mut Vec<Node>) {
    match node.rule {
        // flatten these nodes
        Rule::Any | Rule::List => {
            for node in node.children {
                flatten_rec(node, buffer)
            }
        }
        // not important
        Rule::EOI => {}
        // Rule::_expr0|Rule::_expr1|Rule::_expr2|Rule::_expr3|Rule::_expr4|Rule::_expr5|Rule::_expr6 => {
        //     let len = node.children.len();
        //     match len {
        //         0 => {  },
        //         1 => {
        //             let mut children = node.children;
        //             flatten_rec(children.remove(0), buffer)
        //         },
        //         _ => buffer.push(flatten(node))
        //     }
        // }
        Rule::IGNORE | Rule::Terminal | Rule::MustNotMatch => {}
        /*
        #[cfg(feature = "no-ignored")]
        Rule::IGNORE => {}
        #[cfg(not(feature = "no-ignored"))]
        Rule::IGNORE if node.start == node.end => {}
        #[cfg(feature = "no-unnamed")]
        Rule::Terminal => {}
        #[cfg(not(feature = "no-unnamed"))]
        Rule::Terminal if node.start == node.end => {}
        */
        _ => buffer.push(flatten(node)),
    }
}
