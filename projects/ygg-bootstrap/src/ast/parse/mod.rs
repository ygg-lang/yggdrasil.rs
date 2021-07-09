mod parse_custom;

use super::*;

impl ASTNode<Node> for Program {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let statement = ASTNode::named_many(&mut map, "statement", errors);
        return Ok(Self { statement, range });
    }
}

impl ASTNode<Node> for Statement {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let mut map = node.get_tag_map();
        match node.get_branch_tag() {
            Some("Grammar") => unimplemented!(),
            Some("Fragment") => Ok(Self::Fragment(Box::new(ASTNode::named_one(&mut map, "fragment_statement", errors)?))),
            Some("Ignore") => Ok(Self::Ignore(Box::new(ASTNode::named_one(&mut map, "ignore_statement", errors)?))),
            Some("Assign") => Ok(Self::Assign(Box::new(ASTNode::named_one(&mut map, "assign_statement", errors)?))),
            _ => {
                unreachable!("{:#?}", map);
            }
        }
    }
}

impl ASTNode<Node> for FragmentStatement {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let range = node.get_span();
        let mut map = node.get_tag_map();
        let id = ASTNode::named_one(&mut map, "id", errors)?;
        return Ok(Self { id, range });
    }
}

impl ASTNode<Node> for IgnoreStatement {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(node);
        let mut map = node.get_tag_map();
        let rules = ASTNode::named_many(&mut map, "rules", errors);
        return Ok(Self { rules, range: position });
    }
}

impl ASTNode<Node> for AssignStatement {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(node);
        let mut map = node.get_tag_map();
        let id = ASTNode::named_one(&mut map, "id", errors)?;
        let eq = ASTNode::named_one(&mut map, "eq", errors)?;
        let rhs = ASTNode::named_one(&mut map, "rhs", errors)?;
        Ok(Self { id, eq, rhs, range: position })
    }
}

impl ASTNode<Node> for Expression {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(node);
        let mut map = node.get_tag_map();
        let head = map.remove("__rec_expr_left").as_mut().map(|s| s.remove(0));
        let rest = map.remove("__rec_expr_rest").unwrap_or_default();
        if let Some(s) = head {
            unreachable!()
        };
        match node.get_branch_tag() {
            Some("Priority") => Self::named_one(&mut map, "expr", errors),
            Some("Concat") => {
                let base = ASTNode::named_one(&mut map, "base", errors)?;
                let rest = ASTNode::named_many(&mut map, "rest", errors);
                let resolver = ExpressionResolver { base: Expression::Data(Box::new(base)), rest };
                Ok(Self::Concat(Box::new(resolver.dyn_associative())))
            }
            Some("Mark") => {
                let lhs = ASTNode::named_one(&mut map, "lhs", errors)?;
                let ty = ASTNode::named_some(&mut map, "ty", errors);
                let rhs = ASTNode::named_one(&mut map, "rhs", errors)?;
                Ok(Self::Mark(Box::new(MarkExpression { lhs, ty, rhs, range: position })))
            }
            Some("Choice") => {
                let lhs = ASTNode::named_one(&mut map, "lhs", errors)?;
                let lhs_tag = ASTNode::named_some(&mut map, "lhs_tag", errors);
                let lhs_ty = ASTNode::named_some(&mut map, "lhs_ty", errors);
                let rhs = ASTNode::named_one(&mut map, "rhs", errors)?;
                let rhs_tag = ASTNode::named_some(&mut map, "rhs_tag", errors);
                let rhs_ty = ASTNode::named_some(&mut map, "rhs_ty", errors);
                Ok(Self::Choice(Box::new(ChoiceExpression {
                    lhs,
                    lhs_tag,
                    lhs_ty,
                    rhs,
                    rhs_tag,
                    rhs_ty,
                    range: position,
                })))
            }
            Some("Suffix") => {
                unimplemented!("{:#?}", map);
            }
            Some("Data") => Ok(Self::Data(Box::new(ASTNode::named_one(&mut map, "data", errors)?))),
            Some(s) => {
                unreachable!("Some({:#?})=>{{}}", s);
            }
            _ => return Err(Error::node_missing("Expression")),
        }
    }
}

impl ASTNode<Node> for ConcatExpressionRest {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(node);
        let mut map = node.get_tag_map();
        let expr = ASTNode::named_one(&mut map, "expr", errors)?;
        Ok(Self { expr, position })
    }
}

impl ASTNode<Node> for Data {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let mut map = node.get_tag_map();
        match node.get_branch_tag() {
            Some("SymbolPath") => Ok(Self::SymbolPath(Box::new(ASTNode::named_one(&mut map, "symbol_path", errors)?))),
            Some("Integer") => Ok(Self::Integer(Box::new(ASTNode::named_one(&mut map, "integer", errors)?))),
            Some(s) => {
                unreachable!("{:#?}", s);
            }
            _ => return Err(Error::node_missing("Data")),
        }
    }
}

impl ASTNode<Node> for Integer {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&node);
        let data = ASTNode::parse(node, errors)?;
        Ok(Self { data, range: position })
    }
}

impl ASTNode<Node> for SymbolPath {
    fn parse(node: Node, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(node);
        let mut map = node.get_tag_map();
        let data = ASTNode::named_many(&mut map, "symbol", errors);
        Ok(Self { symbol: data, range: position })
    }
}

impl ASTNode<Node> for Symbol {
    fn parse(node: Node, _: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(node);
        let data = pairs.as_str().to_string();
        Ok(Self { data, range: position })
    }
}

#[test]
fn test1() {
    let mut parser = ASTBuilder::default();
    let out = parser.parse_program("x = a ~ b ~ c | d #D :T");
    println!("{:#?}", out.unwrap());
    println!("{:#?}", parser.errors);
}

#[test]
fn test() {
    let mut parser = ASTBuilder::default();
    let out = parser.parse_program(include_str!("bootstrap.ygg"));
    println!("{:#?}", out.unwrap());
    println!("{:#?}", parser.errors);
}
