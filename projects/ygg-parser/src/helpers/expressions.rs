use super::*;

impl<'i> AstBuilder<'i> for ExpressionNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> yggdrasil_error::Result<Self::Output> {
        match self.expression_hard().as_slice() {
            [head, rest @ ..] => {
                let mut head = head.build(ctx, state)?;
                for term in rest {
                    head |= term.build(ctx, state)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class or", self.get_range()))?,
        }
    }
}

impl<'i> AstBuilder<'i> for ExpressionHardNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> yggdrasil_error::Result<Self::Output> {
        match self.expression_soft().as_slice() {
            [head, rest @ ..] => {
                let mut head = head.build(ctx, state)?;
                for term in rest {
                    head += term.build(ctx, state)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class hard", self.get_range()))?,
        }
    }
}

impl<'i> AstBuilder<'i> for ExpressionSoftNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> yggdrasil_error::Result<Self::Output> {
        match self.expression_tag().as_slice() {
            [head, rest @ ..] => {
                let mut head = head.build(ctx, state)?;
                for term in rest {
                    head &= term.build(ctx, state)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class soft", self.get_range()))?,
        }
    }
}

impl<'i> AstBuilder<'i> for ExpressionTagNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> yggdrasil_error::Result<Self::Output> {
        let e = match &self.identifier() {
            Some(first) => self.term().build(ctx, state)?.with_tag(first.build(ctx, state)?),
            None => self.term().build(ctx, state)?,
        };
        Ok(e)
    }
}
impl<'i> AstBuilder<'i> for TermNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> yggdrasil_error::Result<Self::Output> {
        let mut base = self.atomic().build(ctx, state)?;
        let mut unary = Vec::with_capacity(self.prefix().len() + self.suffix().len());
        for i in self.suffix() {
            let suffix = match i {
                SuffixNode::Optional(_) => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::OPTIONAL),
                SuffixNode::Many(_) => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::MANY),
                SuffixNode::Many1(_) => YggdrasilOperator::RepeatsBetween(YggdrasilCounter::MANY1),
                SuffixNode::RangeExact(u) => {
                    let i = u32::from_str_radix(u.integer().get_str(), 10).unwrap_or(u32::MAX);
                    YggdrasilOperator::RepeatsBetween(YggdrasilCounter::new(i, i))
                }
                SuffixNode::Range(v) => {
                    let min = match &v.min() {
                        Some(v) => u32::from_str_radix(v.get_str(), 10).unwrap_or(0),
                        None => 0,
                    };
                    let max = match &v.max() {
                        Some(v) => u32::from_str_radix(v.get_str(), 10).unwrap_or(u32::MAX),
                        None => u32::MAX,
                    };
                    YggdrasilOperator::RepeatsBetween(YggdrasilCounter::new(min, max))
                }
            };
            unary.push(suffix)
        }
        for i in self.prefix().iter().rev() {
            match i {
                PrefixNode::Negative(_) => unary.push(YggdrasilOperator::Negative),
                PrefixNode::Positive(_) => unary.push(YggdrasilOperator::Positive),
                PrefixNode::Remark(_) => base.remark = true,
            }
        }
        if unary.is_empty() { Ok(base) } else { Ok(UnaryExpression { base: Box::new(base), operators: unary }.into()) }
    }
}

