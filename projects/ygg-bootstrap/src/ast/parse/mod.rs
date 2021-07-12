mod parse_custom;

use super::*;
use crate::cst::Rule;
use yggdrasil_shared::records::CSTNode;

pub type Node = CSTNode<Rule>;

impl ASTNode<Node> for Program {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let statement = ASTNode::named_many(&mut map, "statement", builder);
        return Ok(Self { statement, range });
    }
}

impl ASTNode<Node> for Statement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let branch = node.branch_tag;
        let mut map = node.get_tag_map();
        match branch {
            Some("Grammar") => unimplemented!(),
            Some("Fragment") => Ok(Self::Fragment(Box::new(ASTNode::named_one(&mut map, "fragment_statement", builder)?))),
            Some("Ignore") => Ok(Self::Ignore(Box::new(ASTNode::named_one(&mut map, "ignore_statement", builder)?))),
            Some("Assign") => Ok(Self::Assign(Box::new(ASTNode::named_one(&mut map, "assign_statement", builder)?))),
            _ => {
                unreachable!("{:#?}", map);
            }
        }
    }
}

impl ASTNode<Node> for FragmentStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let id = ASTNode::named_one(&mut map, "id", builder)?;
        return Ok(Self { id, range });
    }
}

impl ASTNode<Node> for IgnoreStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let rules = ASTNode::named_many(&mut map, "rules", builder);
        return Ok(Self { rules, range });
    }
}

impl ASTNode<Node> for AssignStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let id = ASTNode::named_one(&mut map, "id", builder)?;
        let eq = ASTNode::named_one(&mut map, "eq", builder)?;
        let rhs = ASTNode::named_one(&mut map, "rhs", builder)?;
        Ok(Self { id, eq, rhs, range })
    }
}

impl ASTNode<Node> for Expression {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let branch = node.branch_tag;
        let mut map = node.get_tag_map();
        let head = map.remove("__rec_expr_left").as_mut().map(|s| s.remove(0));
        match branch {
            Some("Priority") => Self::named_one(&mut map, "expr", builder),
            Some("Concat") => {
                unimplemented!()
            }
            Some("Mark") => {
                let lhs = ASTNode::named_one(&mut map, "lhs", builder)?;
                let ty = ASTNode::named_some(&mut map, "ty", builder);
                let rhs = ASTNode::named_one(&mut map, "rhs", builder)?;
                Ok(Self::Mark(Box::new(MarkExpression { lhs, ty, rhs, range })))
            }
            Some("Choice") => {
                unimplemented!()
            }
            Some("Suffix") => {
                unimplemented!("{:#?}", map);
            }
            Some("Data") => Ok(Self::Data(Box::new(ASTNode::named_one(&mut map, "data", builder)?))),
            Some(s) => {
                unreachable!("Some({:#?})=>{{}}", s);
            }
            _ => return Err(Error::node_missing("Expression")),
        }
    }
}

impl ASTNode<Node> for Data {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let branch = node.branch_tag;
        let mut map = node.get_tag_map();
        match branch {
            Some("SymbolPath") => Ok(Self::SymbolPath(Box::new(ASTNode::named_one(&mut map, "symbol_path", builder)?))),
            Some("Integer") => Ok(Self::Integer(Box::new(ASTNode::named_one(&mut map, "integer", builder)?))),
            Some(s) => {
                unreachable!("{:#?}", s);
            }
            _ => return Err(Error::node_missing("Data")),
        }
    }
}

impl ASTNode<Node> for Integer {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let data = ASTNode::parse(node, builder)?;
        Ok(Self { data, range })
    }
}

impl ASTNode<Node> for SymbolPath {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let data = ASTNode::named_many(&mut map, "symbol", builder);
        Ok(Self { symbol: data, range })
    }
}

impl ASTNode<Node> for Symbol {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let data = node.get_string(&builder.input);
        Ok(Self { data, range })
    }
}

#[test]
fn test1() {
    let mut parser = ASTBuilder::default();
    let out = parser.parse_program("x = a ~ b ~ c | d #D :T");
    println!("{:#?}", out.unwrap());
    println!("{:#?}", parser.builder);
}

#[test]
fn test() {
    let mut parser = ASTBuilder::default();
    let out = parser.parse_program(include_str!("bootstrap.ygg"));
    println!("{:#?}", out.unwrap());
    println!("{:#?}", parser.builder);
}
