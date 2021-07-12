#![allow(non_snake_case, non_camel_case_types)]
#![allow(unused_variables, dead_code)]

include!(concat!(env!("OUT_DIR"), "/ygg.rs"));

mod parse;
pub use self::ygg::{Node, Rule, PEG};
//pub use self::parse::{Node, Rule, PEG};
use yggdrasil_shared::records::CSTNode;
use yggdrasil_shared::{Error, Result};

pub struct CSTBuilder {
    pub peg: PEG,
    pub error: Vec<Error>,
}

impl Default for CSTBuilder {
    fn default() -> Self {
        Self { peg: PEG::new(), error: vec![] }
    }
}

impl CSTBuilder {
    pub fn parse(&mut self, input: &str) -> Result<CSTNode<Rule>> {
        match self.peg.parse(input) {
            Ok(o) => Ok(flatten(o)),
            Err(e) => Err(Error::node_missing("E")),
        }
    }
}

fn flatten(node: Node) -> CSTNode<Rule> {
    let mut buffer = vec![];
    for node in node.children {
        flatten_rec(node, &mut buffer)
    }
    CSTNode {
        rule: node.rule,
        start: node.start,
        end: node.end,
        children: buffer,
        node_tag: node.label,
        branch_tag: node.alternative,
    }
}

fn flatten_rec(node: Node, buffer: &mut Vec<CSTNode<Rule>>) {
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
