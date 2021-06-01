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
                    let variant = IgnoreStatement::parse(pair, errors)?;
                    return Ok(Self::Ignore(Box::new(variant)));
                }
                Rule::assign_statement => {
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
        let mut id: Option<Identifier> = None;
        let mut eq: Option<String> = None;
        let mut rhs: Option<Expression> = None;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => Identifier::try_one(pair, &mut id, errors)?,
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
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => {}
                Rule::data => {}
                Rule::expr => {}
                _ => {
                    unreachable!("Rule::{:#?}=>{{}}", pair.as_rule());
                }
            }
        }
        unreachable!()
    }
}

impl ASTParser for Data {
    fn parse(pairs: Pair<Rule>, errors: &mut Vec<Error>) -> Result<Self> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                _ => {
                    unreachable!("Rule::{:#?}=>{{}}", pair.as_rule());
                }
            }
        }
        unreachable!()
    }
}

impl ASTParser for Identifier {
    fn parse(pairs: Pair<Rule>, _: &mut Vec<Error>) -> Result<Self> {
        let position = get_position(&pairs);
        let data = pairs.as_str().to_string();
        Ok(Self { data, position })
    }
}

impl ASTParser for String {
    fn parse(pairs: Pair<Rule>, _: &mut Vec<Error>) -> Result<Self> {
        Ok(pairs.as_str().to_string())
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
