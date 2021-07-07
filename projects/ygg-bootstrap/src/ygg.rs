use std::env;

include!(concat!(env!("OUT_DIR"), "/ygg.rs"));

pub use ygg::{Node, Rule, PEG};

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
        Rule::IGNORE|Rule::Terminal|Rule::MustNotMatch => {}
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
