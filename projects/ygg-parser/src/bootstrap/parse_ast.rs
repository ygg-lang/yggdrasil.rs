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
            statement: pair.take_tagged_items::<StatementNode>(Cow::Borrowed("statement"))?,
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
            Self::GrammarStatements(s) => s.get_range(),
            Self::GroupStatements(s) => s.get_range(),
            Self::UnionStatements(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ClassStatementsNode>(Cow::Borrowed("class_statements")) {
            return Ok(Self::ClassStatements(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GrammarStatementsNode>(Cow::Borrowed("grammar_statements")) {
            return Ok(Self::GrammarStatements(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GroupStatementsNode>(Cow::Borrowed("group_statements")) {
            return Ok(Self::GroupStatements(s));
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
impl YggdrasilNode for GrammarStatementsNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            grammar_block: pair.take_tagged_one::<GrammarBlockNode>(Cow::Borrowed("grammar_block"))?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl FromStr for GrammarStatementsNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarStatements)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for GrammarBlockNode {
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
impl FromStr for GrammarBlockNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarBlock)?)
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
            class_block: pair.take_tagged_one::<ClassBlockNode>(Cow::Borrowed("class_block"))?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
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
impl YggdrasilNode for ClassBlockNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_items::<ExpressionNode>(Cow::Borrowed("expression"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassBlockNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ClassBlock)?)
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
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
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
impl YggdrasilNode for UnionBlockNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            union_branch: pair.take_tagged_items::<UnionBranchNode>(Cow::Borrowed("union_branch"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for UnionBlockNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionBlock)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for UnionBranchNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for UnionBranchNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionBranch)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GroupStatementsNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            group_block: pair.take_tagged_one::<GroupBlockNode>(Cow::Borrowed("group_block"))?,
            identifier: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("identifier")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for GroupStatementsNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupStatements)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for GroupBlockNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            group_pair: pair.take_tagged_items::<GroupPairNode>(Cow::Borrowed("group_pair"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl FromStr for GroupBlockNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupBlock)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for GroupPairNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl FromStr for GroupPairNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupPair)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            infix: pair.take_tagged_one::<InfixNode>(Cow::Borrowed("infix"))?,
            term: pair.take_tagged_items::<TermNode>(Cow::Borrowed("term"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Expression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TermNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            prefix: pair.take_tagged_items::<PrefixNode>(Cow::Borrowed("prefix"))?,
            suffix: pair.take_tagged_items::<SuffixNode>(Cow::Borrowed("suffix"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for TermNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Term)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for InfixNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Infix0 => None,
            Self::Infix1 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("infix_0") {
            return Ok(Self::Infix0);
        }
        if let Some(_) = pair.find_first_tag("infix_1") {
            return Ok(Self::Infix1);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Infix, _span))
    }
}
#[automatically_derived]
impl FromStr for InfixNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Infix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for PrefixNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Prefix0 => None,
            Self::Prefix1 => None,
            Self::Prefix2 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("prefix_0") {
            return Ok(Self::Prefix0);
        }
        if let Some(_) = pair.find_first_tag("prefix_1") {
            return Ok(Self::Prefix1);
        }
        if let Some(_) = pair.find_first_tag("prefix_2") {
            return Ok(Self::Prefix2);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Prefix, _span))
    }
}
#[automatically_derived]
impl FromStr for PrefixNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Prefix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SuffixNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Suffix0 => None,
            Self::Suffix1 => None,
            Self::Suffix2 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("suffix_0") {
            return Ok(Self::Suffix0);
        }
        if let Some(_) = pair.find_first_tag("suffix_1") {
            return Ok(Self::Suffix1);
        }
        if let Some(_) = pair.find_first_tag("suffix_2") {
            return Ok(Self::Suffix2);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Suffix, _span))
    }
}
#[automatically_derived]
impl FromStr for SuffixNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Suffix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomicNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Boolean(s) => s.get_range(),
            Self::Identifier(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<BooleanNode>(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Atomic, _span))
    }
}
#[automatically_derived]
impl FromStr for AtomicNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Atomic)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for StringNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::String0 => None,
            Self::String1 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("string_0") {
            return Ok(Self::String0);
        }
        if let Some(_) = pair.find_first_tag("string_1") {
            return Ok(Self::String1);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::String, _span))
    }
}

#[automatically_derived]
impl FromStr for StringNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::String)?)
    }
}

#[automatically_derived]
impl YggdrasilNode for RegexNode {
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
impl FromStr for RegexNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Regex)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathFreeNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathFreeNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::NamepathFree)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NamepathNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for NamepathNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Namepath)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierNode {
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
impl FromStr for IdentifierNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Identifier)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for BooleanNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Boolean0 => None,
            Self::Boolean1 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("boolean_0") {
            return Ok(Self::Boolean0);
        }
        if let Some(_) = pair.find_first_tag("boolean_1") {
            return Ok(Self::Boolean1);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Boolean, _span))
    }
}
#[automatically_derived]
impl FromStr for BooleanNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Boolean)?)
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
impl YggdrasilNode for KwGroupNode {
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
impl FromStr for KwGroupNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_GROUP)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
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
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::WhiteSpace)?)
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
