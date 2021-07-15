mod parse_custom;

use super::*;
use crate::cst::Rule;
use yggdrasil_shared::records::CSTNode;

type Node = CSTNode<Rule>;

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
            Some("Grammar") => Ok(Self::Grammar(Box::new(ASTNode::named_one(&mut map, "grammar_statement", builder)?))),
            Some("Fragment") => Ok(Self::Fragment(Box::new(ASTNode::named_one(&mut map, "fragment_statement", builder)?))),
            Some("Ignore") => Ok(Self::Ignore(Box::new(ASTNode::named_one(&mut map, "ignore_statement", builder)?))),
            Some("Import") => Ok(Self::Import(Box::new(ASTNode::named_one(&mut map, "import_statement", builder)?))),
            Some("Assign") => Ok(Self::Assign(Box::new(ASTNode::named_one(&mut map, "assign_statement", builder)?))),
            _ => {
                unreachable!("{:#?}", map);
            }
        }
    }
}

impl ASTNode<Node> for GrammarStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let id = ASTNode::named_one(&mut map, "id", builder)?;
        let ext = ASTNode::named_many(&mut map, "ext", builder);
        return Ok(Self { id, ext, range });
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

impl ASTNode<Node> for ImportStatement {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let path = ASTNode::named_one(&mut map, "path", builder)?;
        let symbol_alias = ASTNode::named_many(&mut map, "symbol_alias", builder);
        return Ok(Self { path, symbol_alias, range });
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
        let ty = ASTNode::named_some(&mut map, "ty", builder);
        let eq = ASTNode::named_one(&mut map, "eq", builder)?;
        let rhs = ASTNode::named_one(&mut map, "rhs", builder)?;
        Ok(Self { id, ty, eq, rhs, range })
    }
}

impl ASTNode<Node> for Data {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        println!("{:#?}", node);
        let branch = node.branch_tag;
        let mut map = node.get_tag_map();

        match branch {
            Some("Symbol") => Ok(Self::Symbol(Box::new(ASTNode::named_one(&mut map, "symbol_path", builder)?))),
            Some("Integer") => Ok(Self::Integer(Box::new(ASTNode::named_one(&mut map, "integer", builder)?))),
            Some("String") => Ok(Self::String(Box::new(ASTNode::named_one(&mut map, "string", builder)?))),
            Some(s) => {
                unreachable!("{:#?}", s);
            }
            _ => return Err(Error::node_missing("Data")),
        }
    }
}

macro_rules! string_node {
    ($node:ty, $kind:ty) => {
        impl ASTNode<$node> for $kind {
            fn parse(node: $node, builder: &mut ASTBuilder) -> Result<Self> {
                let range = node.get_span();
                let data = ASTNode::parse(node, builder)?;
                Ok(Self { data, range })
            }
        }
    };
}




impl ASTNode<Node> for SymbolPath {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let data = ASTNode::named_many(&mut map, "symbol", builder);
        Ok(Self { symbol: data, range })
    }
}

impl ASTNode<Node> for SymbolAlias {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let from = ASTNode::named_one(&mut map, "from", builder)?;
        let into = ASTNode::named_some(&mut map, "into", builder);
        Ok(Self { from, into, range })
    }
}

string_node!(Node,Prefix);
string_node!(Node,Suffix);
string_node!(Node,Symbol);
string_node!(Node,Integer);

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
