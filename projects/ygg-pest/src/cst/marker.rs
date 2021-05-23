use super::*;

pub fn mark_program(pairs: Pairs<Rule>) -> RuleResult<CSTNode> {
    let mut node = CSTNode {
        text: pairs.as_str(),
        mark: None,
        range: OffsetRange { start: 0, end: 0 },
        children: vec![],
    };
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::WHITESPACE => self::split_whitespace(&mut node.children, pair)?,
            _ => {
                println!("{:#?}", pair)
            }
        }
    }
    return Ok(node);
}

pub fn split_whitespace<'a>(nodes: &mut Vec<CSTNode<'a>>, pairs: Pair<'a, Rule>) ->  RuleResult<()> {
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::COMMENT|Rule::WHITE_SPACE|Rule::NEWLINE => {nodes.push(self::atomic(pair)?)}
            _ => unreachable!()
        }
    }
    Ok(())
}

pub fn atomic(pairs: Pair<Rule>) ->  RuleResult<CSTNode>  {
    let span = pairs.as_span();
    let range = pairs.into();
    Ok(CSTNode {
        text: span.as_str(),
        mark: None,
        range,
        children: vec![],
    })
}