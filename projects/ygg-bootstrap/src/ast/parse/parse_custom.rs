use super::*;
use yggdrasil_shared::traits::{Affix, Associativity, PrattParser};

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
            let branch = e.branch_tag;
            let mut map = e.get_tag_map();
            match branch {
                Some("Infix") => {
                    let infix = map.remove("infix").unwrap_or_default().remove(0);
                    if let Some(s) = infix.get_str(&builder.input).chars().next() {
                        terms.push(Term::Infix(s))
                    }
                }
                Some("Other") => terms.push(Term::Infix(' ')),
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
        // println!("{:#?}", data);
        // println!("{:#?}", prefix);
        // println!("{:#?}", suffix);

        return Ok(Term::Atom(Expression::Data(data)))
    }
}

impl Term {
    pub fn build_expression(input: Vec<Term>) -> Result<Expression> {

        TermResolve.parse(&mut input.into_iter())
    }
}

pub struct TermResolve;

impl<I> PrattParser<I> for TermResolve
where
    I: Iterator<Item = Term>,
{
    type Input = Term;
    type Output = Expression;

    fn query(&mut self, term: &Self::Input) -> Result<Affix> {
        let affix = match term {
            Term::Suffix('?') => Affix::suffix(300),
            Term::Suffix('+') => Affix::suffix(300),
            Term::Suffix('*') => Affix::suffix(300),

            Term::Prefix('!') => Affix::prefix(200),
            Term::Prefix('&') => Affix::prefix(200),
            Term::Prefix('^') => Affix::prefix(200),
            Term::Prefix('%') => Affix::prefix(200),

            Term::Infix(' ') => Affix::infix(100, Associativity::Left),
            Term::Infix('~') => Affix::infix(90, Associativity::Left),
            Term::Infix('|') => Affix::infix(80, Associativity::Left),
            Term::Infix('/') => Affix::infix(80, Associativity::Left),
            Term::Infix(':') => Affix::infix(70, Associativity::Left),
            Term::Infix('<') => Affix::infix(60, Associativity::Left),

            Term::Atom(_) => Affix::None,
            _ => unreachable!(),
        };
        Ok(affix)
    }

    fn primary(&mut self, term: Self::Input) -> Result<Self::Output> {
        match term {
            Term::Atom(e) => Ok(e),
            _ => unreachable!(),
        }
    }

    fn infix(&mut self, lhs: Self::Output, term: Self::Input, rhs: Self::Output) -> Result<Self::Output> {
        let out = match term {
            Term::Infix(' ') => Expression::Concat { is_soft: false, lhs: Box::new(lhs), rhs:Box::new(rhs) },
            Term::Infix('~') => Expression::Concat { is_soft: true, lhs: Box::new(lhs), rhs:Box::new(rhs) },
            Term::Infix('|') => Expression::Choice { lhs: Box::new(lhs), rhs:Box::new(rhs) },
            Term::Infix('/') => Expression::Choice { lhs: Box::new(lhs), rhs:Box::new(rhs) },
            Term::Infix(':') => Expression::MarkType { lhs: Box::new(lhs), rhs:Box::new(rhs) },
            Term::Infix('<') => Expression::MarkNode { lhs: Box::new(lhs), rhs:Box::new(rhs) },
            _ => unreachable!(),
        };
        return Ok(out);
    }

    fn prefix(&mut self, term: Self::Input, rhs: Self::Output) -> Result<Self::Output> {
        let out = match term {
            Term::Prefix('!') => Expression::MustNot(Box::new(rhs)),
            Term::Prefix('&') => Expression::MustOne (Box::new(rhs)),
            Term::Prefix('^') => Expression::MarkNodeShort(Box::new(rhs)),
            Term::Prefix('%') => unimplemented!(),
            _ => unreachable!(),
        };
        Ok(out)
    }

    // Construct a unary postfix expression, e.g. 1?
    fn suffix(&mut self, lhs: Self::Output, tree: Self::Input) -> Result<Self::Output> {
        let out = match tree {
            Term::Suffix('?') => Expression::Maybe(Box::new(lhs)),
            Term::Suffix('+') => Expression::Maybe (Box::new(lhs)),
            Term::Suffix('*') => Expression::Maybe (Box::new(lhs)),
            _ => unreachable!(),
        };
        Ok(out)
    }
}

impl ASTNode<Node> for Data {
    fn parse(node: Node, builder: &mut ASTBuilder) -> Result<Self> {
        let branch = node.branch_tag;
        let mut children = node.children;
        let node = children.remove(0);
        match branch {
            Some("Symbol") => Ok(Self::Symbol(Box::new(ASTNode::one(node, builder)?))),
            Some("Integer") => Ok(Self::Integer(Box::new(ASTNode::one(node, builder)?))),
            Some("String") => Ok(Self::String(Box::new(ASTNode::one(node, builder)?))),
            Some(s) => {
                unreachable!("{:#?}", s);
            }
            _ => return Err(Error::node_missing("Data")),
        }
    }
}