mod parse_custom;

use super::*;
use std::collections::HashMap;

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
        // let position = get_position(&pairs);
        // let mut statement = vec![];
        // match pairs.as_branch_tag() {
        //     Some("Ignore") => {
        //         let variant = IgnoreStatement::parse(pair, errors)?;
        //         return Ok(Self::Ignore(Box::new(variant)));
        //     },
        //     _ => Err(Error::node_missing("statement"))
        // }

        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::ignore_statement => {
                    let variant = IgnoreStatement::parse_pair(pair, errors)?;
                    return Ok(Self::Ignore(Box::new(variant)));
                }
                Rule::assign_statement => {
                    let variant = AssignStatement::parse_pair(pair, errors)?;
                    return Ok(Self::AssignStatement(Box::new(variant)));
                }
                _ => {
                    unreachable!("Rule::{:#?}=>{{}}", pair.as_rule());
                }
            }
        }
        unreachable!()
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
        let mut id: Option<Symbol> = None;
        let mut eq: Option<String> = None;
        let mut rhs: Option<Expression> = None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => Symbol::try_one(pair, &mut id, errors)?,
                Rule::assign_kind => String::try_one(pair, &mut eq, errors)?,
                Rule::expr => Expression::try_one(pair, &mut rhs, errors)?,
                _ => continue,
            }
        }
        let id = id.ok_or(Error::node_missing("id"))?;
        let eq = eq.ok_or(Error::node_missing("eq"))?;
        let rhs = rhs.ok_or(Error::node_missing("eq"))?;
        Ok(Self { id, eq, rhs, position })
    }
}

impl ASTParser for Expression {
    fn parse_pair(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut map = collect_tag_map(&pairs);
        match pairs.as_branch_tag() {
            Some("Concat") => {
                let lhs = Expression::Data(Box::new(Data::try_named_one(&mut map, "lhs", errors)?));
                let rhs = ASTParser::try_named_one(&mut map, "rhs", errors)?;
                Ok(Self::ConcatExpression(Box::new(ConcatExpression { lhs, rhs, position })))
            }
            Some("Data") => Ok(Self::Data(Box::new(ASTParser::try_named_one(&mut map, "data", errors)?))),
            Some(s) => {
                unreachable!("{:#?}", s);
            }
            _ => return Err(Error::node_missing("Expression")),
        }
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
    let out = parser.parse_program("x = a ~ 0");
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
