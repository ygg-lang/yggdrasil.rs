use super::*;

#[automatically_derived]
impl YggdrasilNode for RootNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            // Missing rule Rule("Statements")
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl FromStr for RootNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Root)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for StatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::ClassStatements(s) => s.get_range(),
            Self::UnionStatements(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ClassStatementsNode>(Cow::Borrowed("class_statements")) {
            return Ok(Self::ClassStatements(s));
        }
        if let Ok(s) = pair.take_tagged_one::<UnionStatementsNode>(Cow::Borrowed("union_statements")) {
            return Ok(Self::UnionStatements(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Statement, _span))
    }
}

#[automatically_derived]
impl FromStr for StatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Statement)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for ClassStatementsNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            kw_class: pair.take_tagged_one::<KwClassNode>(Cow::Borrowed("kw_class"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl FromStr for ClassStatementsNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ClassStatements)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for UnionStatementsNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            kw_union: pair.take_tagged_one::<KwUnionNode>(Cow::Borrowed("kw_union"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl FromStr for UnionStatementsNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionStatements)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for KwClassNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl FromStr for KwClassNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_CLASS)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for KwUnionNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl FromStr for KwUnionNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_UNION)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for CommentNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Comment0 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("comment_0") {
            return Ok(Self::Comment0);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Comment, _span))
    }
}

#[automatically_derived]
impl FromStr for CommentNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Comment)?)
    }
}
