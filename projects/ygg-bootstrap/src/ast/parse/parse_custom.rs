use yggdrasil_shared::traits::{Affix, Associativity, PrattParser, Precedence};
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
        I: Iterator<Item=char>,
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
                Some("infix") => {
                    let infix = map.remove("infix").unwrap_or_default().remove(0);
                    if let Some(s) = infix.get_str(&builder.input).chars().next() {
                        terms.push(Term::Split(s))
                    }
                }
                Some("other") => terms.push(Term::Split(' ')),
                _ => {}
            }
            terms.push(ASTNode::named_one(&mut map, "term", builder)?);
        }
        Term::build_expression(&mut terms)

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
            return Ok(Self::Atom(Expression::Data(Box::new(data))));
        }
        println!("{:#?}", prefix);
        println!("{:#?}", suffix);

        unimplemented!();
    }
}

impl Term {
    pub fn build_expression(input: &mut Vec<Term>) -> Result<Expression> {
        if let 1 = input.len() {
            if let Term::Atom(s) = input.remove(0) {
                return Ok(s);
            }
        }
        unimplemented!()
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



struct ExprParser;

// From this
#[derive(Debug)]
pub enum TokenTree {
    Prefix(char),
    Postfix(char),
    Infix(char),
    Primary(i32),
    Group(Vec<TokenTree>),
}

impl<I> PrattParser<I> for ExprParser
    where
        I: Iterator<Item = TokenTree>,
{
    type Error = pratt::NoError;
    type Input = TokenTree;
    type Output = Expr;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &TokenTree) -> Result<Affix> {
        let affix = match tree {
            TokenTree::Infix('=') => Affix::Infix(Precedence(2), Associativity::Neither),
            TokenTree::Infix('+') => Affix::Infix(Precedence(3), Associativity::Left),
            TokenTree::Infix('-') => Affix::Infix(Precedence(3), Associativity::Left),
            TokenTree::Infix('*') => Affix::Infix(Precedence(4), Associativity::Left),
            TokenTree::Infix('/') => Affix::Infix(Precedence(4), Associativity::Left),
            TokenTree::Postfix('?') => Affix::Postfix(Precedence(5)),
            TokenTree::Prefix('-') => Affix::Prefix(Precedence(6)),
            TokenTree::Prefix('!') => Affix::Prefix(Precedence(6)),
            TokenTree::Infix('^') => Affix::Infix(Precedence(7), Associativity::Right),
            TokenTree::Group(_) => Affix::Nilfix,
            TokenTree::Primary(_) => Affix::Nilfix,
            _ => unreachable!(),
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: TokenTree) -> Result<Expr> {
        let expr = match tree {
            TokenTree::Primary(num) => Expr::Int(num),
            TokenTree::Group(group) => self.parse(&mut group.into_iter()).unwrap(),
            _ => unreachable!(),
        };
        Ok(expr)
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(&mut self, lhs: Expr, tree: TokenTree, rhs: Expr) -> Result<Expr> {
        let op = match tree {
            TokenTree::Infix('+') => BinOpKind::Add,
            TokenTree::Infix('-') => BinOpKind::Sub,
            TokenTree::Infix('*') => BinOpKind::Mul,
            TokenTree::Infix('/') => BinOpKind::Div,
            TokenTree::Infix('^') => BinOpKind::Pow,
            TokenTree::Infix('=') => BinOpKind::Eq,
            _ => unreachable!(),
        };
        Ok(Expr::BinOp(Box::new(lhs), op, Box::new(rhs)))
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: TokenTree, rhs: Expr) -> Result<Expr> {
        let op = match tree {
            TokenTree::Prefix('!') => UnOpKind::Not,
            TokenTree::Prefix('-') => UnOpKind::Neg,
            _ => unreachable!(),
        };
        Ok(Expr::UnOp(op, Box::new(rhs)))
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: Expr, tree: TokenTree) -> Result<Expr> {
        let op = match tree {
            TokenTree::Postfix('?') => UnOpKind::Try,
            _ => unreachable!(),
        };
        Ok(Expr::UnOp(op, Box::new(lhs)))
    }
}