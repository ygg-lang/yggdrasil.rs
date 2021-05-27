mod parse_custom;

use super::*;
use yggdrasil_cst_shared::position_system::get_position;

pub fn program(pairs: Pair<Rule>) -> ASTResult<Program> {
    let mut statement = vec![];
    let position = get_position(&pairs);

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::EOI => continue,
            Rule::statement => statement.push(self::statement(pair)?),
            _ => {
                unreachable!("{:#?}", pair);
                Self::unreachable(&pair)?
            }
        }
    }
    return Ok(Program { statement, position });
}

pub fn statement(&pairs: Pair<Rule>) -> ASTResult<Statement> {
    let mut node = Self::new_node(&pairs, mark);
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::EOI => continue,
            Rule::statement => node.children.push(self.mark_statement(pair, Some("statement"))?),
            _ => {
                unreachable!("{:#?}", pair);
                Self::unreachable(&pair)?
            }
        }
    }
    return Ok(node);
}
