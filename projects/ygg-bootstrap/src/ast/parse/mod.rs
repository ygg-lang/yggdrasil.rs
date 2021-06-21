mod parse_custom;

use super::*;

impl ASTParser for Program {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let statement = ASTParser::named_many(&mut map, "statement", errors);
        return Ok(Self { statement, position });
    }
}

impl ASTParser for Statement {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let mut map = collect_tag_map(&pairs);
        match pairs.as_branch_tag() {
            Some("Grammar") => unimplemented!(),
            Some("Fragment") => Ok(Self::Fragment(Box::new(ASTParser::try_named_one(&mut map, "fragment_statement", errors)?))),
            Some("Ignore") => Ok(Self::Ignore(Box::new(ASTParser::try_named_one(&mut map, "ignore_statement", errors)?))),
            Some("Assign") => Ok(Self::Assign(Box::new(ASTParser::try_named_one(&mut map, "assign_statement", errors)?))),
            _ => {
                unreachable!("{:#?}", map);
            }
        }
    }
}

impl ASTParser for FragmentStatement {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let id = ASTParser::try_named_one(&mut map, "id", errors)?;
        return Ok(Self { id, position });
    }
}

impl ASTParser for IgnoreStatement {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let rules = ASTParser::named_many(&mut map, "rules", errors);
        return Ok(Self { rules, position });
    }
}

impl ASTParser for AssignStatement {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let id = ASTParser::try_named_one(&mut map, "id", errors)?;
        let eq = ASTParser::try_named_one(&mut map, "eq", errors)?;
        let rhs = ASTParser::try_named_one(&mut map, "rhs", errors)?;
        Ok(Self { id, eq, rhs, position })
    }
}

impl ASTParser for Expression {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);

        let left = ASTParser::try_named_one(&mut map, "__rec_expr_left", errors)?;
        let rest = ASTParser::named_many(&mut map, "__rec_expr_rest", errors);
        let resolver = ExpressionResolver { base: Expression::Data(Box::new(base)), rest };


        match pairs.as_branch_tag() {
            Some("Priority") => Self::try_named_one(&mut map, "expr", errors),
            Some("Concat") => {
                let base = ASTParser::try_named_one(&mut map, "base", errors)?;
                let rest = ASTParser::named_many(&mut map, "rest", errors);
                let resolver = ExpressionResolver { base: Expression::Data(Box::new(base)), rest };
                Ok(Self::Concat(Box::new(resolver.dyn_associative())))
            }
            Some("Mark") => {
                let lhs = ASTParser::try_named_one(&mut map, "lhs", errors)?;
                let ty = ASTParser::named_some(&mut map, "ty", errors);
                let rhs = ASTParser::try_named_one(&mut map, "rhs", errors)?;
                Ok(Self::Mark(Box::new(MarkExpression { lhs, ty, rhs, position })))
            }
            Some("Choice") => {
                let lhs = ASTParser::try_named_one(&mut map, "lhs", errors)?;
                let lhs_tag = ASTParser::named_some(&mut map, "lhs_tag", errors);
                let lhs_ty = ASTParser::named_some(&mut map, "lhs_ty", errors);
                let rhs = ASTParser::try_named_one(&mut map, "rhs", errors)?;
                let rhs_tag = ASTParser::named_some(&mut map, "rhs_tag", errors);
                let rhs_ty = ASTParser::named_some(&mut map, "rhs_ty", errors);
                Ok(Self::Choice(Box::new(ChoiceExpression { lhs, lhs_tag, lhs_ty, rhs, rhs_tag, rhs_ty, position })))
            }
            Some("Suffix") => {
                unimplemented!("{:#?}", map);
            }
            Some("Data") => Ok(Self::Data(Box::new(ASTParser::try_named_one(&mut map, "data", errors)?))),
            Some(s) => {
                unreachable!("Some({:#?})=>{{}}", s);
            }
            _ => return Err(Error::node_missing("Expression")),
        }
    }
}

impl ASTParser for ConcatExpressionRest {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let expr = ASTParser::try_named_one(&mut map, "expr", errors)?;
        Ok(Self { expr, position })
    }
}

impl ASTParser for Data {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let mut map = collect_tag_map(&pairs);
        match pairs.as_branch_tag() {
            Some("SymbolPath") => Ok(Self::SymbolPath(Box::new(ASTParser::try_named_one(&mut map, "symbol_path", errors)?))),
            Some("Integer") => Ok(Self::Integer(Box::new(ASTParser::try_named_one(&mut map, "integer", errors)?))),
            Some(s) => {
                unreachable!("{:#?}", s);
            }
            _ => return Err(Error::node_missing("Data")),
        }
    }
}

impl ASTParser for Integer {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let data = ASTParser::parse_pair(pairs, errors)?;
        Ok(Self { data, position })
    }
}

impl ASTParser for SymbolPath {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        let data = ASTParser::named_many(&mut map, "symbol", errors);
        Ok(SymbolPath { symbol: data, position })
    }
}

impl ASTParser for Symbol {
    fn parse_pair(pairs: Pair<Rule>, _: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let data = pairs.as_str().to_string();
        Ok(Self { data, position })
    }
}

///---------------------------------------------------------------------------------------------------

impl ASTParser for isize {
    fn parse_pair(pairs: Pair<Rule>, _: &mut Vec<Error>) -> Result<Self> {
        Ok(pairs.as_str().parse::<isize>()?)
    }
}

impl ASTParser for String {
    fn parse_pair(pairs: Pair<Rule>, _: &mut Vec<Error>) -> Result<Self> {
        Ok(pairs.as_str().to_string())
    }
}

#[test]
fn test1() {
    let mut parser = ASTBuilder::default();
    let out = parser.parse_program("x = ((a ~ 0) ~ c )~ 1");
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

fn collect_tag_map<'a>(pairs: &'a Pair<Rule>) -> HashMap<String, Vec<Pair<'a, Rule>>> {
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
