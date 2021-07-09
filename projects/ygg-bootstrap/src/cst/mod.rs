#![allow(non_snake_case, non_camel_case_types)]
#![allow(unused_variables, dead_code)]

// #[rustfmt::skip]
mod parse;

pub use self::parse::{Node, Rule, PEG};
use std::fmt::{Debug, Formatter};
use yggdrasil_shared::Result;

pub struct CSTBuilder {
    peg: PEG,
}

impl Default for CSTBuilder {
    fn default() -> Self {
        Self { peg: PEG::new() }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("Node");
        w.field("rule", &self.rule);
        w.field("range", &format!("{}-{}", self.start, self.end));
        if let Some(s) = self.label {
            w.field("label", &s);
        };
        if let Some(s) = self.alternative {
            w.field("branch", &s);
        };
        w.field("children", &self.children);
        w.finish()
    }
}

impl Debug for CSTBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("CSTBuilder");
        w.field("input", &self.input);
        w.finish()
    }
}

impl CSTBuilder {
    pub fn parse(&mut self, input: &str) -> Result<Node> {
        let p = &mut self.peg;
        let out = flatten(p.parse(&input).unwrap());
        return Ok(out);
    }
}

fn flatten(node: Node) -> Node {
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

fn flatten_rec(node: Node, buffer: &mut Vec<Node>) {
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
