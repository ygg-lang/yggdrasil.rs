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
            statement: pair.take_tagged_items::<StatementNode>(Cow::Borrowed("statement")).collect::<Result<Vec<_>, _>>(),
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
            Self::Statement0 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("statement_0") {
            return Ok(Self::Statement0)
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
impl YggdrasilNode for GrammarStatementNode {
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
impl FromStr for GrammarStatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarStatement)?)
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
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
impl YggdrasilNode for ClassStatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            annotation_call: pair.take_tagged_items::<AnnotationCallNode>(Cow::Borrowed("annotation_call")).collect::<Result<Vec<_>, _>>(),
            class_block: pair.take_tagged_one::<ClassBlockNode>(Cow::Borrowed("class_block"))?,
            // Missing rule ModifierCall
            // Missing rule OP_REMARK
            cast: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("cast")),
            name: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("name"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ClassStatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ClassStatement)?)
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
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
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
impl YggdrasilNode for UnionStatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            annotation_call: pair.take_tagged_items::<AnnotationCallNode>(Cow::Borrowed("annotation_call")).collect::<Result<Vec<_>, _>>(),
            // Missing rule ModifierCall
            // Missing rule OP_REMARK
            union_block: pair.take_tagged_one::<UnionBlockNode>(Cow::Borrowed("union_block"))?,
            name: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("name"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for UnionStatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionStatement)?)
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
            union_branch: pair.take_tagged_items::<UnionBranchNode>(Cow::Borrowed("union_branch")).collect::<Result<Vec<_>, _>>(),
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
            branch_tag: pair.take_tagged_option::<BranchTagNode>(Cow::Borrowed("branch_tag")),
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
impl YggdrasilNode for BranchTagNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
            right_associativity: pair.take_tagged_option::<RightAssociativityNode>(Cow::Borrowed("right_associativity")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for BranchTagNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::BranchTag)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RightAssociativityNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RightAssociativityNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RightAssociativity)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GroupStatementNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            annotation_call: pair.take_tagged_items::<AnnotationCallNode>(Cow::Borrowed("annotation_call")).collect::<Result<Vec<_>, _>>(),
            group_block: pair.take_tagged_one::<GroupBlockNode>(Cow::Borrowed("group_block"))?,
            identifier: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("identifier")),
            // Missing rule ModifierCall
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for GroupStatementNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupStatement)?)
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
            group_pair: pair.take_tagged_items::<GroupPairNode>(Cow::Borrowed("group_pair")).collect::<Result<Vec<_>, _>>(),
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
impl YggdrasilNode for AnnotationCallNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            annotation_name: pair.take_tagged_one::<AnnotationNameNode>(Cow::Borrowed("annotation_name"))?,
            call_body: pair.take_tagged_one::<CallBodyNode>(Cow::Borrowed("call_body"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for AnnotationCallNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::AnnotationCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AnnotationNameNode {
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
impl FromStr for AnnotationNameNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::AnnotationName)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for FunctionCallNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            call_body: pair.take_tagged_one::<CallBodyNode>(Cow::Borrowed("call_body"))?,
            function_name: pair.take_tagged_one::<FunctionNameNode>(Cow::Borrowed("function_name"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for FunctionCallNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::FunctionCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for FunctionNameNode {
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
impl FromStr for FunctionNameNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::FunctionName)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for CallBodyNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_items::<ExpressionNode>(Cow::Borrowed("expression")).collect::<Result<Vec<_>, _>>(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for CallBodyNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::CallBody)?)
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
            expression_hard: pair.take_tagged_items::<ExpressionHardNode>(Cow::Borrowed("expression_hard")).collect::<Result<Vec<_>, _>>(),
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
impl YggdrasilNode for ExpressionHardNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression_soft: pair.take_tagged_items::<ExpressionSoftNode>(Cow::Borrowed("expression_soft")).collect::<Result<Vec<_>, _>>(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionHardNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionHard)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionSoftNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression_tag: pair.take_tagged_items::<ExpressionTagNode>(Cow::Borrowed("expression_tag")).collect::<Result<Vec<_>, _>>(),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionSoftNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionSoft)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionTagNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("identifier")),
            term: pair.take_tagged_one::<TermNode>(Cow::Borrowed("term"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionTagNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionTag)?)
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
            prefix: pair.take_tagged_items::<PrefixNode>(Cow::Borrowed("prefix")).collect::<Result<Vec<_>, _>>(),
            suffix: pair.take_tagged_items::<SuffixNode>(Cow::Borrowed("suffix")).collect::<Result<Vec<_>, _>>(),
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
impl YggdrasilNode for PrefixNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Prefix0 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("prefix_0") {
            return Ok(Self::Prefix0)
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
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("suffix_0") {
            return Ok(Self::Suffix0)
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
            Self::Atomic0 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("atomic_0") {
            return Ok(Self::Atomic0)
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
impl YggdrasilNode for GroupExpressionNode {
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
impl FromStr for GroupExpressionNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupExpression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::String0 => None,
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("string_0") {
            return Ok(Self::String0)
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
impl YggdrasilNode for RegexEmbedNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RegexEmbedNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexEmbed)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RegexRangeNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            regex_negative: pair.take_tagged_option::<RegexNegativeNode>(Cow::Borrowed("regex_negative")),
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RegexRangeNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexRange)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for RegexNegativeNode {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for RegexNegativeNode {
    type Err = YggdrasilError<BootstrapRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<BootstrapRule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexNegative)?)
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
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>(),
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
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>(),
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
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("boolean_0") {
            return Ok(Self::Boolean0)
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
