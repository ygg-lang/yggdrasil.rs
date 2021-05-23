
use super::*;

pub fn mark_program(pairs: Pairs<Rule>) -> RuleResult<CSTNode> {
    let mut node = CSTNode {
        text: pairs.as_str(),
        mark: None,
        position: OffsetRange { start: 0, end: 0 },
        children: vec![],
    };
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::WHITESPACE => self::split_whitespace(&mut node.children, pair)?,
            Rule::statement => node.children.push(self::mark_statement(pair, Some("statement"))?),
            _ => {
                println!("mark_program");
                println!("{:#?}", pair)
            }
        }
    }
    return Ok(node);
}

pub fn mark_statement<'i>(pairs: Pair<'i,Rule>, mark: Option<&'static str>) -> RuleResult<CSTNode<'i>> {
    let position = get_position(&pairs);
    let mut node = CSTNode {
        text: pairs.as_str(),
        mark,
        position,
        children: vec![],
    };
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::WHITESPACE => self::split_whitespace(&mut node.children, pair)?,
            _ => {
                println!("mark_statement");
                println!("{:#?}", pair)
            }
        }
    }
    return Ok(node);
}

pub fn split_whitespace<'i>(nodes: &mut Vec<CSTNode<'i>>, pairs: Pair<'i,Rule>) ->  RuleResult<()> {
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::COMMENT|Rule::WHITE_SPACE|Rule::NEWLINE => {nodes.push(self::atomic(pair, None)?)}
            _ => unreachable!()
        }
    }
    Ok(())
}

pub fn atomic<'i>(pairs: Pair<'i, Rule>, mark: Option<&'static str>) ->  RuleResult<CSTNode<'i>>  {
    let position = get_position(&pairs);
    let text = pairs.as_str();
    Ok(CSTNode {
        text,
        mark,
        position,
        children: vec![],
    })
}