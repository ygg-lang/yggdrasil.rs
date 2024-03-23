#![allow(unused_variables)]
use super::*;
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RootNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Root)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Root
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RootNode<'i> {
    pub fn statement(&self) -> Vec<StatementNode<'i>> {
        self.pair.take_tagged_items("statement").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StatementNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Statement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("grammar_statement") {
            return Ok(Self::GrammarStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one("class_statement") {
            return Ok(Self::ClassStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one("union_statement") {
            return Ok(Self::UnionStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one("group_statement") {
            return Ok(Self::GroupStatement(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Statement, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Statement
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::GrammarStatement(s) => s.get_str(),
            Self::ClassStatement(s) => s.get_str(),
            Self::UnionStatement(s) => s.get_str(),
            Self::GroupStatement(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarStatement(s) => s.get_range(),
            Self::ClassStatement(s) => s.get_range(),
            Self::UnionStatement(s) => s.get_range(),
            Self::GroupStatement(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarStatementNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarStatement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GrammarStatement
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GrammarStatementNode<'i> {
    pub fn grammar_dict(&self) -> GrammarDictNode<'i> {
        self.pair.take_tagged_one("grammar_dict").unwrap()
    }
    pub fn identifier(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("identifier").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarTerm1Node<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarTerm1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GrammarTerm1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GrammarTerm1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarTermNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarTerm)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("grammar_pair") {
            return Ok(Self::GrammarPair(s));
        }
        if let Ok(s) = pair.take_tagged_one("grammar_term_1") {
            return Ok(Self::GrammarTerm1(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::GrammarTerm, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GrammarTerm
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::GrammarPair(s) => s.get_str(),
            Self::GrammarTerm1(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarPair(s) => s.get_range(),
            Self::GrammarTerm1(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarPairNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarPair)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GrammarPair
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GrammarPairNode<'i> {
    pub fn grammar_value(&self) -> GrammarValueNode<'i> {
        self.pair.take_tagged_one("grammar_value").unwrap()
    }
    pub fn key(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("key").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarValueNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarValue)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("grammar_dict") {
            return Ok(Self::GrammarDict(s));
        }
        if let Ok(s) = pair.take_tagged_one("grammar_list") {
            return Ok(Self::GrammarList(s));
        }
        if let Ok(s) = pair.take_tagged_one("namepath") {
            return Ok(Self::Namepath(s));
        }
        if let Ok(s) = pair.take_tagged_one("string_raw") {
            return Ok(Self::StringRaw(s));
        }
        if let Ok(s) = pair.take_tagged_one("string_normal") {
            return Ok(Self::StringNormal(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::GrammarValue, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GrammarValue
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::GrammarDict(s) => s.get_str(),
            Self::GrammarList(s) => s.get_str(),
            Self::Namepath(s) => s.get_str(),
            Self::StringRaw(s) => s.get_str(),
            Self::StringNormal(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarDict(s) => s.get_range(),
            Self::GrammarList(s) => s.get_range(),
            Self::Namepath(s) => s.get_range(),
            Self::StringRaw(s) => s.get_range(),
            Self::StringNormal(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarDictNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarDict)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GrammarDict
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GrammarDictNode<'i> {
    pub fn grammar_term(&self) -> Vec<GrammarTermNode<'i>> {
        self.pair.take_tagged_items("grammar_term").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarListNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarList)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GrammarList
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GrammarListNode<'i> {
    pub fn grammar_value(&self) -> Vec<GrammarValueNode<'i>> {
        self.pair.take_tagged_items("grammar_value").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarListTermsNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GrammarListTerms)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GrammarListTerms
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GrammarListTermsNode<'i> {
    pub fn grammar_value(&self) -> Vec<GrammarValueNode<'i>> {
        self.pair.take_tagged_items("grammar_value").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ClassStatementNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ClassStatement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::ClassStatement
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ClassStatementNode<'i> {
    pub fn class_block(&self) -> ClassBlockNode<'i> {
        self.pair.take_tagged_one("class_block").unwrap()
    }
    pub fn decorator_call(&self) -> Vec<DecoratorCallNode<'i>> {
        self.pair.take_tagged_items("decorator_call").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn modifier_call(&self) -> Vec<ModifierCallNode<'i>> {
        self.pair.take_tagged_items("modifier_call").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn op_remark(&self) -> Option<OpRemarkNode<'i>> {
        self.pair.take_tagged_option("op_remark")
    }
    pub fn cast(&self) -> Option<IdentifierNode<'i>> {
        self.pair.take_tagged_option("cast")
    }
    pub fn name(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("name").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ClassBlockNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ClassBlock)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::ClassBlock
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ClassBlockNode<'i> {
    pub fn expression(&self) -> ExpressionNode<'i> {
        self.pair.take_tagged_one("expression").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for OpRemarkNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::OP_REMARK)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::OP_REMARK
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> OpRemarkNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for UnionStatementNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionStatement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::UnionStatement
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> UnionStatementNode<'i> {
    pub fn decorator_call(&self) -> Vec<DecoratorCallNode<'i>> {
        self.pair.take_tagged_items("decorator_call").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn modifier_call(&self) -> Vec<ModifierCallNode<'i>> {
        self.pair.take_tagged_items("modifier_call").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn op_remark(&self) -> Option<OpRemarkNode<'i>> {
        self.pair.take_tagged_option("op_remark")
    }
    pub fn union_block(&self) -> UnionBlockNode<'i> {
        self.pair.take_tagged_one("union_block").unwrap()
    }
    pub fn name(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("name").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for UnionBlockNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionBlock)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::UnionBlock
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> UnionBlockNode<'i> {
    pub fn union_branch(&self) -> Vec<UnionBranchNode<'i>> {
        self.pair.take_tagged_items("union_branch").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for UnionBranchNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::UnionBranch)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::UnionBranch
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> UnionBranchNode<'i> {
    pub fn branch_tag(&'i self) -> Option<BranchTagNode<'i>> {
        self.pair.take_tagged_option("branch_tag")
    }
    pub fn expression_hard(&'i self) -> ExpressionHardNode<'i> {
        self.pair.take_tagged_one("expression_hard").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BranchTagNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::BranchTag)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::BranchTag
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> BranchTagNode<'i> {
    pub fn identifier(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("identifier").unwrap()
    }
    pub fn right_associativity(&self) -> Option<RightAssociativityNode<'i>> {
        self.pair.take_tagged_option("right_associativity")
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RightAssociativityNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RightAssociativity)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::RightAssociativity
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RightAssociativityNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupStatementNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupStatement)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GroupStatement
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GroupStatementNode<'i> {
    pub fn decorator_call(&self) -> Vec<DecoratorCallNode<'i>> {
        self.pair.take_tagged_items("decorator_call").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn group_block(&self) -> GroupBlockNode<'i> {
        self.pair.take_tagged_one("group_block").unwrap()
    }
    pub fn identifier(&self) -> Option<IdentifierNode<'i>> {
        self.pair.take_tagged_option("identifier")
    }
    pub fn modifier_call(&self) -> Vec<ModifierCallNode<'i>> {
        self.pair.take_tagged_items("modifier_call").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupBlockNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupBlock)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GroupBlock
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GroupBlockNode<'i> {
    pub fn group_pair(&self) -> Vec<GroupPairNode<'i>> {
        self.pair.take_tagged_items("group_pair").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupPairNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupPair)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GroupPair
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GroupPairNode<'i> {
    pub fn atomic(&self) -> AtomicNode<'i> {
        self.pair.take_tagged_one("atomic").unwrap()
    }
    pub fn identifier(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("identifier").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DecoratorCallNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::DecoratorCall)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::DecoratorCall
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> DecoratorCallNode<'i> {
    pub fn call_body(&self) -> CallBodyNode<'i> {
        self.pair.take_tagged_one("call_body").unwrap()
    }
    pub fn decorator_name(&self) -> DecoratorNameNode<'i> {
        self.pair.take_tagged_one("decorator_name").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for DecoratorNameNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::DecoratorName)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::DecoratorName
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> DecoratorNameNode<'i> {
    pub fn identifier(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("identifier").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for FunctionCallNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::FunctionCall)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::FunctionCall
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> FunctionCallNode<'i> {
    pub fn call_body(&self) -> CallBodyNode<'i> {
        self.pair.take_tagged_one("call_body").unwrap()
    }
    pub fn function_name(&self) -> FunctionNameNode<'i> {
        self.pair.take_tagged_one("function_name").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for FunctionNameNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::FunctionName)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::FunctionName
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> FunctionNameNode<'i> {
    pub fn identifier(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("identifier").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CallBodyNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::CallBody)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::CallBody
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> CallBodyNode<'i> {
    pub fn expression(&self) -> Vec<ExpressionNode<'i>> {
        self.pair.take_tagged_items("expression").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Expression)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Expression
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ExpressionNode<'i> {
    pub fn expression_hard(&self) -> Vec<ExpressionHardNode<'i>> {
        self.pair.take_tagged_items("expression_hard").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionHardNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionHard)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::ExpressionHard
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ExpressionHardNode<'i> {
    pub fn expression_soft(&self) -> Vec<ExpressionSoftNode<'i>> {
        self.pair.take_tagged_items("expression_soft").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionSoftNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionSoft)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::ExpressionSoft
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ExpressionSoftNode<'i> {
    pub fn expression_tag(&self) -> Vec<ExpressionTagNode<'i>> {
        self.pair.take_tagged_items("expression_tag").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExpressionTagNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ExpressionTag)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::ExpressionTag
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ExpressionTagNode<'i> {
    pub fn identifier(&self) -> Option<IdentifierNode<'i>> {
        self.pair.take_tagged_option("identifier")
    }
    pub fn term(&self) -> TermNode<'i> {
        self.pair.take_tagged_one("term").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TermNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Term)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Term
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TermNode<'i> {
    pub fn atomic(&self) -> AtomicNode<'i> {
        self.pair.take_tagged_one("atomic").unwrap()
    }
    pub fn prefix(&self) -> Vec<PrefixNode<'i>> {
        self.pair.take_tagged_items("prefix").collect::<Result<Vec<_>, _>>().unwrap()
    }
    pub fn suffix(&self) -> Vec<SuffixNode<'i>> {
        self.pair.take_tagged_items("suffix").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NegativeNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Negative)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Negative
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> NegativeNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for PositiveNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Positive)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Positive
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> PositiveNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RemarkNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Remark)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Remark
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RemarkNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for PrefixNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Prefix)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("negative") {
            return Ok(Self::Negative(s));
        }
        if let Ok(s) = pair.take_tagged_one("positive") {
            return Ok(Self::Positive(s));
        }
        if let Ok(s) = pair.take_tagged_one("remark") {
            return Ok(Self::Remark(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Prefix, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Prefix
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Negative(s) => s.get_str(),
            Self::Positive(s) => s.get_str(),
            Self::Remark(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Negative(s) => s.get_range(),
            Self::Positive(s) => s.get_range(),
            Self::Remark(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for OptionalNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Optional)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Optional
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> OptionalNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ManyNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Many)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Many
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ManyNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for Many1Node<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Many1)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Many1
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> Many1Node<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SuffixNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Suffix)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("optional") {
            return Ok(Self::Optional(s));
        }
        if let Ok(s) = pair.take_tagged_one("many") {
            return Ok(Self::Many(s));
        }
        if let Ok(s) = pair.take_tagged_one("many_1") {
            return Ok(Self::Many1(s));
        }
        if let Ok(s) = pair.take_tagged_one("range_exact") {
            return Ok(Self::RangeExact(s));
        }
        if let Ok(s) = pair.take_tagged_one("range") {
            return Ok(Self::Range(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Suffix, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Suffix
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Optional(s) => s.get_str(),
            Self::Many(s) => s.get_str(),
            Self::Many1(s) => s.get_str(),
            Self::RangeExact(s) => s.get_str(),
            Self::Range(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Optional(s) => s.get_range(),
            Self::Many(s) => s.get_range(),
            Self::Many1(s) => s.get_range(),
            Self::RangeExact(s) => s.get_range(),
            Self::Range(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for AtomicNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Atomic)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("group_expression") {
            return Ok(Self::GroupExpression(s));
        }
        if let Ok(s) = pair.take_tagged_one("function_call") {
            return Ok(Self::FunctionCall(s));
        }
        if let Ok(s) = pair.take_tagged_one("boolean") {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one("integer") {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one("string_raw") {
            return Ok(Self::StringRaw(s));
        }
        if let Ok(s) = pair.take_tagged_one("string_normal") {
            return Ok(Self::StringNormal(s));
        }
        if let Ok(s) = pair.take_tagged_one("category") {
            return Ok(Self::Category(s));
        }
        if let Ok(s) = pair.take_tagged_one("escaped_unicode") {
            return Ok(Self::EscapedUnicode(s));
        }
        if let Ok(s) = pair.take_tagged_one("regex_embed") {
            return Ok(Self::RegexEmbed(s));
        }
        if let Ok(s) = pair.take_tagged_one("regex_range") {
            return Ok(Self::RegexRange(s));
        }
        if let Ok(s) = pair.take_tagged_one("identifier") {
            return Ok(Self::Identifier(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Atomic, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Atomic
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::GroupExpression(s) => s.get_str(),
            Self::FunctionCall(s) => s.get_str(),
            Self::Boolean(s) => s.get_str(),
            Self::Integer(s) => s.get_str(),
            Self::StringRaw(s) => s.get_str(),
            Self::StringNormal(s) => s.get_str(),
            Self::Category(s) => s.get_str(),
            Self::EscapedUnicode(s) => s.get_str(),
            Self::RegexEmbed(s) => s.get_str(),
            Self::RegexRange(s) => s.get_str(),
            Self::Identifier(s) => s.get_str(),
        }
    }

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
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GroupExpressionNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::GroupExpression)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::GroupExpression
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> GroupExpressionNode<'i> {
    pub fn expression(&self) -> ExpressionNode<'i> {
        self.pair.take_tagged_one("expression").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringRawNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::StringRaw)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::StringRaw
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> StringRawNode<'i> {
    pub fn string_raw_text(&self) -> StringRawTextNode<'i> {
        self.pair.take_tagged_one("string_raw_text").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringRawTextNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::StringRawText)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::StringRawText
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> StringRawTextNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringNormalNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::StringNormal)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::StringNormal
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> StringNormalNode<'i> {
    pub fn string_item(&self) -> Vec<StringItemNode<'i>> {
        self.pair.take_tagged_items("string_item").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringItemNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::StringItem)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("escaped_unicode") {
            return Ok(Self::EscapedUnicode(s));
        }
        if let Ok(s) = pair.take_tagged_one("escaped_character") {
            return Ok(Self::EscapedCharacter(s));
        }
        if let Ok(s) = pair.take_tagged_one("text_any") {
            return Ok(Self::TextAny(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::StringItem, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::StringItem
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::EscapedUnicode(s) => s.get_str(),
            Self::EscapedCharacter(s) => s.get_str(),
            Self::TextAny(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::EscapedUnicode(s) => s.get_range(),
            Self::EscapedCharacter(s) => s.get_range(),
            Self::TextAny(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for EscapedUnicodeNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::EscapedUnicode)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::EscapedUnicode
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> EscapedUnicodeNode<'i> {
    pub fn hex(&self) -> HexNode<'i> {
        self.pair.take_tagged_one("hex").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for EscapedCharacterNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::EscapedCharacter)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::EscapedCharacter
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> EscapedCharacterNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for HexNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::HEX)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::HEX
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> HexNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TextAnyNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::TextAny)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::TextAny
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TextAnyNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexEmbedNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexEmbed)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::RegexEmbed
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RegexEmbedNode<'i> {
    pub fn regex_item(&self) -> Vec<RegexItemNode<'i>> {
        self.pair.take_tagged_items("regex_item").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexItemNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexItem)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("escaped_character") {
            return Ok(Self::EscapedCharacter(s));
        }
        if let Ok(s) = pair.take_tagged_one("regex_character") {
            return Ok(Self::RegexCharacter(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::RegexItem, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::RegexItem
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::EscapedCharacter(s) => s.get_str(),
            Self::RegexCharacter(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::EscapedCharacter(s) => s.get_range(),
            Self::RegexCharacter(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexCharacterNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexCharacter)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::RegexCharacter
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RegexCharacterNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexRangeNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexRange)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::RegexRange
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RegexRangeNode<'i> {
    pub fn regex_negative(&self) -> Option<RegexNegativeNode<'i>> {
        self.pair.take_tagged_option("regex_negative")
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RegexNegativeNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RegexNegative)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::RegexNegative
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RegexNegativeNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CategoryNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Category)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Category
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> CategoryNode<'i> {
    pub fn group(&self) -> Option<IdentifierNode<'i>> {
        self.pair.take_tagged_option("group")
    }
    pub fn script(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("script").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NamepathFreeNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::NamepathFree)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::NamepathFree
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> NamepathFreeNode<'i> {
    pub fn identifier(&self) -> Vec<IdentifierNode<'i>> {
        self.pair.take_tagged_items("identifier").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for NamepathNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Namepath)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Namepath
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> NamepathNode<'i> {
    pub fn identifier(&self) -> Vec<IdentifierNode<'i>> {
        self.pair.take_tagged_items("identifier").collect::<Result<Vec<_>, _>>().unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IdentifierNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Identifier)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Identifier
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> IdentifierNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for TrueNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::True)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::True
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> TrueNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for FalseNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::False)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::False
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> FalseNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BooleanNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Boolean)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("true") {
            return Ok(Self::True(s));
        }
        if let Ok(s) = pair.take_tagged_one("false") {
            return Ok(Self::False(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Boolean, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Boolean
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::True(s) => s.get_str(),
            Self::False(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::True(s) => s.get_range(),
            Self::False(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for IntegerNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Integer)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Integer
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> IntegerNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RangeExactNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::RangeExact)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::RangeExact
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RangeExactNode<'i> {
    pub fn integer(&self) -> IntegerNode<'i> {
        self.pair.take_tagged_one("integer").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for RangeNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Range)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Range
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> RangeNode<'i> {
    pub fn max(&self) -> Option<IntegerNode<'i>> {
        self.pair.take_tagged_option("max")
    }
    pub fn min(&self) -> Option<IntegerNode<'i>> {
        self.pair.take_tagged_option("min")
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ModifierCallNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::ModifierCall)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::ModifierCall
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ModifierCallNode<'i> {
    pub fn identifier(&self) -> IdentifierNode<'i> {
        self.pair.take_tagged_one("identifier").unwrap()
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for OpCategoryNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::OP_CATEGORY)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::OP_CATEGORY
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> OpCategoryNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ParserNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Parser)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Parser
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ParserNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for InspectorNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Inspector)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Inspector
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> InspectorNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for ExternalNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::External)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::External
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> ExternalNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwExternalNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_EXTERNAL)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one("parser") {
            return Ok(Self::Parser(s));
        }
        if let Ok(s) = pair.take_tagged_one("inspector") {
            return Ok(Self::Inspector(s));
        }
        if let Ok(s) = pair.take_tagged_one("external") {
            return Ok(Self::External(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::KW_EXTERNAL, _span))
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::KW_EXTERNAL
    }

    fn get_str(&self) -> &'i str {
        match self {
            Self::Parser(s) => s.get_str(),
            Self::Inspector(s) => s.get_str(),
            Self::External(s) => s.get_str(),
        }
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Parser(s) => s.get_range(),
            Self::Inspector(s) => s.get_range(),
            Self::External(s) => s.get_range(),
        }
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwGrammarNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_GRAMMAR)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::KW_GRAMMAR
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> KwGrammarNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwImportNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_IMPORT)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::KW_IMPORT
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> KwImportNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwClassNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_CLASS)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::KW_CLASS
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> KwClassNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwUnionNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_UNION)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::KW_UNION
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> KwUnionNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwGroupNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_GROUP)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::KW_GROUP
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> KwGroupNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwClimbNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_CLIMB)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::KW_CLIMB
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> KwClimbNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwMacroNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::KW_MACRO)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::KW_MACRO
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> KwMacroNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for WhiteSpaceNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::WhiteSpace)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::WhiteSpace
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> WhiteSpaceNode<'i> {}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for CommentNode<'i> {
    type Rule = BootstrapRule;

    fn from_str(input: &'i str, offset: usize) -> Result<Self, YggdrasilError<Self::Rule>> {
        Self::from_cst(BootstrapParser::parse_cst(input, BootstrapRule::Comment)?)
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        Ok(Self { pair })
    }

    fn get_rule(&self) -> Self::Rule {
        BootstrapRule::Comment
    }

    fn get_str(&self) -> &'i str {
        self.pair.get_span().as_str()
    }

    fn get_range(&self) -> Range<usize> {
        self.pair.get_span().get_range()
    }
}
impl<'i> CommentNode<'i> {}
