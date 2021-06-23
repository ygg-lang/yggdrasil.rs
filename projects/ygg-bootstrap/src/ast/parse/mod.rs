mod parse_custom;

use super::*;

impl<'i> ASTNode<Pair<'i, Rule>> for Program {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let statement = ASTNode::named_many(&mut map, "statement", errors);
        return Ok(Self { statement, range: position });
    }
}

impl<'i> ASTNode<Pair<'i, Rule>> for Statement {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let mut map = collect_tag_map(&pairs);
        match pairs.as_branch_tag() {
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

impl<'i> ASTNode<Pair<'i, Rule>> for FragmentStatement {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let id = ASTNode::named_one(&mut map, "id", errors)?;
        return Ok(Self { id, range: position });
    }
}

impl<'i> ASTNode<Pair<'i, Rule>> for IgnoreStatement {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let rules = ASTNode::named_many(&mut map, "rules", errors);
        return Ok(Self { rules, range: position });
    }
}

impl<'i> ASTNode<Pair<'i, Rule>> for AssignStatement {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let id = ASTNode::named_one(&mut map, "id", errors)?;
        let eq = ASTNode::named_one(&mut map, "eq", errors)?;
        let rhs = ASTNode::named_one(&mut map, "rhs", errors)?;
        Ok(Self { id, eq, rhs, range: position })
    }
}

impl<'i> ASTNode<Pair<'i, Rule>> for Expression {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let head = map.remove("__rec_expr_left").as_mut().map(|s| s.remove(0));
        let rest = map.remove("__rec_expr_rest").unwrap_or_default();
        if let Some(s) = head {
            ExpressionResolver::build(s, rest, errors)?;
            unreachable!()
        };
        match pairs.as_branch_tag() {
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
                Ok(Self::Choice(Box::new(ChoiceExpression { lhs, lhs_tag, lhs_ty, rhs, rhs_tag, rhs_ty, range: position })))
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

impl<'i> ASTNode<Pair<'i, Rule>> for ConcatExpressionRest {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let expr = ASTNode::named_one(&mut map, "expr", errors)?;
        Ok(Self { expr, position })
    }
}

impl<'i> ASTNode<Pair<'i, Rule>> for Data {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let mut map = collect_tag_map(&pairs);
        match pairs.as_branch_tag() {
            Some("SymbolPath") => Ok(Self::SymbolPath(Box::new(ASTNode::named_one(&mut map, "symbol_path", errors)?))),
            Some("Integer") => Ok(Self::Integer(Box::new(ASTNode::named_one(&mut map, "integer", errors)?))),
            Some(s) => {
                unreachable!("{:#?}", s);
            }
            _ => return Err(Error::node_missing("Data")),
        }
    }
}

impl<'i> ASTNode<Pair<'i, Rule>> for Integer {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let data = ASTNode::parse(pairs, errors)?;
        Ok(Self { data, range: position })
    }
}

impl<'i> ASTNode<Pair<'i, Rule>> for SymbolPath {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let data = ASTNode::named_many(&mut map, "symbol", errors);
        Ok(Self { symbol: data, range: position })
    }
}

impl<'i> ASTNode<Pair<'i, Rule>> for Symbol {
    fn parse(pairs: Pair<Rule>, _: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
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

//region utility functions

fn collect_tag_map<'a>(pairs:  Pair<'a, Rule>) -> HashMap<String, Vec<Pair<'a, Rule>>> {
    let mut out: HashMap<String, Vec<Pair<Rule>>> = HashMap::new();
    for pair in pairs.clone().into_inner() {
        if let Some(s) = pair.as_node_tag() {
            match out.get_mut(s) {
                Some(s) => s.push(pair),
                None => {
                    out.insert(s.to_string(), vec![pair]);
                }
            }
        }
    }
    return out;
}

//endregion
