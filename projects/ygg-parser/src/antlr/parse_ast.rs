use super::*;
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RootNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RootNode<'i> {
    pub fn statement(&self) -> TokenTreeFilterTag<'i, StatementNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("statement"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StatementNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarStatement(s) => s.get_range(),
            Self::ClassStatement(s) => s.get_range(),
            Self::UnionStatement(s) => s.get_range(),
            Self::GroupStatement(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("grammar_statement")) {
            return Ok(Self::GrammarStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("class_statement")) {
            return Ok(Self::ClassStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("union_statement")) {
            return Ok(Self::UnionStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("group_statement")) {
            return Ok(Self::GroupStatement(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Statement, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarStatementNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GrammarStatementNode<'i> {
    pub fn grammar_dict(&self) {
        // pair.take_tagged_one::<GrammarDictNode>(Cow::Borrowed("grammar_dict"))?,
    }
    pub fn identifier(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarTerm1Node<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GrammarTerm1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarTermNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarPair(s) => s.get_range(),
            Self::GrammarTerm1(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("grammar_pair")) {
            return Ok(Self::GrammarPair(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("grammar_term_1")) {
            return Ok(Self::GrammarTerm1(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::GrammarTerm, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarPairNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GrammarPairNode<'i> {
    pub fn grammar_value(&self) {
        // pair.take_tagged_one::<GrammarValueNode>(Cow::Borrowed("grammar_value"))?,
    }
    pub fn key(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("key"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarValueNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarDict(s) => s.get_range(),
            Self::GrammarList(s) => s.get_range(),
            Self::Namepath(s) => s.get_range(),
            Self::StringRaw(s) => s.get_range(),
            Self::StringNormal(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("grammar_dict")) {
            return Ok(Self::GrammarDict(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("grammar_list")) {
            return Ok(Self::GrammarList(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("namepath")) {
            return Ok(Self::Namepath(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("string_raw")) {
            return Ok(Self::StringRaw(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("string_normal")) {
            return Ok(Self::StringNormal(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::GrammarValue, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarDictNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GrammarDictNode<'i> {
    pub fn grammar_term(&self) -> TokenTreeFilterTag<'i, GrammarTermNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("grammar_term"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarListNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GrammarListNode<'i> {
    pub fn grammar_value(&self) -> TokenTreeFilterTag<'i, GrammarValueNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("grammar_value"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarListTermsNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GrammarListTermsNode<'i> {
    pub fn grammar_value(&self) -> TokenTreeFilterTag<'i, GrammarValueNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("grammar_value"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ClassStatementNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ClassStatementNode<'i> {
    pub fn class_block(&self) {
        // pair.take_tagged_one::<ClassBlockNode>(Cow::Borrowed("class_block"))?,
    }
    pub fn decorator_call(&self) -> TokenTreeFilterTag<'i, DecoratorCallNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("decorator_call"))
    }
    pub fn modifier_call(&self) -> TokenTreeFilterTag<'i, ModifierCallNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("modifier_call"))
    }
    pub fn op_remark(&self) -> Option<OpRemarkNode<'i>> {
        self.pair.take_tagged_option(Cow::Borrowed("op_remark"))
    }
    pub fn cast(&self) {
        // pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("cast")),
    }
    pub fn name(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("name"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ClassBlockNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ClassBlockNode<'i> {
    pub fn expression(&self) {
        // pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for OpRemarkNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> OpRemarkNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for UnionStatementNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> UnionStatementNode<'i> {
    pub fn decorator_call(&self) -> TokenTreeFilterTag<'i, DecoratorCallNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("decorator_call"))
    }
    pub fn modifier_call(&self) -> TokenTreeFilterTag<'i, ModifierCallNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("modifier_call"))
    }
    pub fn op_remark(&self) -> Option<OpRemarkNode<'i>> {
        self.pair.take_tagged_option(Cow::Borrowed("op_remark"))
    }
    pub fn union_block(&self) {
        // pair.take_tagged_one::<UnionBlockNode>(Cow::Borrowed("union_block"))?,
    }
    pub fn name(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("name"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for UnionBlockNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> UnionBlockNode<'i> {
    pub fn union_branch(&self) -> TokenTreeFilterTag<'i, UnionBranchNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("union_branch"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for UnionBranchNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> UnionBranchNode<'i> {
    pub fn branch_tag(&self) {
        // pair.take_tagged_option::<BranchTagNode>(Cow::Borrowed("branch_tag")),
    }
    pub fn expression_hard(&self) {
        // pair.take_tagged_one::<ExpressionHardNode>(Cow::Borrowed("expression_hard"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BranchTagNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> BranchTagNode<'i> {
    pub fn identifier(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
    }
    pub fn right_associativity(&self) {
        // pair.take_tagged_option::<RightAssociativityNode>(Cow::Borrowed("right_associativity")),
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RightAssociativityNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RightAssociativityNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupStatementNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GroupStatementNode<'i> {
    pub fn decorator_call(&self) -> TokenTreeFilterTag<'i, DecoratorCallNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("decorator_call"))
    }
    pub fn group_block(&self) {
        // pair.take_tagged_one::<GroupBlockNode>(Cow::Borrowed("group_block"))?,
    }
    pub fn identifier(&self) {
        // pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("identifier")),
    }
    pub fn modifier_call(&self) -> TokenTreeFilterTag<'i, ModifierCallNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("modifier_call"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupBlockNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GroupBlockNode<'i> {
    pub fn group_pair(&self) -> TokenTreeFilterTag<'i, GroupPairNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("group_pair"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupPairNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GroupPairNode<'i> {
    pub fn atomic(&self) {
        // pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
    }
    pub fn identifier(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DecoratorCallNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> DecoratorCallNode<'i> {
    pub fn call_body(&self) {
        // pair.take_tagged_one::<CallBodyNode>(Cow::Borrowed("call_body"))?,
    }
    pub fn decorator_name(&self) {
        // pair.take_tagged_one::<DecoratorNameNode>(Cow::Borrowed("decorator_name"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DecoratorNameNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> DecoratorNameNode<'i> {
    pub fn identifier(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for FunctionCallNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> FunctionCallNode<'i> {
    pub fn call_body(&self) {
        // pair.take_tagged_one::<CallBodyNode>(Cow::Borrowed("call_body"))?,
    }
    pub fn function_name(&self) {
        // pair.take_tagged_one::<FunctionNameNode>(Cow::Borrowed("function_name"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for FunctionNameNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> FunctionNameNode<'i> {
    pub fn identifier(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CallBodyNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> CallBodyNode<'i> {
    pub fn expression(&self) -> TokenTreeFilterTag<'i, ExpressionNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("expression"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ExpressionNode<'i> {
    pub fn expression_hard(&self) -> TokenTreeFilterTag<'i, ExpressionHardNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("expression_hard"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionHardNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ExpressionHardNode<'i> {
    pub fn expression_soft(&self) -> TokenTreeFilterTag<'i, ExpressionSoftNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("expression_soft"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionSoftNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ExpressionSoftNode<'i> {
    pub fn expression_tag(&self) -> TokenTreeFilterTag<'i, ExpressionTagNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("expression_tag"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionTagNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ExpressionTagNode<'i> {
    pub fn identifier(&self) {
        // pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("identifier")),
    }
    pub fn term(&self) {
        // pair.take_tagged_one::<TermNode>(Cow::Borrowed("term"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TermNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> TermNode<'i> {
    pub fn atomic(&self) {
        // pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
    }
    pub fn prefix(&self) -> TokenTreeFilterTag<'i, PrefixNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("prefix"))
    }
    pub fn suffix(&self) -> TokenTreeFilterTag<'i, SuffixNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("suffix"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NegativeNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> NegativeNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for PositiveNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> PositiveNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RemarkNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RemarkNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for PrefixNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Negative(s) => s.get_range(),
            Self::Positive(s) => s.get_range(),
            Self::Remark(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("negative")) {
            return Ok(Self::Negative(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("positive")) {
            return Ok(Self::Positive(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("remark")) {
            return Ok(Self::Remark(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Prefix, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for OptionalNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> OptionalNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ManyNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ManyNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Many1Node<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> Many1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SuffixNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Optional(s) => s.get_range(),
            Self::Many(s) => s.get_range(),
            Self::Many1(s) => s.get_range(),
            Self::RangeExact(s) => s.get_range(),
            Self::Range(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("optional")) {
            return Ok(Self::Optional(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("many")) {
            return Ok(Self::Many(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("many_1")) {
            return Ok(Self::Many1(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("range_exact")) {
            return Ok(Self::RangeExact(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("range")) {
            return Ok(Self::Range(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Suffix, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for AtomicNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GroupExpression(s) => s.get_range(),
            Self::FunctionCall(s) => s.get_range(),
            Self::Boolean(s) => s.get_range(),
            Self::Integer(s) => s.get_range(),
            Self::StringRaw(s) => s.get_range(),
            Self::StringNormal(s) => s.get_range(),
            Self::Category(s) => s.get_range(),
            Self::EscapedUnicode(s) => s.get_range(),
            Self::RegexEmbed(s) => s.get_range(),
            Self::RegexRange(s) => s.get_range(),
            Self::Identifier(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("group_expression")) {
            return Ok(Self::GroupExpression(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("function_call")) {
            return Ok(Self::FunctionCall(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("integer")) {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("string_raw")) {
            return Ok(Self::StringRaw(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("string_normal")) {
            return Ok(Self::StringNormal(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("category")) {
            return Ok(Self::Category(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("escaped_unicode")) {
            return Ok(Self::EscapedUnicode(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("regex_embed")) {
            return Ok(Self::RegexEmbed(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("regex_range")) {
            return Ok(Self::RegexRange(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Atomic, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupExpressionNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> GroupExpressionNode<'i> {
    pub fn expression(&self) {
        // pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringRawNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> StringRawNode<'i> {
    pub fn string_raw_text(&self) {
        // pair.take_tagged_one::<StringRawTextNode>(Cow::Borrowed("string_raw_text"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringRawTextNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> StringRawTextNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringNormalNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> StringNormalNode<'i> {
    pub fn string_item(&self) -> TokenTreeFilterTag<'i, StringItemNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("string_item"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringItemNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::EscapedUnicode(s) => s.get_range(),
            Self::EscapedCharacter(s) => s.get_range(),
            Self::TextAny(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("escaped_unicode")) {
            return Ok(Self::EscapedUnicode(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("escaped_character")) {
            return Ok(Self::EscapedCharacter(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("text_any")) {
            return Ok(Self::TextAny(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::StringItem, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for EscapedUnicodeNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> EscapedUnicodeNode<'i> {
    pub fn hex(&self) {
        // pair.take_tagged_one::<HexNode>(Cow::Borrowed("hex"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for EscapedCharacterNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> EscapedCharacterNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for HexNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> HexNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TextAnyNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> TextAnyNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexEmbedNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RegexEmbedNode<'i> {
    pub fn regex_item(&self) -> TokenTreeFilterTag<'i, RegexItemNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("regex_item"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexItemNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::EscapedCharacter(s) => s.get_range(),
            Self::RegexCharacter(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("escaped_character")) {
            return Ok(Self::EscapedCharacter(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("regex_character")) {
            return Ok(Self::RegexCharacter(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::RegexItem, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexCharacterNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RegexCharacterNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexRangeNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RegexRangeNode<'i> {
    pub fn regex_negative(&self) {
        // pair.take_tagged_option::<RegexNegativeNode>(Cow::Borrowed("regex_negative")),
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexNegativeNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RegexNegativeNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CategoryNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> CategoryNode<'i> {
    pub fn group(&self) {
        // pair.take_tagged_option::<IdentifierNode>(Cow::Borrowed("group")),
    }
    pub fn script(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("script"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NamepathFreeNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> NamepathFreeNode<'i> {
    pub fn identifier(&self) -> TokenTreeFilterTag<'i, IdentifierNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("identifier"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NamepathNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> NamepathNode<'i> {
    pub fn identifier(&self) -> TokenTreeFilterTag<'i, IdentifierNode<'i>> {
        self.pair.take_tagged_items(Cow::Borrowed("identifier"))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IdentifierNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> IdentifierNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TrueNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> TrueNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for FalseNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> FalseNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BooleanNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::True(s) => s.get_range(),
            Self::False(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("true")) {
            return Ok(Self::True(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("false")) {
            return Ok(Self::False(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Boolean, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IntegerNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> IntegerNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RangeExactNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RangeExactNode<'i> {
    pub fn integer(&self) {
        // pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RangeNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> RangeNode<'i> {
    pub fn max(&self) {
        // pair.take_tagged_option::<IntegerNode>(Cow::Borrowed("max")),
    }
    pub fn min(&self) {
        // pair.take_tagged_option::<IntegerNode>(Cow::Borrowed("min")),
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ModifierCallNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ModifierCallNode<'i> {
    pub fn identifier(&self) {
        // pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier"))?,
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for OpCategoryNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> OpCategoryNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ParserNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ParserNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for InspectorNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> InspectorNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExternalNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> ExternalNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwExternalNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Parser(s) => s.get_range(),
            Self::Inspector(s) => s.get_range(),
            Self::External(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("parser")) {
            return Ok(Self::Parser(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("inspector")) {
            return Ok(Self::Inspector(s));
        }
        if let Ok(s) = pair.take_tagged_one(Cow::Borrowed("external")) {
            return Ok(Self::External(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::KW_EXTERNAL, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwGrammarNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> KwGrammarNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwImportNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> KwImportNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwClassNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> KwClassNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwUnionNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> KwUnionNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwGroupNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> KwGroupNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwClimbNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> KwClimbNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwMacroNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> KwMacroNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for WhiteSpaceNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> WhiteSpaceNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CommentNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }
}
impl<'i> CommentNode<'i> {}
