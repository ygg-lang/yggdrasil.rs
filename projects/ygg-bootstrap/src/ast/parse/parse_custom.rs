use super::*;

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

impl ASTNode<Node> for Expression {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let mut map = node.get_tag_map();
        let mut terms = vec![];
        terms.push(ASTNode::named_one(&mut map, "term", builder)?);
        for e in map.remove("expr_next").unwrap_or_default() {
            let mut branch = e.branch_tag;
            let mut map = e.get_tag_map();
            match branch {
                Some("infix") => {
                    let infix = map.remove("infix").unwrap_or_default().remove(0);
                    if let Some(s) = infix.get_str(&builder.input).chars().next() { terms.push(Term::Split(s)) }
                },
                Some("other") => {
                    terms.push(Term::Split(' '))
                },
                _ => {}
            }
            terms.push(ASTNode::named_one(&mut map, "term", builder)?);
        }
        Term::build_expression(terms)

        /*
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
        */
    }
}

impl ASTNode<Node> for Term {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let mut map = node.get_tag_map();

        let data = Data::named_one(&mut map, "data", builder)?;
        let prefix = Prefix::named_many(&mut map, "prefix", builder);
        let suffix = Prefix::named_many(&mut map, "term_next", builder);
        if prefix.len() + suffix.len() == 0 {
            return Ok(Self::Atom(Expression::Data(Box::new(data))))
        }
        println!("{:#?}", prefix);
        println!("{:#?}", suffix);

        unimplemented!();
    }
}



impl Term {
    pub fn build_expression(input: Vec<Term>) -> Result<Expression> {
        unimplemented!()
    }
}