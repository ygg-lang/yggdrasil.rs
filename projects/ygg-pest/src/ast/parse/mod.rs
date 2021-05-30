mod parse_custom;

use super::*;

impl ASTParser for Program {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut statement = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::statement => Statement::try_many(pair, &mut statement, errors),
                _ => continue,
            }
        }
        return Ok(Self { statement, position });
    }
}

impl ASTParser for Statement {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        // let position = get_position(&pairs);
        // let mut statement = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::ignore_statement => {
                    let variant = IgnoreStatement::parse(pair, errors)?;
                    return Ok(Self::IgnoreStatement(Box::new(variant)));
                }
                Rule::assign_statement=>{
                    let variant = AssignStatement::parse(pair, errors)?;
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
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let mut rules = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => Identifier::try_many(pair, &mut rules, errors),
                _ => continue,
            }
        }
        Ok(Self { rules, position })
    }
}

impl ASTParser for AssignStatement {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        //let mut rules = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
               // Rule::SYMBOL => Identifier::try_many(pair, &mut rules, errors),
                _ => continue,
            }
        }
        unreachable!()
        Ok(Self { id: Identifier { data: "".to_string(), position }, eq: "".to_string(), rules, position, rhs: () })
    }
}

impl ASTParser for Identifier {
    fn parse(pairs: Pair<Rule>, _: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let data = pairs.as_str().to_string();
        Ok(Self { data, position })
    }
}

#[test]
fn test1() {
    let mut parser = ASTBuilder::default();
    let out = parser.parse_program("x = a ~ b ~ c | d");
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
