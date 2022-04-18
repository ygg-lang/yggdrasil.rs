use super::*;
use std::ops::Add;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub base: ExpressionNode,
    pub ops: Vec<Operator>,
}

impl From<UnaryExpression> for ExpressionKind {
    fn from(e: UnaryExpression) -> Self {
        Self::Unary(Box::new(e))
    }
}

impl Add<Operator> for ExpressionNode {
    type Output = Self;

    fn add(self, o: Operator) -> Self::Output {
        match self.kind {
            ExpressionKind::Unary(node) => {
                let mut ops = node.ops;
                ops.push(o);
                let unary = UnaryExpression { base: node.base, ops };
                ExpressionNode { kind: ExpressionKind::Unary(Box::new(unary)), branch_tag: self.branch_tag, node_tag: self.node_tag }
            }
            _ => {
                let unary = UnaryExpression { base: self, ops: vec![o] };
                ExpressionNode { kind: ExpressionKind::Unary(Box::new(unary)), branch_tag: "".to_string(), node_tag: "".to_string() }
            }
        }
    }
}

impl Operator {
    pub fn prefix(o: &str) -> Operator {
        match o {
            "*" => Self::Recursive,
            "^" => Self::Remark,
            _ => unreachable!(),
        }
    }
    pub fn suffix(o: &str) -> Operator {
        match o {
            "?" => Self::Optional,
            "+" => Self::Repeats,
            "*" => Self::Repeat1,
            _ => unreachable!(),
        }
    }
}

// impl Expression {
//     pub fn prefix(e: Term, o: &str) -> Self {
//         let mut node = Self::from(e).as_unary();
//         node.get_unary_mut().map(|e| e.ops.push(Operator::prefix(o)));
//         return node;
//     }
//     pub fn suffix(e: Term, o: &str) -> Self {
//         let mut node = Self::from(e).as_unary();
//         node.get_unary_mut().map(|e| e.ops.push(Operator::suffix(o)));
//         return node;
//     }
//     fn as_unary(self) -> Self {
//         if let Some(_) = self.get_unary() {
//             return self;
//         }
//         return Self { inline_token: false, ty: None, branch_tag: None, node_tag: None, node: Term::unary(self) };
//     }
//     pub fn mark_node(name: Term, node: Term) -> Self {
//         let mut node = Self::from(node);
//         node.node_tag = name.as_symbol();
//         return node;
//     }
//     pub fn mark_branch(base: Term, kind: Option<char>, name: Symbol) -> Self {
//         let mut node = Self::from(base);
//         node.branch_tag = Some(ExpressionTag::new(kind, name));
//         return node;
//     }
// }
//
// impl ExpressionTag {
//     pub fn new(kind: Option<char>, name: Symbol) -> Self {
//         let mode = match kind {
//             None => "".to_string(),
//             Some(_) => {
//                 unimplemented!()
//             }
//         };
//         Self { name, mode }
//     }
// }
//
// impl Term {
//     pub fn unary(base: Expression) -> Self {
//         Self::Unary(Box::new(UnaryExpression { base, ops: vec![] }))
//     }
// }
