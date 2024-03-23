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
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StatementNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::ClassStatement(s) => s.get_range(),
            Self::GrammarStatement(s) => s.get_range(),
            Self::GroupStatement(s) => s.get_range(),
            Self::UnionStatement(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ClassStatementNode>(Cow::Borrowed("class_statement")) {
            return Ok(Self::ClassStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GrammarStatementNode>(Cow::Borrowed("grammar_statement")) {
            return Ok(Self::GrammarStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GroupStatementNode>(Cow::Borrowed("group_statement")) {
            return Ok(Self::GroupStatement(s));
        }
        if let Ok(s) = pair.take_tagged_one::<UnionStatementNode>(Cow::Borrowed("union_statement")) {
            return Ok(Self::UnionStatement(s));
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
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarTermNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarPair(s) => s.get_range(),
            Self::GrammarTerm1 => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<GrammarPairNode>(Cow::Borrowed("grammar_pair")) {
            return Ok(Self::GrammarPair(s));
        }
        if let Some(_) = pair.find_first_tag("grammar_term_1") {
            return Ok(Self::GrammarTerm1);
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
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for GrammarValueNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::GrammarDict(s) => s.get_range(),
            Self::GrammarList(s) => s.get_range(),
            Self::Namepath(s) => s.get_range(),
            Self::StringNormal(s) => s.get_range(),
            Self::StringRaw(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<GrammarDictNode>(Cow::Borrowed("grammar_dict")) {
            return Ok(Self::GrammarDict(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GrammarListNode>(Cow::Borrowed("grammar_list")) {
            return Ok(Self::GrammarList(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NamepathNode>(Cow::Borrowed("namepath")) {
            return Ok(Self::Namepath(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringNormalNode>(Cow::Borrowed("string_normal")) {
            return Ok(Self::StringNormal(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringRawNode>(Cow::Borrowed("string_raw")) {
            return Ok(Self::StringRaw(s));
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
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for PrefixNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Negative => Range::default(),
            Self::Positive => Range::default(),
            Self::Remark => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("negative") {
            return Ok(Self::Negative);
        }
        if let Some(_) = pair.find_first_tag("positive") {
            return Ok(Self::Positive);
        }
        if let Some(_) = pair.find_first_tag("remark") {
            return Ok(Self::Remark);
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Prefix, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for SuffixNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Many => Range::default(),
            Self::Many1 => Range::default(),
            Self::Optional => Range::default(),
            Self::Range(s) => s.get_range(),
            Self::RangeExact(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("many") {
            return Ok(Self::Many);
        }
        if let Some(_) = pair.find_first_tag("many_1") {
            return Ok(Self::Many1);
        }
        if let Some(_) = pair.find_first_tag("optional") {
            return Ok(Self::Optional);
        }
        if let Ok(s) = pair.take_tagged_one::<RangeNode>(Cow::Borrowed("range")) {
            return Ok(Self::Range(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RangeExactNode>(Cow::Borrowed("range_exact")) {
            return Ok(Self::RangeExact(s));
        }
        Err(YggdrasilError::invalid_node(BootstrapRule::Suffix, _span))
    }
}
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for AtomicNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::Boolean(s) => s.get_range(),
            Self::Category(s) => s.get_range(),
            Self::EscapedUnicode(s) => s.get_range(),
            Self::FunctionCall(s) => s.get_range(),
            Self::GroupExpression(s) => s.get_range(),
            Self::Identifier(s) => s.get_range(),
            Self::Integer(s) => s.get_range(),
            Self::RegexEmbed(s) => s.get_range(),
            Self::RegexRange(s) => s.get_range(),
            Self::StringNormal(s) => s.get_range(),
            Self::StringRaw(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<BooleanNode>(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one::<CategoryNode>(Cow::Borrowed("category")) {
            return Ok(Self::Category(s));
        }
        if let Ok(s) = pair.take_tagged_one::<EscapedUnicodeNode>(Cow::Borrowed("escaped_unicode")) {
            return Ok(Self::EscapedUnicode(s));
        }
        if let Ok(s) = pair.take_tagged_one::<FunctionCallNode>(Cow::Borrowed("function_call")) {
            return Ok(Self::FunctionCall(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GroupExpressionNode>(Cow::Borrowed("group_expression")) {
            return Ok(Self::GroupExpression(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer")) {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RegexEmbedNode>(Cow::Borrowed("regex_embed")) {
            return Ok(Self::RegexEmbed(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RegexRangeNode>(Cow::Borrowed("regex_range")) {
            return Ok(Self::RegexRange(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringNormalNode>(Cow::Borrowed("string_normal")) {
            return Ok(Self::StringNormal(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringRawNode>(Cow::Borrowed("string_raw")) {
            return Ok(Self::StringRaw(s));
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
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for StringItemNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::EscapedCharacter(s) => s.get_range(),
            Self::EscapedUnicode(s) => s.get_range(),
            Self::TextAny(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<EscapedCharacterNode>(Cow::Borrowed("escaped_character")) {
            return Ok(Self::EscapedCharacter(s));
        }
        if let Ok(s) = pair.take_tagged_one::<EscapedUnicodeNode>(Cow::Borrowed("escaped_unicode")) {
            return Ok(Self::EscapedUnicode(s));
        }
        if let Ok(s) = pair.take_tagged_one::<TextAnyNode>(Cow::Borrowed("text_any")) {
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
        if let Ok(s) = pair.take_tagged_one::<EscapedCharacterNode>(Cow::Borrowed("escaped_character")) {
            return Ok(Self::EscapedCharacter(s));
        }
        if let Ok(s) = pair.take_tagged_one::<RegexCharacterNode>(Cow::Borrowed("regex_character")) {
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
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for BooleanNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::False => Range::default(),
            Self::True => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("false") {
            return Ok(Self::False);
        }
        if let Some(_) = pair.find_first_tag("true") {
            return Ok(Self::True);
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
#[automatically_derived]
impl<'i> YggdrasilNode<'i> for KwExternalNode<'i> {
    type Rule = BootstrapRule;

    fn get_range(&self) -> Range<usize> {
        match self {
            Self::External => Range::default(),
            Self::Inspector => Range::default(),
            Self::Parser => Range::default(),
        }
    }
    fn from_pair(pair: TokenPair<'i, Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("external") {
            return Ok(Self::External);
        }
        if let Some(_) = pair.find_first_tag("inspector") {
            return Ok(Self::Inspector);
        }
        if let Some(_) = pair.find_first_tag("parser") {
            return Ok(Self::Parser);
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
