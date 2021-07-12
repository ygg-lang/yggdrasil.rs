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
            Some("Grammar") => Ok(Self::GrammarStatement(Box::new(ASTNode::named_one(&mut map, "grammar_statement", builder)?))),
            Some("Fragment") => Ok(Self::Fragment(Box::new(ASTNode::named_one(&mut map, "fragment_statement", builder)?))),
            Some("Ignore") => Ok(Self::Ignore(Box::new(ASTNode::named_one(&mut map, "ignore_statement", builder)?))),
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

impl ASTNode<Node> for StringLiteral {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let range = node.get_span();
        let raw = unsafe { (&builder.input).get_unchecked((range.0 + 1)..(range.1 - 1)) };
        let data = unescape(raw)?;
        Ok(Self { data, range })
    }
}

fn unescape(raw: &str) -> Result<String> {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        };
        let c = chars.next().ok_or(Error::node_missing("???"))?;
        match c {
            'b' => out.push('\u{08}'),
            't' => out.push('\t'),
            'n' => out.push('\n'),
            'f' => out.push('\u{0C}'),
            'r' => out.push('\r'),
            'u' => out.push(parse_unicode(&mut chars)?),
            _ => out.push(c),
        }
    }
    return Ok(out);
}

fn parse_unicode<I>(chars: &mut I) -> Result<char>
where
    I: Iterator<Item = char>,
{
    match chars.next() {
        Some('{') => {}
        _ => {
            return Err(Error::node_missing("???"));
        }
    }
    let unicode_seq: String = chars.take_while(|&c| c != '}').collect();
    let n = u32::from_str_radix(&unicode_seq, 16)?;
    Ok(char::from_u32(n).ok_or(Error::node_missing("???"))?)
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
        let data = node.get_str(&builder.input).to_string();
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