impl<'i> AstBuilder<'i> for AtomicNode<'i> {
    type Output = YggdrasilExpression;

    fn build(&self, ctx: &ParseContext, state: &mut ParseState) -> yggdrasil_error::Result<Self::Output> {
        let expr = match self {
            AtomicNode::GroupExpression(e) => e.expression().build(ctx, state)?,
            AtomicNode::Boolean(v) => match v {
                BooleanNode::False(_) => YggdrasilExpression::boolean(false),
                BooleanNode::True(_) => YggdrasilExpression::boolean(true),
            },
            AtomicNode::FunctionCall(_) => {
                todo!()
            }
            AtomicNode::RegexEmbed(v) => YggdrasilRegex::new(v.get_str().trim(), v.get_range()).into(),
            AtomicNode::RegexRange(v) => YggdrasilRegex::new(v.get_str(), v.get_range()).into(),
            AtomicNode::StringRaw(s) => YggdrasilText::new(s.string_raw_text().get_str(), s.get_range()).into(),
            AtomicNode::StringNormal(s) => {
                let mut buffer = String::with_capacity(s.string_item().len() * 2);
                for item in &s.string_item() {
                    match item {
                        StringItemNode::EscapedCharacter(item) => match item.get_chars().last() {
                            Some(c) => match c {
                                'r' => buffer.push('\r'),
                                'n' => buffer.push('\n'),
                                _ => buffer.push(c),
                            },
                            None => unreachable!(),
                        },
                        StringItemNode::EscapedUnicode(unicode) => match unicode.as_char() {
                            Some(c) => buffer.push(c),
                            None => {
                                println!("parse fail: {:?}", unicode)
                            }
                        },
                        StringItemNode::TextAny(s) => buffer.push_str(s.get_str()),
                    }
                }
                YggdrasilText::new(buffer, s.get_range()).into()
            }
            AtomicNode::Category(cat) => {
                let r = cat.get_range();
                match &cat.group() {
                    Some(g) => YggdrasilRegex::new(format!("\\p{{{}={}}}", g.get_str(), cat.script().get_str()), r).into(),
                    None => YggdrasilRegex::new(format!("\\p{{{}}}", cat.script().get_str()), r).into(),
                }
            }
            AtomicNode::EscapedUnicode(unicode) => match unicode.as_char() {
                Some(c) => YggdrasilText::new(c, unicode.get_range()).into(),
                None => Err(YggdrasilError::syntax_error(unicode.hex().get_str(), unicode.get_range()))?,
            },
            AtomicNode::Identifier(v) => v.build(ctx, state)?.into(),
            AtomicNode::Integer(v) => BigInt::from_str_radix(v.get_str(), 10)?.into(),
        };
        Ok(expr)
    }
}

impl<'i> ExpressionNode<'i> {
    pub fn to_single(self) -> Option<ExpressionHardNode<'i>> {
        let mut children = self.expression_hard();
        if children.len() == 1 { children.pop() } else { None }
    }
    pub fn to_atom(self) -> Option<AtomicNode<'i>> {
        self.to_single()?.to_atom()
    }
    pub fn to_boolean(self) -> Option<bool> {
        match self.to_atom()? {
            AtomicNode::Boolean(BooleanNode::True(_)) => Some(true),
            AtomicNode::Boolean(BooleanNode::False(_)) => Some(false),
            _ => None,
        }
    }
    pub fn to_identifier(self) -> Option<IdentifierNode<'i>> {
        if let AtomicNode::Identifier(v) = self.to_atom()? {
            return Some(v);
        }
        return None;
    }
}

impl<'i> ExpressionHardNode<'i> {
    pub fn to_single(self) -> Option<ExpressionSoftNode<'i>> {
        let mut children = self.expression_soft();
        if children.len() == 1 { return children.pop() } else { None }
    }

    pub fn to_atom(self) -> Option<AtomicNode<'i>> {
        self.to_single()?.to_single()?.to_single()?.to_single()
    }
    pub fn to_identifier(self) -> Option<IdentifierNode<'i>> {
        self.to_atom()?.to_identifier()
    }
}

impl<'i> ExpressionSoftNode<'i> {
    pub fn to_single(self) -> Option<ExpressionTagNode<'i>> {
        let mut children = self.expression_tag();
        if children.len() == 1 { return children.pop() } else { None }
    }
}
impl<'i> ExpressionTagNode<'i> {
    pub fn to_single(self) -> Option<TermNode<'i>> {
        match self.identifier() {
            Some(_) => None,
            _ => Some(self.term()),
        }
    }
}

impl<'i> TermNode<'i> {
    pub fn to_single(self) -> Option<AtomicNode<'i>> {
        if self.prefix().is_empty() && self.suffix().is_empty() {
            return Some(self.atomic());
        }
        return None;
    }
}

impl<'i> AtomicNode<'i> {
    pub fn to_identifier(self) -> Option<IdentifierNode<'i>> {
        match self {
            Self::Identifier(s) => Some(s),
            _ => None,
        }
    }
}
