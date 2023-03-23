// Generated from YggdrasilAntlr.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use super::{yggdrasilantlrlistener::*, yggdrasilantlrvisitor::*};
use antlr_rust::{
    atn::{ATN, INVALID_ALT},
    atn_deserializer::ATNDeserializer,
    dfa::DFA,
    error_strategy::{DefaultErrorStrategy, ErrorStrategy},
    errors::*,
    int_stream::EOF,
    parser::{BaseParser, Parser, ParserNodeType, ParserRecog},
    parser_atn_simulator::ParserATNSimulator,
    parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext},
    recognizer::{Actions, Recognizer},
    rule_context::{BaseRuleContext, CustomRuleContext, RuleContext},
    token::{OwningToken, Token, TOKEN_EOF},
    token_factory::{CommonTokenFactory, TokenAware, TokenFactory},
    token_stream::TokenStream,
    tree::*,
    vocabulary::{Vocabulary, VocabularyImpl},
    PredictionContextCache, TokenSource,
};

use antlr_rust::{lazy_static, TidAble, TidExt};

use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    convert::TryFrom,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

pub const DOT: isize = 1;
pub const COMMA: isize = 2;
pub const COLON: isize = 3;
pub const SEMICOLON: isize = 4;
pub const MATCH_MANY: isize = 5;
pub const MATCH_MANY1: isize = 6;
pub const MATCH_OPTIONAL: isize = 7;
pub const OP_NOT: isize = 8;
pub const OP_AT: isize = 9;
pub const OP_HASH: isize = 10;
pub const OP_CONCAT: isize = 11;
pub const OP_OR: isize = 12;
pub const OP_GT: isize = 13;
pub const OP_UNTAG: isize = 14;
pub const OP_TO: isize = 15;
pub const KW_GRAMMAR: isize = 16;
pub const KW_IMPORT: isize = 17;
pub const KW_AS: isize = 18;
pub const KW_CLASS: isize = 19;
pub const KW_UNION: isize = 20;
pub const KW_CLIMB: isize = 21;
pub const KW_TOKEN: isize = 22;
pub const KW_EXTERNAL: isize = 23;
pub const KW_INSPECTOR: isize = 24;
pub const OP_PROPORTION: isize = 25;
pub const PARENTHESES_L: isize = 26;
pub const PARENTHESES_R: isize = 27;
pub const BRACKET_L: isize = 28;
pub const BRACKET_R: isize = 29;
pub const BRACE_L: isize = 30;
pub const BRACE_R: isize = 31;
pub const INTEGER: isize = 32;
pub const BOOLEAN: isize = 33;
pub const ESCAPED: isize = 34;
pub const REGEX_RANGE: isize = 35;
pub const REGEX_FREE: isize = 36;
pub const STRING_SINGLE: isize = 37;
pub const STRING_DOUBLE: isize = 38;
pub const RAW_ID: isize = 39;
pub const UNICODE_ID: isize = 40;
pub const WHITE_SPACE: isize = 41;
pub const LINE_COMMENT: isize = 42;
pub const BLOCK_COMMENT: isize = 43;
pub const ERROR_CHARACTAR: isize = 44;
pub const RULE_program: usize = 0;
pub const RULE_import_statement: usize = 1;
pub const RULE_import_block: usize = 2;
pub const RULE_define_grammar: usize = 3;
pub const RULE_grammar_block: usize = 4;
pub const RULE_grammar_pair: usize = 5;
pub const RULE_grammar_key: usize = 6;
pub const RULE_grammar_value: usize = 7;
pub const RULE_define_class: usize = 8;
pub const RULE_class_block: usize = 9;
pub const RULE_class_expression: usize = 10;
pub const RULE_define_union: usize = 11;
pub const RULE_union_block: usize = 12;
pub const RULE_union_term: usize = 13;
pub const RULE_union_expression: usize = 14;
pub const RULE_define_climb: usize = 15;
pub const RULE_tag_branch: usize = 16;
pub const RULE_define_token: usize = 17;
pub const RULE_token_block: usize = 18;
pub const RULE_token_pair: usize = 19;
pub const RULE_token_expression: usize = 20;
pub const RULE_define_external: usize = 21;
pub const RULE_external_block: usize = 22;
pub const RULE_external_pair: usize = 23;
pub const RULE_define_inspector: usize = 24;
pub const RULE_annotation: usize = 25;
pub const RULE_modifiers: usize = 26;
pub const RULE_macro_call: usize = 27;
pub const RULE_tuple_block: usize = 28;
pub const RULE_suffix: usize = 29;
pub const RULE_atomic: usize = 30;
pub const RULE_regex: usize = 31;
pub const RULE_namepath: usize = 32;
pub const RULE_string: usize = 33;
pub const RULE_identifier: usize = 34;
pub const ruleNames: [&'static str; 35] = [
    "program",
    "import_statement",
    "import_block",
    "define_grammar",
    "grammar_block",
    "grammar_pair",
    "grammar_key",
    "grammar_value",
    "define_class",
    "class_block",
    "class_expression",
    "define_union",
    "union_block",
    "union_term",
    "union_expression",
    "define_climb",
    "tag_branch",
    "define_token",
    "token_block",
    "token_pair",
    "token_expression",
    "define_external",
    "external_block",
    "external_pair",
    "define_inspector",
    "annotation",
    "modifiers",
    "macro_call",
    "tuple_block",
    "suffix",
    "atomic",
    "regex",
    "namepath",
    "string",
    "identifier",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 32] = [
    None,
    Some("'.'"),
    Some("','"),
    Some("':'"),
    Some("';'"),
    Some("'*'"),
    Some("'+'"),
    Some("'?'"),
    Some("'!'"),
    Some("'@'"),
    Some("'#'"),
    Some("'~'"),
    Some("'|'"),
    Some("'>'"),
    Some("'^'"),
    Some("'->'"),
    Some("'grammar'"),
    Some("'using'"),
    Some("'as'"),
    None,
    None,
    Some("'climb'"),
    Some("'token'"),
    None,
    None,
    Some("'::'"),
    Some("'('"),
    Some("')'"),
    Some("'['"),
    Some("']'"),
    Some("'{'"),
    Some("'}'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 45] = [
    None,
    Some("DOT"),
    Some("COMMA"),
    Some("COLON"),
    Some("SEMICOLON"),
    Some("MATCH_MANY"),
    Some("MATCH_MANY1"),
    Some("MATCH_OPTIONAL"),
    Some("OP_NOT"),
    Some("OP_AT"),
    Some("OP_HASH"),
    Some("OP_CONCAT"),
    Some("OP_OR"),
    Some("OP_GT"),
    Some("OP_UNTAG"),
    Some("OP_TO"),
    Some("KW_GRAMMAR"),
    Some("KW_IMPORT"),
    Some("KW_AS"),
    Some("KW_CLASS"),
    Some("KW_UNION"),
    Some("KW_CLIMB"),
    Some("KW_TOKEN"),
    Some("KW_EXTERNAL"),
    Some("KW_INSPECTOR"),
    Some("OP_PROPORTION"),
    Some("PARENTHESES_L"),
    Some("PARENTHESES_R"),
    Some("BRACKET_L"),
    Some("BRACKET_R"),
    Some("BRACE_L"),
    Some("BRACE_R"),
    Some("INTEGER"),
    Some("BOOLEAN"),
    Some("ESCAPED"),
    Some("REGEX_RANGE"),
    Some("REGEX_FREE"),
    Some("STRING_SINGLE"),
    Some("STRING_DOUBLE"),
    Some("RAW_ID"),
    Some("UNICODE_ID"),
    Some("WHITE_SPACE"),
    Some("LINE_COMMENT"),
    Some("BLOCK_COMMENT"),
    Some("ERROR_CHARACTAR"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> =
        Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    YggdrasilAntlrParserExt<'input>,
    I,
    YggdrasilAntlrParserContextType,
    dyn YggdrasilAntlrListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type YggdrasilAntlrTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, YggdrasilAntlrParserContextType, dyn YggdrasilAntlrListener<'input> + 'a>;

/// Parser for YggdrasilAntlr grammar
pub struct YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter =
            Arc::new(ParserATNSimulator::new(_ATN.clone(), _decision_to_DFA.clone(), _shared_context_cache.clone()));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                YggdrasilAntlrParserExt { _pd: Default::default() },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> YggdrasilAntlrParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> YggdrasilAntlrParser<'input, I, DefaultErrorStrategy<'input, YggdrasilAntlrParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for YggdrasilAntlrParser
pub trait YggdrasilAntlrParserContext<'input>:
    for<'x> Listenable<dyn YggdrasilAntlrListener<'input> + 'x>
    + for<'x> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = YggdrasilAntlrParserContextType>
{
}

antlr_rust::coerce_from! { 'input : YggdrasilAntlrParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn YggdrasilAntlrParserContext<'input> + 'input
where
    T: YggdrasilAntlrVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn YggdrasilAntlrVisitor<'input> + 'x))
    }
}

impl<'input> YggdrasilAntlrParserContext<'input> for TerminalNode<'input, YggdrasilAntlrParserContextType> {}

impl<'input> YggdrasilAntlrParserContext<'input> for ErrorNode<'input, YggdrasilAntlrParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn YggdrasilAntlrParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn YggdrasilAntlrListener<'input> + 'input }

pub struct YggdrasilAntlrParserContextType;
antlr_rust::tid! {YggdrasilAntlrParserContextType}

impl<'input> ParserNodeType<'input> for YggdrasilAntlrParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn YggdrasilAntlrParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct YggdrasilAntlrParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserExt<'input> {}
antlr_rust::tid! { YggdrasilAntlrParserExt<'a> }

impl<'input> TokenAware<'input> for YggdrasilAntlrParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for YggdrasilAntlrParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for YggdrasilAntlrParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "YggdrasilAntlr.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn YggdrasilAntlrParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            10 => YggdrasilAntlrParser::<'input, I, _>::class_expression_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            14 => YggdrasilAntlrParser::<'input, I, _>::union_expression_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            20 => YggdrasilAntlrParser::<'input, I, _>::token_expression_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> YggdrasilAntlrParser<'input, I, DefaultErrorStrategy<'input, YggdrasilAntlrParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn class_expression_sempred(
        _localctx: Option<&Class_expressionContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 4),
            1 => recog.precpred(None, 3),
            2 => recog.precpred(None, 2),
            3 => recog.precpred(None, 8),
            _ => true,
        }
    }
    fn union_expression_sempred(
        _localctx: Option<&Union_expressionContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            4 => recog.precpred(None, 3),
            5 => recog.precpred(None, 2),
            6 => recog.precpred(None, 7),
            _ => true,
        }
    }
    fn token_expression_sempred(
        _localctx: Option<&Token_expressionContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            7 => recog.precpred(None, 2),
            _ => true,
        }
    }
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;

pub type ProgramContext<'input> = BaseParserRuleContext<'input, ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for ProgramContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for ProgramContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_program(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_program(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for ProgramContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_program(self);
    }
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_program
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid! {ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ProgramContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, ProgramContextExt { ph: PhantomData }))
    }
}

pub trait ProgramContextAttrs<'input>: YggdrasilAntlrParserContext<'input> + BorrowMut<ProgramContextExt<'input>> {
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    fn define_grammar_all(&self) -> Vec<Rc<Define_grammarContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn define_grammar(&self, i: usize) -> Option<Rc<Define_grammarContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn import_statement_all(&self) -> Vec<Rc<Import_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn import_statement(&self, i: usize) -> Option<Rc<Import_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn define_class_all(&self) -> Vec<Rc<Define_classContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn define_class(&self, i: usize) -> Option<Rc<Define_classContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn define_union_all(&self) -> Vec<Rc<Define_unionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn define_union(&self, i: usize) -> Option<Rc<Define_unionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn define_climb_all(&self) -> Vec<Rc<Define_climbContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn define_climb(&self, i: usize) -> Option<Rc<Define_climbContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn define_token_all(&self) -> Vec<Rc<Define_tokenContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn define_token(&self, i: usize) -> Option<Rc<Define_tokenContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn define_external_all(&self) -> Vec<Rc<Define_externalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn define_external(&self, i: usize) -> Option<Rc<Define_externalContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn define_inspector_all(&self) -> Vec<Rc<Define_inspectorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn define_inspector(&self, i: usize) -> Option<Rc<Define_inspectorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
    fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
    fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, i)
    }
}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn program(&mut self) -> Result<Rc<ProgramContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(81);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << SEMICOLON)
                            | (1usize << OP_AT)
                            | (1usize << OP_HASH)
                            | (1usize << KW_GRAMMAR)
                            | (1usize << KW_IMPORT)
                            | (1usize << KW_CLASS)
                            | (1usize << KW_UNION)
                            | (1usize << KW_CLIMB)
                            | (1usize << KW_TOKEN)
                            | (1usize << KW_EXTERNAL)
                            | (1usize << KW_INSPECTOR)))
                        != 0)
                    || _la == RAW_ID
                    || _la == UNICODE_ID
                {
                    {
                        recog.base.set_state(79);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(0, &mut recog.base)? {
                            1 => {
                                {
                                    // InvokeRule define_grammar
                                    recog.base.set_state(70);
                                    recog.define_grammar()?;
                                }
                            }
                            2 => {
                                {
                                    // InvokeRule import_statement
                                    recog.base.set_state(71);
                                    recog.import_statement()?;
                                }
                            }
                            3 => {
                                {
                                    // InvokeRule define_class
                                    recog.base.set_state(72);
                                    recog.define_class()?;
                                }
                            }
                            4 => {
                                {
                                    // InvokeRule define_union
                                    recog.base.set_state(73);
                                    recog.define_union()?;
                                }
                            }
                            5 => {
                                {
                                    // InvokeRule define_climb
                                    recog.base.set_state(74);
                                    recog.define_climb()?;
                                }
                            }
                            6 => {
                                {
                                    // InvokeRule define_token
                                    recog.base.set_state(75);
                                    recog.define_token()?;
                                }
                            }
                            7 => {
                                {
                                    // InvokeRule define_external
                                    recog.base.set_state(76);
                                    recog.define_external()?;
                                }
                            }
                            8 => {
                                {
                                    // InvokeRule define_inspector
                                    recog.base.set_state(77);
                                    recog.define_inspector()?;
                                }
                            }
                            9 => {
                                recog.base.set_state(78);
                                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
                            }

                            _ => {}
                        }
                    }
                    recog.base.set_state(83);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(84);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- import_statement ----------------
pub type Import_statementContextAll<'input> = Import_statementContext<'input>;

pub type Import_statementContext<'input> = BaseParserRuleContext<'input, Import_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Import_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Import_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Import_statementContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_import_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_import_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Import_statementContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_import_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for Import_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_import_statement
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_import_statement }
}
antlr_rust::tid! {Import_statementContextExt<'a>}

impl<'input> Import_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Import_statementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Import_statementContextExt { ph: PhantomData }))
    }
}

pub trait Import_statementContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Import_statementContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token KW_IMPORT
    /// Returns `None` if there is no child corresponding to token KW_IMPORT
    fn KW_IMPORT(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_IMPORT, 0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn import_block(&self) -> Option<Rc<Import_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Import_statementContextAttrs<'input> for Import_statementContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn import_statement(&mut self) -> Result<Rc<Import_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Import_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_import_statement);
        let mut _localctx: Rc<Import_statementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(86);
                recog.base.match_token(KW_IMPORT, &mut recog.err_handler)?;

                recog.base.set_state(89);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    RAW_ID | UNICODE_ID => {
                        {
                            // InvokeRule identifier
                            recog.base.set_state(87);
                            recog.identifier()?;
                        }
                    }

                    STRING_SINGLE | STRING_DOUBLE => {
                        {
                            // InvokeRule string
                            recog.base.set_state(88);
                            recog.string()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?,
                }
                recog.base.set_state(92);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == BRACE_L {
                    {
                        // InvokeRule import_block
                        recog.base.set_state(91);
                        recog.import_block()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- import_block ----------------
pub type Import_blockContextAll<'input> = Import_blockContext<'input>;

pub type Import_blockContext<'input> = BaseParserRuleContext<'input, Import_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Import_blockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Import_blockContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Import_blockContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_import_block(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_import_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Import_blockContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_import_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for Import_blockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_import_block
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_import_block }
}
antlr_rust::tid! {Import_blockContextExt<'a>}

impl<'input> Import_blockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Import_blockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Import_blockContextExt { ph: PhantomData }))
    }
}

pub trait Import_blockContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Import_blockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BRACE_L
    /// Returns `None` if there is no child corresponding to token BRACE_L
    fn BRACE_L(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_L, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACE_R
    /// Returns `None` if there is no child corresponding to token BRACE_R
    fn BRACE_R(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_R, 0)
    }
    fn identifier_all(&self) -> Vec<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Import_blockContextAttrs<'input> for Import_blockContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn import_block(&mut self) -> Result<Rc<Import_blockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Import_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_import_block);
        let mut _localctx: Rc<Import_blockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(94);
                recog.base.match_token(BRACE_L, &mut recog.err_handler)?;

                recog.base.set_state(98);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == RAW_ID || _la == UNICODE_ID {
                    {
                        {
                            // InvokeRule identifier
                            recog.base.set_state(95);
                            recog.identifier()?;
                        }
                    }
                    recog.base.set_state(100);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(101);
                recog.base.match_token(BRACE_R, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- define_grammar ----------------
pub type Define_grammarContextAll<'input> = Define_grammarContext<'input>;

pub type Define_grammarContext<'input> = BaseParserRuleContext<'input, Define_grammarContextExt<'input>>;

#[derive(Clone)]
pub struct Define_grammarContextExt<'input> {
    pub parent: Option<Rc<IdentifierContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Define_grammarContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Define_grammarContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_define_grammar(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_define_grammar(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Define_grammarContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_define_grammar(self);
    }
}

impl<'input> CustomRuleContext<'input> for Define_grammarContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_define_grammar
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_define_grammar }
}
antlr_rust::tid! {Define_grammarContextExt<'a>}

impl<'input> Define_grammarContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Define_grammarContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Define_grammarContextExt { parent: None, ph: PhantomData },
        ))
    }
}

pub trait Define_grammarContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Define_grammarContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token KW_GRAMMAR
    /// Returns `None` if there is no child corresponding to token KW_GRAMMAR
    fn KW_GRAMMAR(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_GRAMMAR, 0)
    }
    fn identifier_all(&self) -> Vec<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn grammar_block(&self) -> Option<Rc<Grammar_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
}

impl<'input> Define_grammarContextAttrs<'input> for Define_grammarContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn define_grammar(&mut self) -> Result<Rc<Define_grammarContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Define_grammarContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_define_grammar);
        let mut _localctx: Rc<Define_grammarContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(103);
                recog.base.match_token(KW_GRAMMAR, &mut recog.err_handler)?;

                // InvokeRule identifier
                recog.base.set_state(104);
                recog.identifier()?;

                recog.base.set_state(107);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COLON {
                    {
                        recog.base.set_state(105);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        // InvokeRule identifier
                        recog.base.set_state(106);
                        let tmp = recog.identifier()?;
                        cast_mut::<_, Define_grammarContext>(&mut _localctx).parent = Some(tmp.clone());
                    }
                }

                // InvokeRule grammar_block
                recog.base.set_state(109);
                recog.grammar_block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- grammar_block ----------------
pub type Grammar_blockContextAll<'input> = Grammar_blockContext<'input>;

pub type Grammar_blockContext<'input> = BaseParserRuleContext<'input, Grammar_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Grammar_blockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Grammar_blockContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Grammar_blockContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_grammar_block(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_grammar_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Grammar_blockContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_grammar_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for Grammar_blockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_grammar_block
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_grammar_block }
}
antlr_rust::tid! {Grammar_blockContextExt<'a>}

impl<'input> Grammar_blockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Grammar_blockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Grammar_blockContextExt { ph: PhantomData }))
    }
}

pub trait Grammar_blockContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Grammar_blockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BRACE_L
    /// Returns `None` if there is no child corresponding to token BRACE_L
    fn BRACE_L(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_L, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACE_R
    /// Returns `None` if there is no child corresponding to token BRACE_R
    fn BRACE_R(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_R, 0)
    }
    fn grammar_pair_all(&self) -> Vec<Rc<Grammar_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn grammar_pair(&self, i: usize) -> Option<Rc<Grammar_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
    fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
    fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Grammar_blockContextAttrs<'input> for Grammar_blockContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn grammar_block(&mut self) -> Result<Rc<Grammar_blockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Grammar_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_grammar_block);
        let mut _localctx: Rc<Grammar_blockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(111);
                recog.base.match_token(BRACE_L, &mut recog.err_handler)?;

                recog.base.set_state(117);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA
                    || _la == SEMICOLON
                    || (((_la - 37) & !0x3f) == 0
                        && ((1usize << (_la - 37))
                            & ((1usize << (STRING_SINGLE - 37))
                                | (1usize << (STRING_DOUBLE - 37))
                                | (1usize << (RAW_ID - 37))
                                | (1usize << (UNICODE_ID - 37))))
                            != 0)
                {
                    {
                        recog.base.set_state(115);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            STRING_SINGLE | STRING_DOUBLE | RAW_ID | UNICODE_ID => {
                                {
                                    // InvokeRule grammar_pair
                                    recog.base.set_state(112);
                                    recog.grammar_pair()?;
                                }
                            }

                            SEMICOLON => {
                                recog.base.set_state(113);
                                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
                            }

                            COMMA => {
                                recog.base.set_state(114);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?,
                        }
                    }
                    recog.base.set_state(119);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(120);
                recog.base.match_token(BRACE_R, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

//------------------- grammar_pair ----------------
pub type Grammar_pairContextAll<'input> = Grammar_pairContext<'input>;

pub type Grammar_pairContext<'input> = BaseParserRuleContext<'input, Grammar_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Grammar_pairContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Grammar_pairContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Grammar_pairContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_grammar_pair(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_grammar_pair(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Grammar_pairContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_grammar_pair(self);
    }
}

impl<'input> CustomRuleContext<'input> for Grammar_pairContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_grammar_pair
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_grammar_pair }
}
antlr_rust::tid! {Grammar_pairContextExt<'a>}

impl<'input> Grammar_pairContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Grammar_pairContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Grammar_pairContextExt { ph: PhantomData }))
    }
}

pub trait Grammar_pairContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Grammar_pairContextExt<'input>>
{
    fn grammar_key(&self) -> Option<Rc<Grammar_keyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn grammar_value(&self) -> Option<Rc<Grammar_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Grammar_pairContextAttrs<'input> for Grammar_pairContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn grammar_pair(&mut self) -> Result<Rc<Grammar_pairContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Grammar_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_grammar_pair);
        let mut _localctx: Rc<Grammar_pairContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                // InvokeRule grammar_key
                recog.base.set_state(122);
                recog.grammar_key()?;

                recog.base.set_state(123);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                // InvokeRule grammar_value
                recog.base.set_state(124);
                recog.grammar_value()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

//------------------- grammar_key ----------------
pub type Grammar_keyContextAll<'input> = Grammar_keyContext<'input>;

pub type Grammar_keyContext<'input> = BaseParserRuleContext<'input, Grammar_keyContextExt<'input>>;

#[derive(Clone)]
pub struct Grammar_keyContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Grammar_keyContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Grammar_keyContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_grammar_key(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_grammar_key(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Grammar_keyContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_grammar_key(self);
    }
}

impl<'input> CustomRuleContext<'input> for Grammar_keyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_grammar_key
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_grammar_key }
}
antlr_rust::tid! {Grammar_keyContextExt<'a>}

impl<'input> Grammar_keyContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Grammar_keyContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Grammar_keyContextExt { ph: PhantomData }))
    }
}

pub trait Grammar_keyContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Grammar_keyContextExt<'input>>
{
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Grammar_keyContextAttrs<'input> for Grammar_keyContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn grammar_key(&mut self) -> Result<Rc<Grammar_keyContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Grammar_keyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_grammar_key);
        let mut _localctx: Rc<Grammar_keyContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(128);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                STRING_SINGLE | STRING_DOUBLE => {
                    // recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        // InvokeRule string
                        recog.base.set_state(126);
                        recog.string()?;
                    }
                }

                RAW_ID | UNICODE_ID => {
                    // recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        // InvokeRule identifier
                        recog.base.set_state(127);
                        recog.identifier()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

//------------------- grammar_value ----------------
pub type Grammar_valueContextAll<'input> = Grammar_valueContext<'input>;

pub type Grammar_valueContext<'input> = BaseParserRuleContext<'input, Grammar_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Grammar_valueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Grammar_valueContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Grammar_valueContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_grammar_value(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_grammar_value(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Grammar_valueContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_grammar_value(self);
    }
}

impl<'input> CustomRuleContext<'input> for Grammar_valueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_grammar_value
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_grammar_value }
}
antlr_rust::tid! {Grammar_valueContextExt<'a>}

impl<'input> Grammar_valueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Grammar_valueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Grammar_valueContextExt { ph: PhantomData }))
    }
}

pub trait Grammar_valueContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Grammar_valueContextExt<'input>>
{
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn namepath(&self) -> Option<Rc<NamepathContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BOOLEAN
    /// Returns `None` if there is no child corresponding to token BOOLEAN
    fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BOOLEAN, 0)
    }
}

impl<'input> Grammar_valueContextAttrs<'input> for Grammar_valueContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn grammar_value(&mut self) -> Result<Rc<Grammar_valueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Grammar_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_grammar_value);
        let mut _localctx: Rc<Grammar_valueContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(133);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                STRING_SINGLE | STRING_DOUBLE => {
                    // recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        // InvokeRule string
                        recog.base.set_state(130);
                        recog.string()?;
                    }
                }

                RAW_ID | UNICODE_ID => {
                    // recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        // InvokeRule namepath
                        recog.base.set_state(131);
                        recog.namepath()?;
                    }
                }

                BOOLEAN => {
                    // recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(132);
                        recog.base.match_token(BOOLEAN, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- define_class ----------------
pub type Define_classContextAll<'input> = Define_classContext<'input>;

pub type Define_classContext<'input> = BaseParserRuleContext<'input, Define_classContextExt<'input>>;

#[derive(Clone)]
pub struct Define_classContextExt<'input> {
    pub name: Option<Rc<IdentifierContextAll<'input>>>,
    pub cast: Option<Rc<IdentifierContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Define_classContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Define_classContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_define_class(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_define_class(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Define_classContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_define_class(self);
    }
}

impl<'input> CustomRuleContext<'input> for Define_classContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_define_class
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_define_class }
}
antlr_rust::tid! {Define_classContextExt<'a>}

impl<'input> Define_classContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Define_classContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Define_classContextExt { name: None, cast: None, ph: PhantomData },
        ))
    }
}

pub trait Define_classContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Define_classContextExt<'input>>
{
    fn modifiers(&self) -> Option<Rc<ModifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token KW_CLASS
    /// Returns `None` if there is no child corresponding to token KW_CLASS
    fn KW_CLASS(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_CLASS, 0)
    }
    fn class_block(&self) -> Option<Rc<Class_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn identifier_all(&self) -> Vec<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn annotation_all(&self) -> Vec<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OP_TO
    /// Returns `None` if there is no child corresponding to token OP_TO
    fn OP_TO(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_TO, 0)
    }
}

impl<'input> Define_classContextAttrs<'input> for Define_classContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn define_class(&mut self) -> Result<Rc<Define_classContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Define_classContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_define_class);
        let mut _localctx: Rc<Define_classContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(138);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_AT || _la == OP_HASH {
                    {
                        {
                            // InvokeRule annotation
                            recog.base.set_state(135);
                            recog.annotation()?;
                        }
                    }
                    recog.base.set_state(140);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                // InvokeRule modifiers
                recog.base.set_state(141);
                recog.modifiers()?;

                recog.base.set_state(142);
                recog.base.match_token(KW_CLASS, &mut recog.err_handler)?;

                // InvokeRule identifier
                recog.base.set_state(143);
                let tmp = recog.identifier()?;
                cast_mut::<_, Define_classContext>(&mut _localctx).name = Some(tmp.clone());

                recog.base.set_state(146);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OP_TO {
                    {
                        recog.base.set_state(144);
                        recog.base.match_token(OP_TO, &mut recog.err_handler)?;

                        // InvokeRule identifier
                        recog.base.set_state(145);
                        let tmp = recog.identifier()?;
                        cast_mut::<_, Define_classContext>(&mut _localctx).cast = Some(tmp.clone());
                    }
                }

                // InvokeRule class_block
                recog.base.set_state(148);
                recog.class_block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- class_block ----------------
pub type Class_blockContextAll<'input> = Class_blockContext<'input>;

pub type Class_blockContext<'input> = BaseParserRuleContext<'input, Class_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Class_blockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Class_blockContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Class_blockContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_class_block(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_class_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Class_blockContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_class_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for Class_blockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_block
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_block }
}
antlr_rust::tid! {Class_blockContextExt<'a>}

impl<'input> Class_blockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Class_blockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Class_blockContextExt { ph: PhantomData }))
    }
}

pub trait Class_blockContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Class_blockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BRACE_L
    /// Returns `None` if there is no child corresponding to token BRACE_L
    fn BRACE_L(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_L, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACE_R
    /// Returns `None` if there is no child corresponding to token BRACE_R
    fn BRACE_R(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_R, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OP_OR
    /// Returns `None` if there is no child corresponding to token OP_OR
    fn OP_OR(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_OR, 0)
    }
    fn class_expression_all(&self) -> Vec<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn class_expression(&self, i: usize) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Class_blockContextAttrs<'input> for Class_blockContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn class_block(&mut self) -> Result<Rc<Class_blockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Class_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_class_block);
        let mut _localctx: Rc<Class_blockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(150);
                recog.base.match_token(BRACE_L, &mut recog.err_handler)?;

                recog.base.set_state(152);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OP_OR {
                    {
                        recog.base.set_state(151);
                        recog.base.match_token(OP_OR, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(157);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << OP_NOT) | (1usize << OP_AT) | (1usize << OP_UNTAG) | (1usize << PARENTHESES_L)))
                        != 0)
                    || (((_la - 32) & !0x3f) == 0
                        && ((1usize << (_la - 32))
                            & ((1usize << (INTEGER - 32))
                                | (1usize << (BOOLEAN - 32))
                                | (1usize << (ESCAPED - 32))
                                | (1usize << (REGEX_RANGE - 32))
                                | (1usize << (REGEX_FREE - 32))
                                | (1usize << (STRING_SINGLE - 32))
                                | (1usize << (STRING_DOUBLE - 32))
                                | (1usize << (RAW_ID - 32))
                                | (1usize << (UNICODE_ID - 32))))
                            != 0)
                {
                    {
                        {
                            // InvokeRule class_expression
                            recog.base.set_state(154);
                            recog.class_expression_rec(0)?;
                        }
                    }
                    recog.base.set_state(159);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(160);
                recog.base.match_token(BRACE_R, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- class_expression ----------------
#[derive(Debug)]
pub enum Class_expressionContextAll<'input> {
    CSuffixContext(CSuffixContext<'input>),
    CETagContext(CETagContext<'input>),
    CUntagContext(CUntagContext<'input>),
    CSoftContext(CSoftContext<'input>),
    CHardContext(CHardContext<'input>),
    CPatternContext(CPatternContext<'input>),
    AtomContext(AtomContext<'input>),
    CNotContext(CNotContext<'input>),
    Error(Class_expressionContext<'input>),
}
antlr_rust::tid! {Class_expressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Class_expressionContextAll<'input> {}

impl<'input> YggdrasilAntlrParserContext<'input> for Class_expressionContextAll<'input> {}

impl<'input> Deref for Class_expressionContextAll<'input> {
    type Target = dyn Class_expressionContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Class_expressionContextAll::*;
        match self {
            CSuffixContext(inner) => inner,
            CETagContext(inner) => inner,
            CUntagContext(inner) => inner,
            CSoftContext(inner) => inner,
            CHardContext(inner) => inner,
            CPatternContext(inner) => inner,
            AtomContext(inner) => inner,
            CNotContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Class_expressionContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Class_expressionContextAll<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Class_expressionContext<'input> = BaseParserRuleContext<'input, Class_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Class_expressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Class_expressionContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Class_expressionContext<'input> {}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Class_expressionContext<'input> {}

impl<'input> CustomRuleContext<'input> for Class_expressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}
antlr_rust::tid! {Class_expressionContextExt<'a>}

impl<'input> Class_expressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::Error(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Class_expressionContextExt { ph: PhantomData },
        )))
    }
}

pub trait Class_expressionContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Class_expressionContextExt<'input>>
{
}

impl<'input> Class_expressionContextAttrs<'input> for Class_expressionContext<'input> {}

pub type CSuffixContext<'input> = BaseParserRuleContext<'input, CSuffixContextExt<'input>>;

pub trait CSuffixContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn class_expression(&self) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn suffix(&self) -> Option<Rc<SuffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> CSuffixContextAttrs<'input> for CSuffixContext<'input> {}

pub struct CSuffixContextExt<'input> {
    base: Class_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CSuffixContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for CSuffixContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for CSuffixContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_CSuffix(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_CSuffix(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for CSuffixContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_CSuffix(self);
    }
}

impl<'input> CustomRuleContext<'input> for CSuffixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}

impl<'input> Borrow<Class_expressionContextExt<'input>> for CSuffixContext<'input> {
    fn borrow(&self) -> &Class_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Class_expressionContextExt<'input>> for CSuffixContext<'input> {
    fn borrow_mut(&mut self) -> &mut Class_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Class_expressionContextAttrs<'input> for CSuffixContext<'input> {}

impl<'input> CSuffixContextExt<'input> {
    fn new(ctx: &dyn Class_expressionContextAttrs<'input>) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::CSuffixContext(BaseParserRuleContext::copy_from(
            ctx,
            CSuffixContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type CETagContext<'input> = BaseParserRuleContext<'input, CETagContextExt<'input>>;

pub trait CETagContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn class_expression(&self) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> CETagContextAttrs<'input> for CETagContext<'input> {}

pub struct CETagContextExt<'input> {
    base: Class_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CETagContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for CETagContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for CETagContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_CETag(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_CETag(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for CETagContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_CETag(self);
    }
}

impl<'input> CustomRuleContext<'input> for CETagContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}

impl<'input> Borrow<Class_expressionContextExt<'input>> for CETagContext<'input> {
    fn borrow(&self) -> &Class_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Class_expressionContextExt<'input>> for CETagContext<'input> {
    fn borrow_mut(&mut self) -> &mut Class_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Class_expressionContextAttrs<'input> for CETagContext<'input> {}

impl<'input> CETagContextExt<'input> {
    fn new(ctx: &dyn Class_expressionContextAttrs<'input>) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::CETagContext(BaseParserRuleContext::copy_from(
            ctx,
            CETagContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type CUntagContext<'input> = BaseParserRuleContext<'input, CUntagContextExt<'input>>;

pub trait CUntagContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token OP_UNTAG
    /// Returns `None` if there is no child corresponding to token OP_UNTAG
    fn OP_UNTAG(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_UNTAG, 0)
    }
    fn class_expression(&self) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> CUntagContextAttrs<'input> for CUntagContext<'input> {}

pub struct CUntagContextExt<'input> {
    base: Class_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CUntagContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for CUntagContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for CUntagContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_CUntag(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_CUntag(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for CUntagContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_CUntag(self);
    }
}

impl<'input> CustomRuleContext<'input> for CUntagContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}

impl<'input> Borrow<Class_expressionContextExt<'input>> for CUntagContext<'input> {
    fn borrow(&self) -> &Class_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Class_expressionContextExt<'input>> for CUntagContext<'input> {
    fn borrow_mut(&mut self) -> &mut Class_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Class_expressionContextAttrs<'input> for CUntagContext<'input> {}

impl<'input> CUntagContextExt<'input> {
    fn new(ctx: &dyn Class_expressionContextAttrs<'input>) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::CUntagContext(BaseParserRuleContext::copy_from(
            ctx,
            CUntagContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type CSoftContext<'input> = BaseParserRuleContext<'input, CSoftContextExt<'input>>;

pub trait CSoftContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn class_expression_all(&self) -> Vec<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn class_expression(&self, i: usize) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> CSoftContextAttrs<'input> for CSoftContext<'input> {}

pub struct CSoftContextExt<'input> {
    base: Class_expressionContextExt<'input>,
    pub lhs: Option<Rc<Class_expressionContextAll<'input>>>,
    pub rhs: Option<Rc<Class_expressionContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CSoftContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for CSoftContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for CSoftContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_CSoft(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_CSoft(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for CSoftContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_CSoft(self);
    }
}

impl<'input> CustomRuleContext<'input> for CSoftContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}

impl<'input> Borrow<Class_expressionContextExt<'input>> for CSoftContext<'input> {
    fn borrow(&self) -> &Class_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Class_expressionContextExt<'input>> for CSoftContext<'input> {
    fn borrow_mut(&mut self) -> &mut Class_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Class_expressionContextAttrs<'input> for CSoftContext<'input> {}

impl<'input> CSoftContextExt<'input> {
    fn new(ctx: &dyn Class_expressionContextAttrs<'input>) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::CSoftContext(BaseParserRuleContext::copy_from(
            ctx,
            CSoftContextExt { lhs: None, rhs: None, base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type CHardContext<'input> = BaseParserRuleContext<'input, CHardContextExt<'input>>;

pub trait CHardContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token OP_CONCAT
    /// Returns `None` if there is no child corresponding to token OP_CONCAT
    fn OP_CONCAT(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_CONCAT, 0)
    }
    fn class_expression_all(&self) -> Vec<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn class_expression(&self, i: usize) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> CHardContextAttrs<'input> for CHardContext<'input> {}

pub struct CHardContextExt<'input> {
    base: Class_expressionContextExt<'input>,
    pub lhs: Option<Rc<Class_expressionContextAll<'input>>>,
    pub rhs: Option<Rc<Class_expressionContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CHardContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for CHardContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for CHardContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_CHard(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_CHard(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for CHardContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_CHard(self);
    }
}

impl<'input> CustomRuleContext<'input> for CHardContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}

impl<'input> Borrow<Class_expressionContextExt<'input>> for CHardContext<'input> {
    fn borrow(&self) -> &Class_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Class_expressionContextExt<'input>> for CHardContext<'input> {
    fn borrow_mut(&mut self) -> &mut Class_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Class_expressionContextAttrs<'input> for CHardContext<'input> {}

impl<'input> CHardContextExt<'input> {
    fn new(ctx: &dyn Class_expressionContextAttrs<'input>) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::CHardContext(BaseParserRuleContext::copy_from(
            ctx,
            CHardContextExt { lhs: None, rhs: None, base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type CPatternContext<'input> = BaseParserRuleContext<'input, CPatternContextExt<'input>>;

pub trait CPatternContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token OP_OR
    /// Returns `None` if there is no child corresponding to token OP_OR
    fn OP_OR(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_OR, 0)
    }
    fn class_expression_all(&self) -> Vec<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn class_expression(&self, i: usize) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> CPatternContextAttrs<'input> for CPatternContext<'input> {}

pub struct CPatternContextExt<'input> {
    base: Class_expressionContextExt<'input>,
    pub lhs: Option<Rc<Class_expressionContextAll<'input>>>,
    pub rhs: Option<Rc<Class_expressionContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CPatternContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for CPatternContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for CPatternContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_CPattern(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_CPattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for CPatternContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_CPattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for CPatternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}

impl<'input> Borrow<Class_expressionContextExt<'input>> for CPatternContext<'input> {
    fn borrow(&self) -> &Class_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Class_expressionContextExt<'input>> for CPatternContext<'input> {
    fn borrow_mut(&mut self) -> &mut Class_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Class_expressionContextAttrs<'input> for CPatternContext<'input> {}

impl<'input> CPatternContextExt<'input> {
    fn new(ctx: &dyn Class_expressionContextAttrs<'input>) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::CPatternContext(BaseParserRuleContext::copy_from(
            ctx,
            CPatternContextExt { lhs: None, rhs: None, base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type AtomContext<'input> = BaseParserRuleContext<'input, AtomContextExt<'input>>;

pub trait AtomContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn atomic(&self) -> Option<Rc<AtomicContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input> {}

pub struct AtomContextExt<'input> {
    base: Class_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AtomContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for AtomContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AtomContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_Atom(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_Atom(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AtomContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_Atom(self);
    }
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}

impl<'input> Borrow<Class_expressionContextExt<'input>> for AtomContext<'input> {
    fn borrow(&self) -> &Class_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Class_expressionContextExt<'input>> for AtomContext<'input> {
    fn borrow_mut(&mut self) -> &mut Class_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Class_expressionContextAttrs<'input> for AtomContext<'input> {}

impl<'input> AtomContextExt<'input> {
    fn new(ctx: &dyn Class_expressionContextAttrs<'input>) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::AtomContext(BaseParserRuleContext::copy_from(
            ctx,
            AtomContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type CNotContext<'input> = BaseParserRuleContext<'input, CNotContextExt<'input>>;

pub trait CNotContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token OP_NOT
    /// Returns `None` if there is no child corresponding to token OP_NOT
    fn OP_NOT(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_NOT, 0)
    }
    fn class_expression(&self) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> CNotContextAttrs<'input> for CNotContext<'input> {}

pub struct CNotContextExt<'input> {
    base: Class_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {CNotContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for CNotContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for CNotContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_CNot(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_CNot(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for CNotContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_CNot(self);
    }
}

impl<'input> CustomRuleContext<'input> for CNotContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_class_expression }
}

impl<'input> Borrow<Class_expressionContextExt<'input>> for CNotContext<'input> {
    fn borrow(&self) -> &Class_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Class_expressionContextExt<'input>> for CNotContext<'input> {
    fn borrow_mut(&mut self) -> &mut Class_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Class_expressionContextAttrs<'input> for CNotContext<'input> {}

impl<'input> CNotContextExt<'input> {
    fn new(ctx: &dyn Class_expressionContextAttrs<'input>) -> Rc<Class_expressionContextAll<'input>> {
        Rc::new(Class_expressionContextAll::CNotContext(BaseParserRuleContext::copy_from(
            ctx,
            CNotContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn class_expression(&mut self) -> Result<Rc<Class_expressionContextAll<'input>>, ANTLRError> {
        self.class_expression_rec(0)
    }

    fn class_expression_rec(&mut self, _p: isize) -> Result<Rc<Class_expressionContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = Class_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_recursion_rule(_localctx.clone(), 20, RULE_class_expression, _p);
        let mut _localctx: Rc<Class_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 20;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(172);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(14, &mut recog.base)? {
                    1 => {
                        {
                            let mut tmp = CETagContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();

                            // InvokeRule identifier
                            recog.base.set_state(163);
                            recog.identifier()?;

                            recog.base.set_state(164);
                            recog.base.match_token(COLON, &mut recog.err_handler)?;

                            // InvokeRule class_expression
                            recog.base.set_state(165);
                            recog.class_expression_rec(7)?;
                        }
                    }
                    2 => {
                        {
                            let mut tmp = CUntagContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(167);
                            recog.base.match_token(OP_UNTAG, &mut recog.err_handler)?;

                            // InvokeRule class_expression
                            recog.base.set_state(168);
                            recog.class_expression_rec(6)?;
                        }
                    }
                    3 => {
                        {
                            let mut tmp = CNotContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(169);
                            recog.base.match_token(OP_NOT, &mut recog.err_handler)?;

                            // InvokeRule class_expression
                            recog.base.set_state(170);
                            recog.class_expression_rec(5)?;
                        }
                    }
                    4 => {
                        {
                            let mut tmp = AtomContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            // InvokeRule atomic
                            recog.base.set_state(171);
                            recog.atomic()?;
                        }
                    }

                    _ => {}
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(186);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(16, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(184);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(15, &mut recog.base)? {
                                1 => {
                                    {
                                        // recRuleLabeledAltStartAction
                                        let mut tmp = CHardContextExt::new(&**Class_expressionContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        ));
                                        if let Class_expressionContextAll::CHardContext(ctx) =
                                            cast_mut::<_, Class_expressionContextAll>(&mut tmp)
                                        {
                                            ctx.lhs = Some(_prevctx.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_class_expression);
                                        _localctx = tmp;
                                        recog.base.set_state(174);
                                        if !({ recog.precpred(None, 4) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(175);
                                        recog.base.match_token(OP_CONCAT, &mut recog.err_handler)?;

                                        // InvokeRule class_expression
                                        recog.base.set_state(176);
                                        let tmp = recog.class_expression_rec(5)?;
                                        if let Class_expressionContextAll::CHardContext(ctx) =
                                            cast_mut::<_, Class_expressionContextAll>(&mut _localctx)
                                        {
                                            ctx.rhs = Some(tmp.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                    }
                                }
                                2 => {
                                    {
                                        // recRuleLabeledAltStartAction
                                        let mut tmp = CSoftContextExt::new(&**Class_expressionContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        ));
                                        if let Class_expressionContextAll::CSoftContext(ctx) =
                                            cast_mut::<_, Class_expressionContextAll>(&mut tmp)
                                        {
                                            ctx.lhs = Some(_prevctx.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_class_expression);
                                        _localctx = tmp;
                                        recog.base.set_state(177);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        // InvokeRule class_expression
                                        recog.base.set_state(178);
                                        let tmp = recog.class_expression_rec(4)?;
                                        if let Class_expressionContextAll::CSoftContext(ctx) =
                                            cast_mut::<_, Class_expressionContextAll>(&mut _localctx)
                                        {
                                            ctx.rhs = Some(tmp.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                    }
                                }
                                3 => {
                                    {
                                        // recRuleLabeledAltStartAction
                                        let mut tmp = CPatternContextExt::new(&**Class_expressionContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        ));
                                        if let Class_expressionContextAll::CPatternContext(ctx) =
                                            cast_mut::<_, Class_expressionContextAll>(&mut tmp)
                                        {
                                            ctx.lhs = Some(_prevctx.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_class_expression);
                                        _localctx = tmp;
                                        recog.base.set_state(179);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(180);
                                        recog.base.match_token(OP_OR, &mut recog.err_handler)?;

                                        // InvokeRule class_expression
                                        recog.base.set_state(181);
                                        let tmp = recog.class_expression_rec(3)?;
                                        if let Class_expressionContextAll::CPatternContext(ctx) =
                                            cast_mut::<_, Class_expressionContextAll>(&mut _localctx)
                                        {
                                            ctx.rhs = Some(tmp.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                    }
                                }
                                4 => {
                                    {
                                        // recRuleLabeledAltStartAction
                                        let mut tmp = CSuffixContextExt::new(&**Class_expressionContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        ));
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_class_expression);
                                        _localctx = tmp;
                                        recog.base.set_state(182);
                                        if !({ recog.precpred(None, 8) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 8)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        // InvokeRule suffix
                                        recog.base.set_state(183);
                                        recog.suffix()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(188);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(16, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- define_union ----------------
pub type Define_unionContextAll<'input> = Define_unionContext<'input>;

pub type Define_unionContext<'input> = BaseParserRuleContext<'input, Define_unionContextExt<'input>>;

#[derive(Clone)]
pub struct Define_unionContextExt<'input> {
    pub name: Option<Rc<IdentifierContextAll<'input>>>,
    pub cast: Option<Rc<IdentifierContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Define_unionContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Define_unionContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_define_union(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_define_union(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Define_unionContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_define_union(self);
    }
}

impl<'input> CustomRuleContext<'input> for Define_unionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_define_union
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_define_union }
}
antlr_rust::tid! {Define_unionContextExt<'a>}

impl<'input> Define_unionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Define_unionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Define_unionContextExt { name: None, cast: None, ph: PhantomData },
        ))
    }
}

pub trait Define_unionContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Define_unionContextExt<'input>>
{
    fn modifiers(&self) -> Option<Rc<ModifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token KW_UNION
    /// Returns `None` if there is no child corresponding to token KW_UNION
    fn KW_UNION(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_UNION, 0)
    }
    fn union_block(&self) -> Option<Rc<Union_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn identifier_all(&self) -> Vec<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn annotation_all(&self) -> Vec<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OP_TO
    /// Returns `None` if there is no child corresponding to token OP_TO
    fn OP_TO(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_TO, 0)
    }
}

impl<'input> Define_unionContextAttrs<'input> for Define_unionContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn define_union(&mut self) -> Result<Rc<Define_unionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Define_unionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_define_union);
        let mut _localctx: Rc<Define_unionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(192);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_AT || _la == OP_HASH {
                    {
                        {
                            // InvokeRule annotation
                            recog.base.set_state(189);
                            recog.annotation()?;
                        }
                    }
                    recog.base.set_state(194);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                // InvokeRule modifiers
                recog.base.set_state(195);
                recog.modifiers()?;

                recog.base.set_state(196);
                recog.base.match_token(KW_UNION, &mut recog.err_handler)?;

                // InvokeRule identifier
                recog.base.set_state(197);
                let tmp = recog.identifier()?;
                cast_mut::<_, Define_unionContext>(&mut _localctx).name = Some(tmp.clone());

                recog.base.set_state(200);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OP_TO {
                    {
                        recog.base.set_state(198);
                        recog.base.match_token(OP_TO, &mut recog.err_handler)?;

                        // InvokeRule identifier
                        recog.base.set_state(199);
                        let tmp = recog.identifier()?;
                        cast_mut::<_, Define_unionContext>(&mut _localctx).cast = Some(tmp.clone());
                    }
                }

                // InvokeRule union_block
                recog.base.set_state(202);
                recog.union_block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- union_block ----------------
pub type Union_blockContextAll<'input> = Union_blockContext<'input>;

pub type Union_blockContext<'input> = BaseParserRuleContext<'input, Union_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Union_blockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Union_blockContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Union_blockContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_union_block(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_union_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Union_blockContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_union_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for Union_blockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_block
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_block }
}
antlr_rust::tid! {Union_blockContextExt<'a>}

impl<'input> Union_blockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Union_blockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Union_blockContextExt { ph: PhantomData }))
    }
}

pub trait Union_blockContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Union_blockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BRACE_L
    /// Returns `None` if there is no child corresponding to token BRACE_L
    fn BRACE_L(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_L, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACE_R
    /// Returns `None` if there is no child corresponding to token BRACE_R
    fn BRACE_R(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_R, 0)
    }
    fn union_term_all(&self) -> Vec<Rc<Union_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn union_term(&self, i: usize) -> Option<Rc<Union_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Union_blockContextAttrs<'input> for Union_blockContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn union_block(&mut self) -> Result<Rc<Union_blockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Union_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_union_block);
        let mut _localctx: Rc<Union_blockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(204);
                recog.base.match_token(BRACE_L, &mut recog.err_handler)?;

                recog.base.set_state(208);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_OR {
                    {
                        {
                            // InvokeRule union_term
                            recog.base.set_state(205);
                            recog.union_term()?;
                        }
                    }
                    recog.base.set_state(210);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(211);
                recog.base.match_token(BRACE_R, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- union_term ----------------
pub type Union_termContextAll<'input> = Union_termContext<'input>;

pub type Union_termContext<'input> = BaseParserRuleContext<'input, Union_termContextExt<'input>>;

#[derive(Clone)]
pub struct Union_termContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Union_termContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Union_termContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_union_term(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_union_term(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Union_termContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_union_term(self);
    }
}

impl<'input> CustomRuleContext<'input> for Union_termContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_term
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_term }
}
antlr_rust::tid! {Union_termContextExt<'a>}

impl<'input> Union_termContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Union_termContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Union_termContextExt { ph: PhantomData }))
    }
}

pub trait Union_termContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Union_termContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OP_OR
    /// Returns `None` if there is no child corresponding to token OP_OR
    fn OP_OR(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_OR, 0)
    }
    fn union_expression_all(&self) -> Vec<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn union_expression(&self, i: usize) -> Option<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn tag_branch(&self) -> Option<Rc<Tag_branchContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Union_termContextAttrs<'input> for Union_termContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn union_term(&mut self) -> Result<Rc<Union_termContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Union_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_union_term);
        let mut _localctx: Rc<Union_termContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(213);
                recog.base.match_token(OP_OR, &mut recog.err_handler)?;

                recog.base.set_state(217);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << OP_NOT) | (1usize << OP_AT) | (1usize << OP_UNTAG) | (1usize << PARENTHESES_L)))
                        != 0)
                    || (((_la - 32) & !0x3f) == 0
                        && ((1usize << (_la - 32))
                            & ((1usize << (INTEGER - 32))
                                | (1usize << (BOOLEAN - 32))
                                | (1usize << (ESCAPED - 32))
                                | (1usize << (REGEX_RANGE - 32))
                                | (1usize << (REGEX_FREE - 32))
                                | (1usize << (STRING_SINGLE - 32))
                                | (1usize << (STRING_DOUBLE - 32))
                                | (1usize << (RAW_ID - 32))
                                | (1usize << (UNICODE_ID - 32))))
                            != 0)
                {
                    {
                        {
                            // InvokeRule union_expression
                            recog.base.set_state(214);
                            recog.union_expression_rec(0)?;
                        }
                    }
                    recog.base.set_state(219);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(221);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OP_HASH {
                    {
                        // InvokeRule tag_branch
                        recog.base.set_state(220);
                        recog.tag_branch()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- union_expression ----------------
#[derive(Debug)]
pub enum Union_expressionContextAll<'input> {
    UHardContext(UHardContext<'input>),
    UUntagContext(UUntagContext<'input>),
    USuffixContext(USuffixContext<'input>),
    UETagContext(UETagContext<'input>),
    UtomContext(UtomContext<'input>),
    UNotContext(UNotContext<'input>),
    USoftContext(USoftContext<'input>),
    Error(Union_expressionContext<'input>),
}
antlr_rust::tid! {Union_expressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Union_expressionContextAll<'input> {}

impl<'input> YggdrasilAntlrParserContext<'input> for Union_expressionContextAll<'input> {}

impl<'input> Deref for Union_expressionContextAll<'input> {
    type Target = dyn Union_expressionContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Union_expressionContextAll::*;
        match self {
            UHardContext(inner) => inner,
            UUntagContext(inner) => inner,
            USuffixContext(inner) => inner,
            UETagContext(inner) => inner,
            UtomContext(inner) => inner,
            UNotContext(inner) => inner,
            USoftContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Union_expressionContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Union_expressionContextAll<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Union_expressionContext<'input> = BaseParserRuleContext<'input, Union_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Union_expressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Union_expressionContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Union_expressionContext<'input> {}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Union_expressionContext<'input> {}

impl<'input> CustomRuleContext<'input> for Union_expressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_expression }
}
antlr_rust::tid! {Union_expressionContextExt<'a>}

impl<'input> Union_expressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Union_expressionContextAll<'input>> {
        Rc::new(Union_expressionContextAll::Error(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Union_expressionContextExt { ph: PhantomData },
        )))
    }
}

pub trait Union_expressionContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Union_expressionContextExt<'input>>
{
}

impl<'input> Union_expressionContextAttrs<'input> for Union_expressionContext<'input> {}

pub type UHardContext<'input> = BaseParserRuleContext<'input, UHardContextExt<'input>>;

pub trait UHardContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token OP_CONCAT
    /// Returns `None` if there is no child corresponding to token OP_CONCAT
    fn OP_CONCAT(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_CONCAT, 0)
    }
    fn union_expression_all(&self) -> Vec<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn union_expression(&self, i: usize) -> Option<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> UHardContextAttrs<'input> for UHardContext<'input> {}

pub struct UHardContextExt<'input> {
    base: Union_expressionContextExt<'input>,
    pub lhs: Option<Rc<Union_expressionContextAll<'input>>>,
    pub rhs: Option<Rc<Union_expressionContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {UHardContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for UHardContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for UHardContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_UHard(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_UHard(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for UHardContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_UHard(self);
    }
}

impl<'input> CustomRuleContext<'input> for UHardContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_expression }
}

impl<'input> Borrow<Union_expressionContextExt<'input>> for UHardContext<'input> {
    fn borrow(&self) -> &Union_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Union_expressionContextExt<'input>> for UHardContext<'input> {
    fn borrow_mut(&mut self) -> &mut Union_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Union_expressionContextAttrs<'input> for UHardContext<'input> {}

impl<'input> UHardContextExt<'input> {
    fn new(ctx: &dyn Union_expressionContextAttrs<'input>) -> Rc<Union_expressionContextAll<'input>> {
        Rc::new(Union_expressionContextAll::UHardContext(BaseParserRuleContext::copy_from(
            ctx,
            UHardContextExt { lhs: None, rhs: None, base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type UUntagContext<'input> = BaseParserRuleContext<'input, UUntagContextExt<'input>>;

pub trait UUntagContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token OP_UNTAG
    /// Returns `None` if there is no child corresponding to token OP_UNTAG
    fn OP_UNTAG(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_UNTAG, 0)
    }
    fn union_expression(&self) -> Option<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> UUntagContextAttrs<'input> for UUntagContext<'input> {}

pub struct UUntagContextExt<'input> {
    base: Union_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {UUntagContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for UUntagContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for UUntagContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_UUntag(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_UUntag(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for UUntagContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_UUntag(self);
    }
}

impl<'input> CustomRuleContext<'input> for UUntagContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_expression }
}

impl<'input> Borrow<Union_expressionContextExt<'input>> for UUntagContext<'input> {
    fn borrow(&self) -> &Union_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Union_expressionContextExt<'input>> for UUntagContext<'input> {
    fn borrow_mut(&mut self) -> &mut Union_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Union_expressionContextAttrs<'input> for UUntagContext<'input> {}

impl<'input> UUntagContextExt<'input> {
    fn new(ctx: &dyn Union_expressionContextAttrs<'input>) -> Rc<Union_expressionContextAll<'input>> {
        Rc::new(Union_expressionContextAll::UUntagContext(BaseParserRuleContext::copy_from(
            ctx,
            UUntagContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type USuffixContext<'input> = BaseParserRuleContext<'input, USuffixContextExt<'input>>;

pub trait USuffixContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn union_expression(&self) -> Option<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn suffix(&self) -> Option<Rc<SuffixContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> USuffixContextAttrs<'input> for USuffixContext<'input> {}

pub struct USuffixContextExt<'input> {
    base: Union_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {USuffixContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for USuffixContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for USuffixContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_USuffix(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_USuffix(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for USuffixContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_USuffix(self);
    }
}

impl<'input> CustomRuleContext<'input> for USuffixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_expression }
}

impl<'input> Borrow<Union_expressionContextExt<'input>> for USuffixContext<'input> {
    fn borrow(&self) -> &Union_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Union_expressionContextExt<'input>> for USuffixContext<'input> {
    fn borrow_mut(&mut self) -> &mut Union_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Union_expressionContextAttrs<'input> for USuffixContext<'input> {}

impl<'input> USuffixContextExt<'input> {
    fn new(ctx: &dyn Union_expressionContextAttrs<'input>) -> Rc<Union_expressionContextAll<'input>> {
        Rc::new(Union_expressionContextAll::USuffixContext(BaseParserRuleContext::copy_from(
            ctx,
            USuffixContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type UETagContext<'input> = BaseParserRuleContext<'input, UETagContextExt<'input>>;

pub trait UETagContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn union_expression(&self) -> Option<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> UETagContextAttrs<'input> for UETagContext<'input> {}

pub struct UETagContextExt<'input> {
    base: Union_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {UETagContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for UETagContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for UETagContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_UETag(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_UETag(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for UETagContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_UETag(self);
    }
}

impl<'input> CustomRuleContext<'input> for UETagContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_expression }
}

impl<'input> Borrow<Union_expressionContextExt<'input>> for UETagContext<'input> {
    fn borrow(&self) -> &Union_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Union_expressionContextExt<'input>> for UETagContext<'input> {
    fn borrow_mut(&mut self) -> &mut Union_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Union_expressionContextAttrs<'input> for UETagContext<'input> {}

impl<'input> UETagContextExt<'input> {
    fn new(ctx: &dyn Union_expressionContextAttrs<'input>) -> Rc<Union_expressionContextAll<'input>> {
        Rc::new(Union_expressionContextAll::UETagContext(BaseParserRuleContext::copy_from(
            ctx,
            UETagContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type UtomContext<'input> = BaseParserRuleContext<'input, UtomContextExt<'input>>;

pub trait UtomContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn atomic(&self) -> Option<Rc<AtomicContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> UtomContextAttrs<'input> for UtomContext<'input> {}

pub struct UtomContextExt<'input> {
    base: Union_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {UtomContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for UtomContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for UtomContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_Utom(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_Utom(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for UtomContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_Utom(self);
    }
}

impl<'input> CustomRuleContext<'input> for UtomContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_expression }
}

impl<'input> Borrow<Union_expressionContextExt<'input>> for UtomContext<'input> {
    fn borrow(&self) -> &Union_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Union_expressionContextExt<'input>> for UtomContext<'input> {
    fn borrow_mut(&mut self) -> &mut Union_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Union_expressionContextAttrs<'input> for UtomContext<'input> {}

impl<'input> UtomContextExt<'input> {
    fn new(ctx: &dyn Union_expressionContextAttrs<'input>) -> Rc<Union_expressionContextAll<'input>> {
        Rc::new(Union_expressionContextAll::UtomContext(BaseParserRuleContext::copy_from(
            ctx,
            UtomContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type UNotContext<'input> = BaseParserRuleContext<'input, UNotContextExt<'input>>;

pub trait UNotContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token OP_NOT
    /// Returns `None` if there is no child corresponding to token OP_NOT
    fn OP_NOT(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_NOT, 0)
    }
    fn union_expression(&self) -> Option<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> UNotContextAttrs<'input> for UNotContext<'input> {}

pub struct UNotContextExt<'input> {
    base: Union_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {UNotContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for UNotContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for UNotContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_UNot(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_UNot(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for UNotContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_UNot(self);
    }
}

impl<'input> CustomRuleContext<'input> for UNotContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_expression }
}

impl<'input> Borrow<Union_expressionContextExt<'input>> for UNotContext<'input> {
    fn borrow(&self) -> &Union_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Union_expressionContextExt<'input>> for UNotContext<'input> {
    fn borrow_mut(&mut self) -> &mut Union_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Union_expressionContextAttrs<'input> for UNotContext<'input> {}

impl<'input> UNotContextExt<'input> {
    fn new(ctx: &dyn Union_expressionContextAttrs<'input>) -> Rc<Union_expressionContextAll<'input>> {
        Rc::new(Union_expressionContextAll::UNotContext(BaseParserRuleContext::copy_from(
            ctx,
            UNotContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type USoftContext<'input> = BaseParserRuleContext<'input, USoftContextExt<'input>>;

pub trait USoftContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn union_expression_all(&self) -> Vec<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn union_expression(&self, i: usize) -> Option<Rc<Union_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> USoftContextAttrs<'input> for USoftContext<'input> {}

pub struct USoftContextExt<'input> {
    base: Union_expressionContextExt<'input>,
    pub lhs: Option<Rc<Union_expressionContextAll<'input>>>,
    pub rhs: Option<Rc<Union_expressionContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {USoftContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for USoftContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for USoftContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_USoft(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_USoft(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for USoftContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_USoft(self);
    }
}

impl<'input> CustomRuleContext<'input> for USoftContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_union_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_union_expression }
}

impl<'input> Borrow<Union_expressionContextExt<'input>> for USoftContext<'input> {
    fn borrow(&self) -> &Union_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Union_expressionContextExt<'input>> for USoftContext<'input> {
    fn borrow_mut(&mut self) -> &mut Union_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Union_expressionContextAttrs<'input> for USoftContext<'input> {}

impl<'input> USoftContextExt<'input> {
    fn new(ctx: &dyn Union_expressionContextAttrs<'input>) -> Rc<Union_expressionContextAll<'input>> {
        Rc::new(Union_expressionContextAll::USoftContext(BaseParserRuleContext::copy_from(
            ctx,
            USoftContextExt { lhs: None, rhs: None, base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn union_expression(&mut self) -> Result<Rc<Union_expressionContextAll<'input>>, ANTLRError> {
        self.union_expression_rec(0)
    }

    fn union_expression_rec(&mut self, _p: isize) -> Result<Rc<Union_expressionContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = Union_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_recursion_rule(_localctx.clone(), 28, RULE_union_expression, _p);
        let mut _localctx: Rc<Union_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 28;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(233);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(22, &mut recog.base)? {
                    1 => {
                        {
                            let mut tmp = UETagContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();

                            // InvokeRule identifier
                            recog.base.set_state(224);
                            recog.identifier()?;

                            recog.base.set_state(225);
                            recog.base.match_token(COLON, &mut recog.err_handler)?;

                            // InvokeRule union_expression
                            recog.base.set_state(226);
                            recog.union_expression_rec(6)?;
                        }
                    }
                    2 => {
                        {
                            let mut tmp = UUntagContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(228);
                            recog.base.match_token(OP_UNTAG, &mut recog.err_handler)?;

                            // InvokeRule union_expression
                            recog.base.set_state(229);
                            recog.union_expression_rec(5)?;
                        }
                    }
                    3 => {
                        {
                            let mut tmp = UNotContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(230);
                            recog.base.match_token(OP_NOT, &mut recog.err_handler)?;

                            // InvokeRule union_expression
                            recog.base.set_state(231);
                            recog.union_expression_rec(4)?;
                        }
                    }
                    4 => {
                        {
                            let mut tmp = UtomContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            // InvokeRule atomic
                            recog.base.set_state(232);
                            recog.atomic()?;
                        }
                    }

                    _ => {}
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(244);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(24, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(242);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(23, &mut recog.base)? {
                                1 => {
                                    {
                                        // recRuleLabeledAltStartAction
                                        let mut tmp = UHardContextExt::new(&**Union_expressionContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        ));
                                        if let Union_expressionContextAll::UHardContext(ctx) =
                                            cast_mut::<_, Union_expressionContextAll>(&mut tmp)
                                        {
                                            ctx.lhs = Some(_prevctx.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_union_expression);
                                        _localctx = tmp;
                                        recog.base.set_state(235);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(236);
                                        recog.base.match_token(OP_CONCAT, &mut recog.err_handler)?;

                                        // InvokeRule union_expression
                                        recog.base.set_state(237);
                                        let tmp = recog.union_expression_rec(4)?;
                                        if let Union_expressionContextAll::UHardContext(ctx) =
                                            cast_mut::<_, Union_expressionContextAll>(&mut _localctx)
                                        {
                                            ctx.rhs = Some(tmp.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                    }
                                }
                                2 => {
                                    {
                                        // recRuleLabeledAltStartAction
                                        let mut tmp = USoftContextExt::new(&**Union_expressionContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        ));
                                        if let Union_expressionContextAll::USoftContext(ctx) =
                                            cast_mut::<_, Union_expressionContextAll>(&mut tmp)
                                        {
                                            ctx.lhs = Some(_prevctx.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_union_expression);
                                        _localctx = tmp;
                                        recog.base.set_state(238);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        // InvokeRule union_expression
                                        recog.base.set_state(239);
                                        let tmp = recog.union_expression_rec(3)?;
                                        if let Union_expressionContextAll::USoftContext(ctx) =
                                            cast_mut::<_, Union_expressionContextAll>(&mut _localctx)
                                        {
                                            ctx.rhs = Some(tmp.clone());
                                        }
                                        else {
                                            unreachable!("cant cast");
                                        }
                                    }
                                }
                                3 => {
                                    {
                                        // recRuleLabeledAltStartAction
                                        let mut tmp = USuffixContextExt::new(&**Union_expressionContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        ));
                                        recog.push_new_recursion_context(tmp.clone(), _startState, RULE_union_expression);
                                        _localctx = tmp;
                                        recog.base.set_state(240);
                                        if !({ recog.precpred(None, 7) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 7)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        // InvokeRule suffix
                                        recog.base.set_state(241);
                                        recog.suffix()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(246);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(24, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- define_climb ----------------
pub type Define_climbContextAll<'input> = Define_climbContext<'input>;

pub type Define_climbContext<'input> = BaseParserRuleContext<'input, Define_climbContextExt<'input>>;

#[derive(Clone)]
pub struct Define_climbContextExt<'input> {
    pub name: Option<Rc<IdentifierContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Define_climbContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Define_climbContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_define_climb(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_define_climb(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Define_climbContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_define_climb(self);
    }
}

impl<'input> CustomRuleContext<'input> for Define_climbContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_define_climb
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_define_climb }
}
antlr_rust::tid! {Define_climbContextExt<'a>}

impl<'input> Define_climbContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Define_climbContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Define_climbContextExt { name: None, ph: PhantomData },
        ))
    }
}

pub trait Define_climbContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Define_climbContextExt<'input>>
{
    fn modifiers(&self) -> Option<Rc<ModifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token KW_CLIMB
    /// Returns `None` if there is no child corresponding to token KW_CLIMB
    fn KW_CLIMB(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_CLIMB, 0)
    }
    fn union_block(&self) -> Option<Rc<Union_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn annotation_all(&self) -> Vec<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Define_climbContextAttrs<'input> for Define_climbContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn define_climb(&mut self) -> Result<Rc<Define_climbContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Define_climbContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_define_climb);
        let mut _localctx: Rc<Define_climbContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(250);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_AT || _la == OP_HASH {
                    {
                        {
                            // InvokeRule annotation
                            recog.base.set_state(247);
                            recog.annotation()?;
                        }
                    }
                    recog.base.set_state(252);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                // InvokeRule modifiers
                recog.base.set_state(253);
                recog.modifiers()?;

                recog.base.set_state(254);
                recog.base.match_token(KW_CLIMB, &mut recog.err_handler)?;

                // InvokeRule identifier
                recog.base.set_state(255);
                let tmp = recog.identifier()?;
                cast_mut::<_, Define_climbContext>(&mut _localctx).name = Some(tmp.clone());

                // InvokeRule union_block
                recog.base.set_state(256);
                recog.union_block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- tag_branch ----------------
pub type Tag_branchContextAll<'input> = Tag_branchContext<'input>;

pub type Tag_branchContext<'input> = BaseParserRuleContext<'input, Tag_branchContextExt<'input>>;

#[derive(Clone)]
pub struct Tag_branchContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Tag_branchContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Tag_branchContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_tag_branch(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_tag_branch(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Tag_branchContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_tag_branch(self);
    }
}

impl<'input> CustomRuleContext<'input> for Tag_branchContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_tag_branch
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_tag_branch }
}
antlr_rust::tid! {Tag_branchContextExt<'a>}

impl<'input> Tag_branchContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Tag_branchContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Tag_branchContextExt { ph: PhantomData }))
    }
}

pub trait Tag_branchContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Tag_branchContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OP_HASH
    /// Returns `None` if there is no child corresponding to token OP_HASH
    fn OP_HASH(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_HASH, 0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OP_GT
    /// Returns `None` if there is no child corresponding to token OP_GT
    fn OP_GT(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_GT, 0)
    }
}

impl<'input> Tag_branchContextAttrs<'input> for Tag_branchContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn tag_branch(&mut self) -> Result<Rc<Tag_branchContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Tag_branchContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_tag_branch);
        let mut _localctx: Rc<Tag_branchContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(258);
                recog.base.match_token(OP_HASH, &mut recog.err_handler)?;

                // InvokeRule identifier
                recog.base.set_state(259);
                recog.identifier()?;

                recog.base.set_state(261);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OP_GT {
                    {
                        recog.base.set_state(260);
                        recog.base.match_token(OP_GT, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- define_token ----------------
pub type Define_tokenContextAll<'input> = Define_tokenContext<'input>;

pub type Define_tokenContext<'input> = BaseParserRuleContext<'input, Define_tokenContextExt<'input>>;

#[derive(Clone)]
pub struct Define_tokenContextExt<'input> {
    pub name: Option<Rc<IdentifierContextAll<'input>>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Define_tokenContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Define_tokenContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_define_token(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_define_token(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Define_tokenContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_define_token(self);
    }
}

impl<'input> CustomRuleContext<'input> for Define_tokenContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_define_token
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_define_token }
}
antlr_rust::tid! {Define_tokenContextExt<'a>}

impl<'input> Define_tokenContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Define_tokenContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Define_tokenContextExt { name: None, ph: PhantomData },
        ))
    }
}

pub trait Define_tokenContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Define_tokenContextExt<'input>>
{
    fn modifiers(&self) -> Option<Rc<ModifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token KW_TOKEN
    /// Returns `None` if there is no child corresponding to token KW_TOKEN
    fn KW_TOKEN(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_TOKEN, 0)
    }
    fn token_block(&self) -> Option<Rc<Token_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn annotation_all(&self) -> Vec<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Define_tokenContextAttrs<'input> for Define_tokenContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn define_token(&mut self) -> Result<Rc<Define_tokenContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Define_tokenContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_define_token);
        let mut _localctx: Rc<Define_tokenContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(266);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_AT || _la == OP_HASH {
                    {
                        {
                            // InvokeRule annotation
                            recog.base.set_state(263);
                            recog.annotation()?;
                        }
                    }
                    recog.base.set_state(268);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                // InvokeRule modifiers
                recog.base.set_state(269);
                recog.modifiers()?;

                recog.base.set_state(270);
                recog.base.match_token(KW_TOKEN, &mut recog.err_handler)?;

                recog.base.set_state(272);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == RAW_ID || _la == UNICODE_ID {
                    {
                        // InvokeRule identifier
                        recog.base.set_state(271);
                        let tmp = recog.identifier()?;
                        cast_mut::<_, Define_tokenContext>(&mut _localctx).name = Some(tmp.clone());
                    }
                }

                // InvokeRule token_block
                recog.base.set_state(274);
                recog.token_block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- token_block ----------------
pub type Token_blockContextAll<'input> = Token_blockContext<'input>;

pub type Token_blockContext<'input> = BaseParserRuleContext<'input, Token_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Token_blockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Token_blockContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Token_blockContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_token_block(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_token_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Token_blockContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_token_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for Token_blockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_token_block
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_token_block }
}
antlr_rust::tid! {Token_blockContextExt<'a>}

impl<'input> Token_blockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Token_blockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Token_blockContextExt { ph: PhantomData }))
    }
}

pub trait Token_blockContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Token_blockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BRACE_L
    /// Returns `None` if there is no child corresponding to token BRACE_L
    fn BRACE_L(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_L, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACE_R
    /// Returns `None` if there is no child corresponding to token BRACE_R
    fn BRACE_R(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_R, 0)
    }
    fn token_pair_all(&self) -> Vec<Rc<Token_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn token_pair(&self, i: usize) -> Option<Rc<Token_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
    fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
    fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, i)
    }
}

impl<'input> Token_blockContextAttrs<'input> for Token_blockContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn token_block(&mut self) -> Result<Rc<Token_blockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Token_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_token_block);
        let mut _localctx: Rc<Token_blockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(276);
                recog.base.match_token(BRACE_L, &mut recog.err_handler)?;

                recog.base.set_state(281);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la) & ((1usize << SEMICOLON) | (1usize << OP_AT) | (1usize << OP_HASH))) != 0)
                    || _la == RAW_ID
                    || _la == UNICODE_ID
                {
                    {
                        recog.base.set_state(279);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            OP_AT | OP_HASH | RAW_ID | UNICODE_ID => {
                                {
                                    // InvokeRule token_pair
                                    recog.base.set_state(277);
                                    recog.token_pair()?;
                                }
                            }

                            SEMICOLON => {
                                recog.base.set_state(278);
                                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?,
                        }
                    }
                    recog.base.set_state(283);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(284);
                recog.base.match_token(BRACE_R, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- token_pair ----------------
pub type Token_pairContextAll<'input> = Token_pairContext<'input>;

pub type Token_pairContext<'input> = BaseParserRuleContext<'input, Token_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Token_pairContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Token_pairContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Token_pairContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_token_pair(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_token_pair(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Token_pairContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_token_pair(self);
    }
}

impl<'input> CustomRuleContext<'input> for Token_pairContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_token_pair
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_token_pair }
}
antlr_rust::tid! {Token_pairContextExt<'a>}

impl<'input> Token_pairContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Token_pairContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Token_pairContextExt { ph: PhantomData }))
    }
}

pub trait Token_pairContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Token_pairContextExt<'input>>
{
    fn modifiers(&self) -> Option<Rc<ModifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn atomic(&self) -> Option<Rc<AtomicContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn annotation_all(&self) -> Vec<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Token_pairContextAttrs<'input> for Token_pairContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn token_pair(&mut self) -> Result<Rc<Token_pairContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Token_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_token_pair);
        let mut _localctx: Rc<Token_pairContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(289);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_AT || _la == OP_HASH {
                    {
                        {
                            // InvokeRule annotation
                            recog.base.set_state(286);
                            recog.annotation()?;
                        }
                    }
                    recog.base.set_state(291);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                // InvokeRule modifiers
                recog.base.set_state(292);
                recog.modifiers()?;

                // InvokeRule identifier
                recog.base.set_state(293);
                recog.identifier()?;

                recog.base.set_state(294);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                // InvokeRule atomic
                recog.base.set_state(295);
                recog.atomic()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- token_expression ----------------
#[derive(Debug)]
pub enum Token_expressionContextAll<'input> {
    TOrContext(TOrContext<'input>),
    TAtomContext(TAtomContext<'input>),
    Error(Token_expressionContext<'input>),
}
antlr_rust::tid! {Token_expressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Token_expressionContextAll<'input> {}

impl<'input> YggdrasilAntlrParserContext<'input> for Token_expressionContextAll<'input> {}

impl<'input> Deref for Token_expressionContextAll<'input> {
    type Target = dyn Token_expressionContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Token_expressionContextAll::*;
        match self {
            TOrContext(inner) => inner,
            TAtomContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Token_expressionContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Token_expressionContextAll<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Token_expressionContext<'input> = BaseParserRuleContext<'input, Token_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Token_expressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Token_expressionContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Token_expressionContext<'input> {}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Token_expressionContext<'input> {}

impl<'input> CustomRuleContext<'input> for Token_expressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_token_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_token_expression }
}
antlr_rust::tid! {Token_expressionContextExt<'a>}

impl<'input> Token_expressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Token_expressionContextAll<'input>> {
        Rc::new(Token_expressionContextAll::Error(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Token_expressionContextExt { ph: PhantomData },
        )))
    }
}

pub trait Token_expressionContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Token_expressionContextExt<'input>>
{
}

impl<'input> Token_expressionContextAttrs<'input> for Token_expressionContext<'input> {}

pub type TOrContext<'input> = BaseParserRuleContext<'input, TOrContextExt<'input>>;

pub trait TOrContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn token_expression_all(&self) -> Vec<Rc<Token_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn token_expression(&self, i: usize) -> Option<Rc<Token_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OP_OR
    /// Returns `None` if there is no child corresponding to token OP_OR
    fn OP_OR(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_OR, 0)
    }
}

impl<'input> TOrContextAttrs<'input> for TOrContext<'input> {}

pub struct TOrContextExt<'input> {
    base: Token_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {TOrContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for TOrContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for TOrContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_TOr(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_TOr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for TOrContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_TOr(self);
    }
}

impl<'input> CustomRuleContext<'input> for TOrContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_token_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_token_expression }
}

impl<'input> Borrow<Token_expressionContextExt<'input>> for TOrContext<'input> {
    fn borrow(&self) -> &Token_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Token_expressionContextExt<'input>> for TOrContext<'input> {
    fn borrow_mut(&mut self) -> &mut Token_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Token_expressionContextAttrs<'input> for TOrContext<'input> {}

impl<'input> TOrContextExt<'input> {
    fn new(ctx: &dyn Token_expressionContextAttrs<'input>) -> Rc<Token_expressionContextAll<'input>> {
        Rc::new(Token_expressionContextAll::TOrContext(BaseParserRuleContext::copy_from(
            ctx,
            TOrContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type TAtomContext<'input> = BaseParserRuleContext<'input, TAtomContextExt<'input>>;

pub trait TAtomContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn atomic(&self) -> Option<Rc<AtomicContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TAtomContextAttrs<'input> for TAtomContext<'input> {}

pub struct TAtomContextExt<'input> {
    base: Token_expressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {TAtomContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for TAtomContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for TAtomContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_TAtom(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_TAtom(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for TAtomContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_TAtom(self);
    }
}

impl<'input> CustomRuleContext<'input> for TAtomContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_token_expression
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_token_expression }
}

impl<'input> Borrow<Token_expressionContextExt<'input>> for TAtomContext<'input> {
    fn borrow(&self) -> &Token_expressionContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<Token_expressionContextExt<'input>> for TAtomContext<'input> {
    fn borrow_mut(&mut self) -> &mut Token_expressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Token_expressionContextAttrs<'input> for TAtomContext<'input> {}

impl<'input> TAtomContextExt<'input> {
    fn new(ctx: &dyn Token_expressionContextAttrs<'input>) -> Rc<Token_expressionContextAll<'input>> {
        Rc::new(Token_expressionContextAll::TAtomContext(BaseParserRuleContext::copy_from(
            ctx,
            TAtomContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn token_expression(&mut self) -> Result<Rc<Token_expressionContextAll<'input>>, ANTLRError> {
        self.token_expression_rec(0)
    }

    fn token_expression_rec(&mut self, _p: isize) -> Result<Rc<Token_expressionContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = Token_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_recursion_rule(_localctx.clone(), 40, RULE_token_expression, _p);
        let mut _localctx: Rc<Token_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 40;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    let mut tmp = TAtomContextExt::new(&**_localctx);
                    recog.ctx = Some(tmp.clone());
                    _localctx = tmp;
                    _prevctx = _localctx.clone();

                    // InvokeRule atomic
                    recog.base.set_state(298);
                    recog.atomic()?;
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(305);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(32, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            {
                                // recRuleLabeledAltStartAction
                                let mut tmp =
                                    TOrContextExt::new(&**Token_expressionContextExt::new(_parentctx.clone(), _parentState));
                                recog.push_new_recursion_context(tmp.clone(), _startState, RULE_token_expression);
                                _localctx = tmp;
                                recog.base.set_state(300);
                                if !({ recog.precpred(None, 2) }) {
                                    Err(FailedPredicateError::new(
                                        &mut recog.base,
                                        Some("recog.precpred(None, 2)".to_owned()),
                                        None,
                                    ))?;
                                }
                                recog.base.set_state(301);
                                recog.base.match_token(OP_OR, &mut recog.err_handler)?;

                                // InvokeRule token_expression
                                recog.base.set_state(302);
                                recog.token_expression_rec(3)?;
                            }
                        }
                    }
                    recog.base.set_state(307);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(32, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- define_external ----------------
pub type Define_externalContextAll<'input> = Define_externalContext<'input>;

pub type Define_externalContext<'input> = BaseParserRuleContext<'input, Define_externalContextExt<'input>>;

#[derive(Clone)]
pub struct Define_externalContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Define_externalContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Define_externalContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_define_external(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_define_external(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Define_externalContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_define_external(self);
    }
}

impl<'input> CustomRuleContext<'input> for Define_externalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_define_external
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_define_external }
}
antlr_rust::tid! {Define_externalContextExt<'a>}

impl<'input> Define_externalContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Define_externalContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Define_externalContextExt { ph: PhantomData }))
    }
}

pub trait Define_externalContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Define_externalContextExt<'input>>
{
    fn modifiers(&self) -> Option<Rc<ModifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token KW_EXTERNAL
    /// Returns `None` if there is no child corresponding to token KW_EXTERNAL
    fn KW_EXTERNAL(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_EXTERNAL, 0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn external_block(&self) -> Option<Rc<External_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn annotation_all(&self) -> Vec<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Define_externalContextAttrs<'input> for Define_externalContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn define_external(&mut self) -> Result<Rc<Define_externalContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Define_externalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_define_external);
        let mut _localctx: Rc<Define_externalContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(311);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_AT || _la == OP_HASH {
                    {
                        {
                            // InvokeRule annotation
                            recog.base.set_state(308);
                            recog.annotation()?;
                        }
                    }
                    recog.base.set_state(313);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                // InvokeRule modifiers
                recog.base.set_state(314);
                recog.modifiers()?;

                recog.base.set_state(315);
                recog.base.match_token(KW_EXTERNAL, &mut recog.err_handler)?;

                // InvokeRule identifier
                recog.base.set_state(316);
                recog.identifier()?;

                // InvokeRule external_block
                recog.base.set_state(317);
                recog.external_block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- external_block ----------------
pub type External_blockContextAll<'input> = External_blockContext<'input>;

pub type External_blockContext<'input> = BaseParserRuleContext<'input, External_blockContextExt<'input>>;

#[derive(Clone)]
pub struct External_blockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for External_blockContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for External_blockContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_external_block(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_external_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for External_blockContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_external_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for External_blockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_external_block
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_external_block }
}
antlr_rust::tid! {External_blockContextExt<'a>}

impl<'input> External_blockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<External_blockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, External_blockContextExt { ph: PhantomData }))
    }
}

pub trait External_blockContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<External_blockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BRACE_L
    /// Returns `None` if there is no child corresponding to token BRACE_L
    fn BRACE_L(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_L, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BRACE_R
    /// Returns `None` if there is no child corresponding to token BRACE_R
    fn BRACE_R(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BRACE_R, 0)
    }
    fn external_pair_all(&self) -> Vec<Rc<External_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn external_pair(&self, i: usize) -> Option<Rc<External_pairContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
    fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
    fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMICOLON, i)
    }
}

impl<'input> External_blockContextAttrs<'input> for External_blockContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn external_block(&mut self) -> Result<Rc<External_blockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = External_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_external_block);
        let mut _localctx: Rc<External_blockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(319);
                recog.base.match_token(BRACE_L, &mut recog.err_handler)?;

                recog.base.set_state(324);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la) & ((1usize << SEMICOLON) | (1usize << OP_AT) | (1usize << OP_HASH))) != 0)
                    || _la == RAW_ID
                    || _la == UNICODE_ID
                {
                    {
                        recog.base.set_state(322);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            OP_AT | OP_HASH | RAW_ID | UNICODE_ID => {
                                {
                                    // InvokeRule external_pair
                                    recog.base.set_state(320);
                                    recog.external_pair()?;
                                }
                            }

                            SEMICOLON => {
                                recog.base.set_state(321);
                                recog.base.match_token(SEMICOLON, &mut recog.err_handler)?;
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?,
                        }
                    }
                    recog.base.set_state(326);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(327);
                recog.base.match_token(BRACE_R, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- external_pair ----------------
pub type External_pairContextAll<'input> = External_pairContext<'input>;

pub type External_pairContext<'input> = BaseParserRuleContext<'input, External_pairContextExt<'input>>;

#[derive(Clone)]
pub struct External_pairContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for External_pairContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for External_pairContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_external_pair(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_external_pair(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for External_pairContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_external_pair(self);
    }
}

impl<'input> CustomRuleContext<'input> for External_pairContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_external_pair
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_external_pair }
}
antlr_rust::tid! {External_pairContextExt<'a>}

impl<'input> External_pairContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<External_pairContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, External_pairContextExt { ph: PhantomData }))
    }
}

pub trait External_pairContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<External_pairContextExt<'input>>
{
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn namepath(&self) -> Option<Rc<NamepathContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn annotation_all(&self) -> Vec<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> External_pairContextAttrs<'input> for External_pairContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn external_pair(&mut self) -> Result<Rc<External_pairContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = External_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_external_pair);
        let mut _localctx: Rc<External_pairContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(332);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_AT || _la == OP_HASH {
                    {
                        {
                            // InvokeRule annotation
                            recog.base.set_state(329);
                            recog.annotation()?;
                        }
                    }
                    recog.base.set_state(334);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                // InvokeRule identifier
                recog.base.set_state(335);
                recog.identifier()?;

                recog.base.set_state(336);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                // InvokeRule namepath
                recog.base.set_state(337);
                recog.namepath()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- define_inspector ----------------
pub type Define_inspectorContextAll<'input> = Define_inspectorContext<'input>;

pub type Define_inspectorContext<'input> = BaseParserRuleContext<'input, Define_inspectorContextExt<'input>>;

#[derive(Clone)]
pub struct Define_inspectorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Define_inspectorContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Define_inspectorContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_define_inspector(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_define_inspector(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Define_inspectorContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_define_inspector(self);
    }
}

impl<'input> CustomRuleContext<'input> for Define_inspectorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_define_inspector
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_define_inspector }
}
antlr_rust::tid! {Define_inspectorContextExt<'a>}

impl<'input> Define_inspectorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Define_inspectorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Define_inspectorContextExt { ph: PhantomData }))
    }
}

pub trait Define_inspectorContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Define_inspectorContextExt<'input>>
{
    fn modifiers(&self) -> Option<Rc<ModifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token KW_INSPECTOR
    /// Returns `None` if there is no child corresponding to token KW_INSPECTOR
    fn KW_INSPECTOR(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_INSPECTOR, 0)
    }
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn external_block(&self) -> Option<Rc<External_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn annotation_all(&self) -> Vec<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Define_inspectorContextAttrs<'input> for Define_inspectorContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn define_inspector(&mut self) -> Result<Rc<Define_inspectorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Define_inspectorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_define_inspector);
        let mut _localctx: Rc<Define_inspectorContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(342);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OP_AT || _la == OP_HASH {
                    {
                        {
                            // InvokeRule annotation
                            recog.base.set_state(339);
                            recog.annotation()?;
                        }
                    }
                    recog.base.set_state(344);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                // InvokeRule modifiers
                recog.base.set_state(345);
                recog.modifiers()?;

                recog.base.set_state(346);
                recog.base.match_token(KW_INSPECTOR, &mut recog.err_handler)?;

                // InvokeRule identifier
                recog.base.set_state(347);
                recog.identifier()?;

                // InvokeRule external_block
                recog.base.set_state(348);
                recog.external_block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- annotation ----------------
pub type AnnotationContextAll<'input> = AnnotationContext<'input>;

pub type AnnotationContext<'input> = BaseParserRuleContext<'input, AnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for AnnotationContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AnnotationContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_annotation(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_annotation(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AnnotationContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_annotation(self);
    }
}

impl<'input> CustomRuleContext<'input> for AnnotationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_annotation
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_annotation }
}
antlr_rust::tid! {AnnotationContextExt<'a>}

impl<'input> AnnotationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AnnotationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, AnnotationContextExt { ph: PhantomData }))
    }
}

pub trait AnnotationContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<AnnotationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OP_HASH
    /// Returns `None` if there is no child corresponding to token OP_HASH
    fn OP_HASH(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_HASH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OP_AT
    /// Returns `None` if there is no child corresponding to token OP_AT
    fn OP_AT(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_AT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token KW_EXTERNAL
    /// Returns `None` if there is no child corresponding to token KW_EXTERNAL
    fn KW_EXTERNAL(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_EXTERNAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token KW_INSPECTOR
    /// Returns `None` if there is no child corresponding to token KW_INSPECTOR
    fn KW_INSPECTOR(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KW_INSPECTOR, 0)
    }
    fn namepath(&self) -> Option<Rc<NamepathContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn tuple_block(&self) -> Option<Rc<Tuple_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AnnotationContextAttrs<'input> for AnnotationContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn annotation(&mut self) -> Result<Rc<AnnotationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_annotation);
        let mut _localctx: Rc<AnnotationContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(350);
                _la = recog.base.input.la(1);
                if { !(_la == OP_AT || _la == OP_HASH) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                }
                else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
                recog.base.set_state(354);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    KW_EXTERNAL => {
                        recog.base.set_state(351);
                        recog.base.match_token(KW_EXTERNAL, &mut recog.err_handler)?;
                    }

                    KW_INSPECTOR => {
                        recog.base.set_state(352);
                        recog.base.match_token(KW_INSPECTOR, &mut recog.err_handler)?;
                    }

                    RAW_ID | UNICODE_ID => {
                        {
                            // InvokeRule namepath
                            recog.base.set_state(353);
                            recog.namepath()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?,
                }
                recog.base.set_state(357);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == PARENTHESES_L {
                    {
                        // InvokeRule tuple_block
                        recog.base.set_state(356);
                        recog.tuple_block()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

//------------------- modifiers ----------------
pub type ModifiersContextAll<'input> = ModifiersContext<'input>;

pub type ModifiersContext<'input> = BaseParserRuleContext<'input, ModifiersContextExt<'input>>;

#[derive(Clone)]
pub struct ModifiersContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for ModifiersContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for ModifiersContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_modifiers(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_modifiers(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for ModifiersContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_modifiers(self);
    }
}

impl<'input> CustomRuleContext<'input> for ModifiersContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_modifiers
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_modifiers }
}
antlr_rust::tid! {ModifiersContextExt<'a>}

impl<'input> ModifiersContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ModifiersContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, ModifiersContextExt { ph: PhantomData }))
    }
}

pub trait ModifiersContextAttrs<'input>: YggdrasilAntlrParserContext<'input> + BorrowMut<ModifiersContextExt<'input>> {
    fn identifier_all(&self) -> Vec<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ModifiersContextAttrs<'input> for ModifiersContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn modifiers(&mut self) -> Result<Rc<ModifiersContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ModifiersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_modifiers);
        let mut _localctx: Rc<ModifiersContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(362);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(40, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                // InvokeRule identifier
                                recog.base.set_state(359);
                                recog.identifier()?;
                            }
                        }
                    }
                    recog.base.set_state(364);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(40, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- macro_call ----------------
pub type Macro_callContextAll<'input> = Macro_callContext<'input>;

pub type Macro_callContext<'input> = BaseParserRuleContext<'input, Macro_callContextExt<'input>>;

#[derive(Clone)]
pub struct Macro_callContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Macro_callContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Macro_callContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_macro_call(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_macro_call(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Macro_callContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_macro_call(self);
    }
}

impl<'input> CustomRuleContext<'input> for Macro_callContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_macro_call
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_macro_call }
}
antlr_rust::tid! {Macro_callContextExt<'a>}

impl<'input> Macro_callContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Macro_callContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Macro_callContextExt { ph: PhantomData }))
    }
}

pub trait Macro_callContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Macro_callContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OP_AT
    /// Returns `None` if there is no child corresponding to token OP_AT
    fn OP_AT(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_AT, 0)
    }
    fn namepath(&self) -> Option<Rc<NamepathContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn tuple_block(&self) -> Option<Rc<Tuple_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Macro_callContextAttrs<'input> for Macro_callContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn macro_call(&mut self) -> Result<Rc<Macro_callContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Macro_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_macro_call);
        let mut _localctx: Rc<Macro_callContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(365);
                recog.base.match_token(OP_AT, &mut recog.err_handler)?;

                // InvokeRule namepath
                recog.base.set_state(366);
                recog.namepath()?;

                recog.base.set_state(368);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(41, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            // InvokeRule tuple_block
                            recog.base.set_state(367);
                            recog.tuple_block()?;
                        }
                    }

                    _ => {}
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- tuple_block ----------------
pub type Tuple_blockContextAll<'input> = Tuple_blockContext<'input>;

pub type Tuple_blockContext<'input> = BaseParserRuleContext<'input, Tuple_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Tuple_blockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for Tuple_blockContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for Tuple_blockContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_tuple_block(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_tuple_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for Tuple_blockContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_tuple_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for Tuple_blockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_tuple_block
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_tuple_block }
}
antlr_rust::tid! {Tuple_blockContextExt<'a>}

impl<'input> Tuple_blockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Tuple_blockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Tuple_blockContextExt { ph: PhantomData }))
    }
}

pub trait Tuple_blockContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<Tuple_blockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PARENTHESES_L
    /// Returns `None` if there is no child corresponding to token PARENTHESES_L
    fn PARENTHESES_L(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PARENTHESES_L, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PARENTHESES_R
    /// Returns `None` if there is no child corresponding to token PARENTHESES_R
    fn PARENTHESES_R(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PARENTHESES_R, 0)
    }
    fn class_expression_all(&self) -> Vec<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn class_expression(&self, i: usize) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Tuple_blockContextAttrs<'input> for Tuple_blockContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn tuple_block(&mut self) -> Result<Rc<Tuple_blockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Tuple_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_tuple_block);
        let mut _localctx: Rc<Tuple_blockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(370);
                recog.base.match_token(PARENTHESES_L, &mut recog.err_handler)?;

                recog.base.set_state(382);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << OP_NOT) | (1usize << OP_AT) | (1usize << OP_UNTAG) | (1usize << PARENTHESES_L)))
                        != 0)
                    || (((_la - 32) & !0x3f) == 0
                        && ((1usize << (_la - 32))
                            & ((1usize << (INTEGER - 32))
                                | (1usize << (BOOLEAN - 32))
                                | (1usize << (ESCAPED - 32))
                                | (1usize << (REGEX_RANGE - 32))
                                | (1usize << (REGEX_FREE - 32))
                                | (1usize << (STRING_SINGLE - 32))
                                | (1usize << (STRING_DOUBLE - 32))
                                | (1usize << (RAW_ID - 32))
                                | (1usize << (UNICODE_ID - 32))))
                            != 0)
                {
                    {
                        // InvokeRule class_expression
                        recog.base.set_state(371);
                        recog.class_expression_rec(0)?;

                        recog.base.set_state(376);
                        recog.err_handler.sync(&mut recog.base)?;
                        _alt = recog.interpreter.adaptive_predict(42, &mut recog.base)?;
                        while { _alt != 2 && _alt != INVALID_ALT } {
                            if _alt == 1 {
                                {
                                    {
                                        recog.base.set_state(372);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        // InvokeRule class_expression
                                        recog.base.set_state(373);
                                        recog.class_expression_rec(0)?;
                                    }
                                }
                            }
                            recog.base.set_state(378);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(42, &mut recog.base)?;
                        }
                        recog.base.set_state(380);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA {
                            {
                                recog.base.set_state(379);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(384);
                recog.base.match_token(PARENTHESES_R, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- suffix ----------------
#[derive(Debug)]
pub enum SuffixContextAll<'input> {
    MaybeContext(MaybeContext<'input>),
    OptionalContext(OptionalContext<'input>),
    MaybeGreedyContext(MaybeGreedyContext<'input>),
    ManyGreedyContext(ManyGreedyContext<'input>),
    ManyContext(ManyContext<'input>),
    Error(SuffixContext<'input>),
}
antlr_rust::tid! {SuffixContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for SuffixContextAll<'input> {}

impl<'input> YggdrasilAntlrParserContext<'input> for SuffixContextAll<'input> {}

impl<'input> Deref for SuffixContextAll<'input> {
    type Target = dyn SuffixContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use SuffixContextAll::*;
        match self {
            MaybeContext(inner) => inner,
            OptionalContext(inner) => inner,
            MaybeGreedyContext(inner) => inner,
            ManyGreedyContext(inner) => inner,
            ManyContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for SuffixContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for SuffixContextAll<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type SuffixContext<'input> = BaseParserRuleContext<'input, SuffixContextExt<'input>>;

#[derive(Clone)]
pub struct SuffixContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for SuffixContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for SuffixContext<'input> {}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for SuffixContext<'input> {}

impl<'input> CustomRuleContext<'input> for SuffixContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_suffix
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_suffix }
}
antlr_rust::tid! {SuffixContextExt<'a>}

impl<'input> SuffixContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SuffixContextAll<'input>> {
        Rc::new(SuffixContextAll::Error(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SuffixContextExt { ph: PhantomData },
        )))
    }
}

pub trait SuffixContextAttrs<'input>: YggdrasilAntlrParserContext<'input> + BorrowMut<SuffixContextExt<'input>> {}

impl<'input> SuffixContextAttrs<'input> for SuffixContext<'input> {}

pub type MaybeContext<'input> = BaseParserRuleContext<'input, MaybeContextExt<'input>>;

pub trait MaybeContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token MATCH_MANY
    /// Returns `None` if there is no child corresponding to token MATCH_MANY
    fn MATCH_MANY(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH_MANY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MATCH_OPTIONAL
    /// Returns `None` if there is no child corresponding to token MATCH_OPTIONAL
    fn MATCH_OPTIONAL(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH_OPTIONAL, 0)
    }
}

impl<'input> MaybeContextAttrs<'input> for MaybeContext<'input> {}

pub struct MaybeContextExt<'input> {
    base: SuffixContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {MaybeContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for MaybeContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for MaybeContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_Maybe(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_Maybe(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for MaybeContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_Maybe(self);
    }
}

impl<'input> CustomRuleContext<'input> for MaybeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_suffix
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_suffix }
}

impl<'input> Borrow<SuffixContextExt<'input>> for MaybeContext<'input> {
    fn borrow(&self) -> &SuffixContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<SuffixContextExt<'input>> for MaybeContext<'input> {
    fn borrow_mut(&mut self) -> &mut SuffixContextExt<'input> {
        &mut self.base
    }
}

impl<'input> SuffixContextAttrs<'input> for MaybeContext<'input> {}

impl<'input> MaybeContextExt<'input> {
    fn new(ctx: &dyn SuffixContextAttrs<'input>) -> Rc<SuffixContextAll<'input>> {
        Rc::new(SuffixContextAll::MaybeContext(BaseParserRuleContext::copy_from(
            ctx,
            MaybeContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type OptionalContext<'input> = BaseParserRuleContext<'input, OptionalContextExt<'input>>;

pub trait OptionalContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token MATCH_OPTIONAL
    /// Returns `None` if there is no child corresponding to token MATCH_OPTIONAL
    fn MATCH_OPTIONAL(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH_OPTIONAL, 0)
    }
}

impl<'input> OptionalContextAttrs<'input> for OptionalContext<'input> {}

pub struct OptionalContextExt<'input> {
    base: SuffixContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {OptionalContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for OptionalContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for OptionalContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_Optional(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_Optional(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for OptionalContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_Optional(self);
    }
}

impl<'input> CustomRuleContext<'input> for OptionalContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_suffix
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_suffix }
}

impl<'input> Borrow<SuffixContextExt<'input>> for OptionalContext<'input> {
    fn borrow(&self) -> &SuffixContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<SuffixContextExt<'input>> for OptionalContext<'input> {
    fn borrow_mut(&mut self) -> &mut SuffixContextExt<'input> {
        &mut self.base
    }
}

impl<'input> SuffixContextAttrs<'input> for OptionalContext<'input> {}

impl<'input> OptionalContextExt<'input> {
    fn new(ctx: &dyn SuffixContextAttrs<'input>) -> Rc<SuffixContextAll<'input>> {
        Rc::new(SuffixContextAll::OptionalContext(BaseParserRuleContext::copy_from(
            ctx,
            OptionalContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type MaybeGreedyContext<'input> = BaseParserRuleContext<'input, MaybeGreedyContextExt<'input>>;

pub trait MaybeGreedyContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token MATCH_MANY
    /// Returns `None` if there is no child corresponding to token MATCH_MANY
    fn MATCH_MANY(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH_MANY, 0)
    }
}

impl<'input> MaybeGreedyContextAttrs<'input> for MaybeGreedyContext<'input> {}

pub struct MaybeGreedyContextExt<'input> {
    base: SuffixContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {MaybeGreedyContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for MaybeGreedyContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for MaybeGreedyContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_MaybeGreedy(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_MaybeGreedy(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for MaybeGreedyContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_MaybeGreedy(self);
    }
}

impl<'input> CustomRuleContext<'input> for MaybeGreedyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_suffix
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_suffix }
}

impl<'input> Borrow<SuffixContextExt<'input>> for MaybeGreedyContext<'input> {
    fn borrow(&self) -> &SuffixContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<SuffixContextExt<'input>> for MaybeGreedyContext<'input> {
    fn borrow_mut(&mut self) -> &mut SuffixContextExt<'input> {
        &mut self.base
    }
}

impl<'input> SuffixContextAttrs<'input> for MaybeGreedyContext<'input> {}

impl<'input> MaybeGreedyContextExt<'input> {
    fn new(ctx: &dyn SuffixContextAttrs<'input>) -> Rc<SuffixContextAll<'input>> {
        Rc::new(SuffixContextAll::MaybeGreedyContext(BaseParserRuleContext::copy_from(
            ctx,
            MaybeGreedyContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type ManyGreedyContext<'input> = BaseParserRuleContext<'input, ManyGreedyContextExt<'input>>;

pub trait ManyGreedyContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token MATCH_MANY1
    /// Returns `None` if there is no child corresponding to token MATCH_MANY1
    fn MATCH_MANY1(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH_MANY1, 0)
    }
}

impl<'input> ManyGreedyContextAttrs<'input> for ManyGreedyContext<'input> {}

pub struct ManyGreedyContextExt<'input> {
    base: SuffixContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ManyGreedyContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for ManyGreedyContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for ManyGreedyContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ManyGreedy(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_ManyGreedy(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for ManyGreedyContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_ManyGreedy(self);
    }
}

impl<'input> CustomRuleContext<'input> for ManyGreedyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_suffix
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_suffix }
}

impl<'input> Borrow<SuffixContextExt<'input>> for ManyGreedyContext<'input> {
    fn borrow(&self) -> &SuffixContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<SuffixContextExt<'input>> for ManyGreedyContext<'input> {
    fn borrow_mut(&mut self) -> &mut SuffixContextExt<'input> {
        &mut self.base
    }
}

impl<'input> SuffixContextAttrs<'input> for ManyGreedyContext<'input> {}

impl<'input> ManyGreedyContextExt<'input> {
    fn new(ctx: &dyn SuffixContextAttrs<'input>) -> Rc<SuffixContextAll<'input>> {
        Rc::new(SuffixContextAll::ManyGreedyContext(BaseParserRuleContext::copy_from(
            ctx,
            ManyGreedyContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type ManyContext<'input> = BaseParserRuleContext<'input, ManyContextExt<'input>>;

pub trait ManyContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token MATCH_MANY1
    /// Returns `None` if there is no child corresponding to token MATCH_MANY1
    fn MATCH_MANY1(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH_MANY1, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MATCH_OPTIONAL
    /// Returns `None` if there is no child corresponding to token MATCH_OPTIONAL
    fn MATCH_OPTIONAL(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH_OPTIONAL, 0)
    }
}

impl<'input> ManyContextAttrs<'input> for ManyContext<'input> {}

pub struct ManyContextExt<'input> {
    base: SuffixContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ManyContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for ManyContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for ManyContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_Many(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_Many(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for ManyContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_Many(self);
    }
}

impl<'input> CustomRuleContext<'input> for ManyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_suffix
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_suffix }
}

impl<'input> Borrow<SuffixContextExt<'input>> for ManyContext<'input> {
    fn borrow(&self) -> &SuffixContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<SuffixContextExt<'input>> for ManyContext<'input> {
    fn borrow_mut(&mut self) -> &mut SuffixContextExt<'input> {
        &mut self.base
    }
}

impl<'input> SuffixContextAttrs<'input> for ManyContext<'input> {}

impl<'input> ManyContextExt<'input> {
    fn new(ctx: &dyn SuffixContextAttrs<'input>) -> Rc<SuffixContextAll<'input>> {
        Rc::new(SuffixContextAll::ManyContext(BaseParserRuleContext::copy_from(
            ctx,
            ManyContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn suffix(&mut self) -> Result<Rc<SuffixContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SuffixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_suffix);
        let mut _localctx: Rc<SuffixContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(393);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(45, &mut recog.base)? {
                1 => {
                    let tmp = OptionalContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(386);
                        recog.base.match_token(MATCH_OPTIONAL, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    let tmp = MaybeGreedyContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(387);
                        recog.base.match_token(MATCH_MANY, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    let tmp = MaybeContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(388);
                        recog.base.match_token(MATCH_MANY, &mut recog.err_handler)?;

                        recog.base.set_state(389);
                        recog.base.match_token(MATCH_OPTIONAL, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    let tmp = ManyGreedyContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        recog.base.set_state(390);
                        recog.base.match_token(MATCH_MANY1, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    let tmp = ManyContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        recog.base.set_state(391);
                        recog.base.match_token(MATCH_MANY1, &mut recog.err_handler)?;

                        recog.base.set_state(392);
                        recog.base.match_token(MATCH_OPTIONAL, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- atomic ----------------
#[derive(Debug)]
pub enum AtomicContextAll<'input> {
    AIntContext(AIntContext<'input>),
    AReContext(AReContext<'input>),
    ACharContext(ACharContext<'input>),
    AGroupContext(AGroupContext<'input>),
    ABoolContext(ABoolContext<'input>),
    ACallContext(ACallContext<'input>),
    AIdContext(AIdContext<'input>),
    AStringContext(AStringContext<'input>),
    Error(AtomicContext<'input>),
}
antlr_rust::tid! {AtomicContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for AtomicContextAll<'input> {}

impl<'input> YggdrasilAntlrParserContext<'input> for AtomicContextAll<'input> {}

impl<'input> Deref for AtomicContextAll<'input> {
    type Target = dyn AtomicContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use AtomicContextAll::*;
        match self {
            AIntContext(inner) => inner,
            AReContext(inner) => inner,
            ACharContext(inner) => inner,
            AGroupContext(inner) => inner,
            ABoolContext(inner) => inner,
            ACallContext(inner) => inner,
            AIdContext(inner) => inner,
            AStringContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AtomicContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AtomicContextAll<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type AtomicContext<'input> = BaseParserRuleContext<'input, AtomicContextExt<'input>>;

#[derive(Clone)]
pub struct AtomicContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for AtomicContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AtomicContext<'input> {}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AtomicContext<'input> {}

impl<'input> CustomRuleContext<'input> for AtomicContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}
antlr_rust::tid! {AtomicContextExt<'a>}

impl<'input> AtomicContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::Error(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AtomicContextExt { ph: PhantomData },
        )))
    }
}

pub trait AtomicContextAttrs<'input>: YggdrasilAntlrParserContext<'input> + BorrowMut<AtomicContextExt<'input>> {}

impl<'input> AtomicContextAttrs<'input> for AtomicContext<'input> {}

pub type AIntContext<'input> = BaseParserRuleContext<'input, AIntContextExt<'input>>;

pub trait AIntContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token INTEGER
    /// Returns `None` if there is no child corresponding to token INTEGER
    fn INTEGER(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INTEGER, 0)
    }
}

impl<'input> AIntContextAttrs<'input> for AIntContext<'input> {}

pub struct AIntContextExt<'input> {
    base: AtomicContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AIntContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for AIntContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AIntContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_AInt(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_AInt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AIntContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_AInt(self);
    }
}

impl<'input> CustomRuleContext<'input> for AIntContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}

impl<'input> Borrow<AtomicContextExt<'input>> for AIntContext<'input> {
    fn borrow(&self) -> &AtomicContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<AtomicContextExt<'input>> for AIntContext<'input> {
    fn borrow_mut(&mut self) -> &mut AtomicContextExt<'input> {
        &mut self.base
    }
}

impl<'input> AtomicContextAttrs<'input> for AIntContext<'input> {}

impl<'input> AIntContextExt<'input> {
    fn new(ctx: &dyn AtomicContextAttrs<'input>) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::AIntContext(BaseParserRuleContext::copy_from(
            ctx,
            AIntContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type AReContext<'input> = BaseParserRuleContext<'input, AReContextExt<'input>>;

pub trait AReContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn regex(&self) -> Option<Rc<RegexContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AReContextAttrs<'input> for AReContext<'input> {}

pub struct AReContextExt<'input> {
    base: AtomicContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AReContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for AReContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AReContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ARe(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_ARe(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AReContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_ARe(self);
    }
}

impl<'input> CustomRuleContext<'input> for AReContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}

impl<'input> Borrow<AtomicContextExt<'input>> for AReContext<'input> {
    fn borrow(&self) -> &AtomicContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<AtomicContextExt<'input>> for AReContext<'input> {
    fn borrow_mut(&mut self) -> &mut AtomicContextExt<'input> {
        &mut self.base
    }
}

impl<'input> AtomicContextAttrs<'input> for AReContext<'input> {}

impl<'input> AReContextExt<'input> {
    fn new(ctx: &dyn AtomicContextAttrs<'input>) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::AReContext(BaseParserRuleContext::copy_from(
            ctx,
            AReContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type ACharContext<'input> = BaseParserRuleContext<'input, ACharContextExt<'input>>;

pub trait ACharContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token ESCAPED
    /// Returns `None` if there is no child corresponding to token ESCAPED
    fn ESCAPED(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ESCAPED, 0)
    }
}

impl<'input> ACharContextAttrs<'input> for ACharContext<'input> {}

pub struct ACharContextExt<'input> {
    base: AtomicContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ACharContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for ACharContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for ACharContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_AChar(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_AChar(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for ACharContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_AChar(self);
    }
}

impl<'input> CustomRuleContext<'input> for ACharContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}

impl<'input> Borrow<AtomicContextExt<'input>> for ACharContext<'input> {
    fn borrow(&self) -> &AtomicContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<AtomicContextExt<'input>> for ACharContext<'input> {
    fn borrow_mut(&mut self) -> &mut AtomicContextExt<'input> {
        &mut self.base
    }
}

impl<'input> AtomicContextAttrs<'input> for ACharContext<'input> {}

impl<'input> ACharContextExt<'input> {
    fn new(ctx: &dyn AtomicContextAttrs<'input>) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::ACharContext(BaseParserRuleContext::copy_from(
            ctx,
            ACharContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type AGroupContext<'input> = BaseParserRuleContext<'input, AGroupContextExt<'input>>;

pub trait AGroupContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token PARENTHESES_L
    /// Returns `None` if there is no child corresponding to token PARENTHESES_L
    fn PARENTHESES_L(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PARENTHESES_L, 0)
    }
    fn class_expression(&self) -> Option<Rc<Class_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PARENTHESES_R
    /// Returns `None` if there is no child corresponding to token PARENTHESES_R
    fn PARENTHESES_R(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PARENTHESES_R, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OP_OR
    /// Returns `None` if there is no child corresponding to token OP_OR
    fn OP_OR(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_OR, 0)
    }
}

impl<'input> AGroupContextAttrs<'input> for AGroupContext<'input> {}

pub struct AGroupContextExt<'input> {
    base: AtomicContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AGroupContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for AGroupContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AGroupContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_AGroup(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_AGroup(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AGroupContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_AGroup(self);
    }
}

impl<'input> CustomRuleContext<'input> for AGroupContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}

impl<'input> Borrow<AtomicContextExt<'input>> for AGroupContext<'input> {
    fn borrow(&self) -> &AtomicContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<AtomicContextExt<'input>> for AGroupContext<'input> {
    fn borrow_mut(&mut self) -> &mut AtomicContextExt<'input> {
        &mut self.base
    }
}

impl<'input> AtomicContextAttrs<'input> for AGroupContext<'input> {}

impl<'input> AGroupContextExt<'input> {
    fn new(ctx: &dyn AtomicContextAttrs<'input>) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::AGroupContext(BaseParserRuleContext::copy_from(
            ctx,
            AGroupContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type ABoolContext<'input> = BaseParserRuleContext<'input, ABoolContextExt<'input>>;

pub trait ABoolContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token BOOLEAN
    /// Returns `None` if there is no child corresponding to token BOOLEAN
    fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BOOLEAN, 0)
    }
}

impl<'input> ABoolContextAttrs<'input> for ABoolContext<'input> {}

pub struct ABoolContextExt<'input> {
    base: AtomicContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ABoolContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for ABoolContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for ABoolContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ABool(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_ABool(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for ABoolContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_ABool(self);
    }
}

impl<'input> CustomRuleContext<'input> for ABoolContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}

impl<'input> Borrow<AtomicContextExt<'input>> for ABoolContext<'input> {
    fn borrow(&self) -> &AtomicContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<AtomicContextExt<'input>> for ABoolContext<'input> {
    fn borrow_mut(&mut self) -> &mut AtomicContextExt<'input> {
        &mut self.base
    }
}

impl<'input> AtomicContextAttrs<'input> for ABoolContext<'input> {}

impl<'input> ABoolContextExt<'input> {
    fn new(ctx: &dyn AtomicContextAttrs<'input>) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::ABoolContext(BaseParserRuleContext::copy_from(
            ctx,
            ABoolContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type ACallContext<'input> = BaseParserRuleContext<'input, ACallContextExt<'input>>;

pub trait ACallContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn macro_call(&self) -> Option<Rc<Macro_callContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ACallContextAttrs<'input> for ACallContext<'input> {}

pub struct ACallContextExt<'input> {
    base: AtomicContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ACallContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for ACallContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for ACallContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ACall(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_ACall(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for ACallContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_ACall(self);
    }
}

impl<'input> CustomRuleContext<'input> for ACallContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}

impl<'input> Borrow<AtomicContextExt<'input>> for ACallContext<'input> {
    fn borrow(&self) -> &AtomicContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<AtomicContextExt<'input>> for ACallContext<'input> {
    fn borrow_mut(&mut self) -> &mut AtomicContextExt<'input> {
        &mut self.base
    }
}

impl<'input> AtomicContextAttrs<'input> for ACallContext<'input> {}

impl<'input> ACallContextExt<'input> {
    fn new(ctx: &dyn AtomicContextAttrs<'input>) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::ACallContext(BaseParserRuleContext::copy_from(
            ctx,
            ACallContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type AIdContext<'input> = BaseParserRuleContext<'input, AIdContextExt<'input>>;

pub trait AIdContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AIdContextAttrs<'input> for AIdContext<'input> {}

pub struct AIdContextExt<'input> {
    base: AtomicContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AIdContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for AIdContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AIdContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_AId(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_AId(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AIdContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_AId(self);
    }
}

impl<'input> CustomRuleContext<'input> for AIdContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}

impl<'input> Borrow<AtomicContextExt<'input>> for AIdContext<'input> {
    fn borrow(&self) -> &AtomicContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<AtomicContextExt<'input>> for AIdContext<'input> {
    fn borrow_mut(&mut self) -> &mut AtomicContextExt<'input> {
        &mut self.base
    }
}

impl<'input> AtomicContextAttrs<'input> for AIdContext<'input> {}

impl<'input> AIdContextExt<'input> {
    fn new(ctx: &dyn AtomicContextAttrs<'input>) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::AIdContext(BaseParserRuleContext::copy_from(
            ctx,
            AIdContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

pub type AStringContext<'input> = BaseParserRuleContext<'input, AStringContextExt<'input>>;

pub trait AStringContextAttrs<'input>: YggdrasilAntlrParserContext<'input> {
    fn string(&self) -> Option<Rc<StringContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AStringContextAttrs<'input> for AStringContext<'input> {}

pub struct AStringContextExt<'input> {
    base: AtomicContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {AStringContextExt<'a>}

impl<'input> YggdrasilAntlrParserContext<'input> for AStringContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for AStringContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_AString(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_AString(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for AStringContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_AString(self);
    }
}

impl<'input> CustomRuleContext<'input> for AStringContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atomic
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_atomic }
}

impl<'input> Borrow<AtomicContextExt<'input>> for AStringContext<'input> {
    fn borrow(&self) -> &AtomicContextExt<'input> {
        &self.base
    }
}

impl<'input> BorrowMut<AtomicContextExt<'input>> for AStringContext<'input> {
    fn borrow_mut(&mut self) -> &mut AtomicContextExt<'input> {
        &mut self.base
    }
}

impl<'input> AtomicContextAttrs<'input> for AStringContext<'input> {}

impl<'input> AStringContextExt<'input> {
    fn new(ctx: &dyn AtomicContextAttrs<'input>) -> Rc<AtomicContextAll<'input>> {
        Rc::new(AtomicContextAll::AStringContext(BaseParserRuleContext::copy_from(
            ctx,
            AStringContextExt { base: ctx.borrow().clone(), ph: PhantomData },
        )))
    }
}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn atomic(&mut self) -> Result<Rc<AtomicContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AtomicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_atomic);
        let mut _localctx: Rc<AtomicContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(409);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                PARENTHESES_L => {
                    let tmp = AGroupContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(395);
                        recog.base.match_token(PARENTHESES_L, &mut recog.err_handler)?;

                        recog.base.set_state(397);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == OP_OR {
                            {
                                recog.base.set_state(396);
                                recog.base.match_token(OP_OR, &mut recog.err_handler)?;
                            }
                        }

                        // InvokeRule class_expression
                        recog.base.set_state(399);
                        recog.class_expression_rec(0)?;

                        recog.base.set_state(400);
                        recog.base.match_token(PARENTHESES_R, &mut recog.err_handler)?;
                    }
                }

                OP_AT => {
                    let tmp = ACallContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        // InvokeRule macro_call
                        recog.base.set_state(402);
                        recog.macro_call()?;
                    }
                }

                STRING_SINGLE | STRING_DOUBLE => {
                    let tmp = AStringContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        // InvokeRule string
                        recog.base.set_state(403);
                        recog.string()?;
                    }
                }

                RAW_ID | UNICODE_ID => {
                    let tmp = AIdContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        // InvokeRule identifier
                        recog.base.set_state(404);
                        recog.identifier()?;
                    }
                }

                REGEX_RANGE | REGEX_FREE => {
                    let tmp = AReContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        // InvokeRule regex
                        recog.base.set_state(405);
                        recog.regex()?;
                    }
                }

                INTEGER => {
                    let tmp = AIntContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        recog.base.set_state(406);
                        recog.base.match_token(INTEGER, &mut recog.err_handler)?;
                    }
                }

                BOOLEAN => {
                    let tmp = ABoolContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 7);
                    _localctx = tmp;
                    {
                        recog.base.set_state(407);
                        recog.base.match_token(BOOLEAN, &mut recog.err_handler)?;
                    }
                }

                ESCAPED => {
                    let tmp = ACharContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 8);
                    _localctx = tmp;
                    {
                        recog.base.set_state(408);
                        recog.base.match_token(ESCAPED, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- regex ----------------
pub type RegexContextAll<'input> = RegexContext<'input>;

pub type RegexContext<'input> = BaseParserRuleContext<'input, RegexContextExt<'input>>;

#[derive(Clone)]
pub struct RegexContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for RegexContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for RegexContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_regex(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_regex(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for RegexContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_regex(self);
    }
}

impl<'input> CustomRuleContext<'input> for RegexContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_regex
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_regex }
}
antlr_rust::tid! {RegexContextExt<'a>}

impl<'input> RegexContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<RegexContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, RegexContextExt { ph: PhantomData }))
    }
}

pub trait RegexContextAttrs<'input>: YggdrasilAntlrParserContext<'input> + BorrowMut<RegexContextExt<'input>> {
    /// Retrieves first TerminalNode corresponding to token REGEX_RANGE
    /// Returns `None` if there is no child corresponding to token REGEX_RANGE
    fn REGEX_RANGE(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REGEX_RANGE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REGEX_FREE
    /// Returns `None` if there is no child corresponding to token REGEX_FREE
    fn REGEX_FREE(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REGEX_FREE, 0)
    }
}

impl<'input> RegexContextAttrs<'input> for RegexContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn regex(&mut self) -> Result<Rc<RegexContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = RegexContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_regex);
        let mut _localctx: Rc<RegexContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(411);
                _la = recog.base.input.la(1);
                if { !(_la == REGEX_RANGE || _la == REGEX_FREE) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                }
                else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- namepath ----------------
pub type NamepathContextAll<'input> = NamepathContext<'input>;

pub type NamepathContext<'input> = BaseParserRuleContext<'input, NamepathContextExt<'input>>;

#[derive(Clone)]
pub struct NamepathContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for NamepathContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for NamepathContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_namepath(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_namepath(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for NamepathContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_namepath(self);
    }
}

impl<'input> CustomRuleContext<'input> for NamepathContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_namepath
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_namepath }
}
antlr_rust::tid! {NamepathContextExt<'a>}

impl<'input> NamepathContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NamepathContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, NamepathContextExt { ph: PhantomData }))
    }
}

pub trait NamepathContextAttrs<'input>: YggdrasilAntlrParserContext<'input> + BorrowMut<NamepathContextExt<'input>> {
    fn identifier_all(&self) -> Vec<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OP_PROPORTION in current rule
    fn OP_PROPORTION_all(&self) -> Vec<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OP_PROPORTION, starting from 0.
    /// Returns `None` if number of children corresponding to token OP_PROPORTION is less or equal than `i`.
    fn OP_PROPORTION(&self, i: usize) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OP_PROPORTION, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
}

impl<'input> NamepathContextAttrs<'input> for NamepathContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn namepath(&mut self) -> Result<Rc<NamepathContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NamepathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_namepath);
        let mut _localctx: Rc<NamepathContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                // InvokeRule identifier
                recog.base.set_state(413);
                recog.identifier()?;

                recog.base.set_state(418);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(48, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(414);
                                _la = recog.base.input.la(1);
                                if { !(_la == DOT || _la == OP_PROPORTION) } {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                }
                                else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                                // InvokeRule identifier
                                recog.base.set_state(415);
                                recog.identifier()?;
                            }
                        }
                    }
                    recog.base.set_state(420);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(48, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- string ----------------
pub type StringContextAll<'input> = StringContext<'input>;

pub type StringContext<'input> = BaseParserRuleContext<'input, StringContextExt<'input>>;

#[derive(Clone)]
pub struct StringContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for StringContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for StringContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_string(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_string(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for StringContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_string(self);
    }
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_string
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_string }
}
antlr_rust::tid! {StringContextExt<'a>}

impl<'input> StringContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StringContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, StringContextExt { ph: PhantomData }))
    }
}

pub trait StringContextAttrs<'input>: YggdrasilAntlrParserContext<'input> + BorrowMut<StringContextExt<'input>> {
    /// Retrieves first TerminalNode corresponding to token STRING_SINGLE
    /// Returns `None` if there is no child corresponding to token STRING_SINGLE
    fn STRING_SINGLE(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_SINGLE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_DOUBLE
    /// Returns `None` if there is no child corresponding to token STRING_DOUBLE
    fn STRING_DOUBLE(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_DOUBLE, 0)
    }
}

impl<'input> StringContextAttrs<'input> for StringContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn string(&mut self) -> Result<Rc<StringContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_string);
        let mut _localctx: Rc<StringContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(421);
                _la = recog.base.input.la(1);
                if { !(_la == STRING_SINGLE || _la == STRING_DOUBLE) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                }
                else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- identifier ----------------
pub type IdentifierContextAll<'input> = IdentifierContext<'input>;

pub type IdentifierContext<'input> = BaseParserRuleContext<'input, IdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YggdrasilAntlrParserContext<'input> for IdentifierContext<'input> {}

impl<'input, 'a> Listenable<dyn YggdrasilAntlrListener<'input> + 'a> for IdentifierContext<'input> {
    fn enter(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_identifier(self);
    }
    fn exit(&self, listener: &mut (dyn YggdrasilAntlrListener<'input> + 'a)) {
        listener.exit_identifier(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YggdrasilAntlrVisitor<'input> + 'a> for IdentifierContext<'input> {
    fn accept(&self, visitor: &mut (dyn YggdrasilAntlrVisitor<'input> + 'a)) {
        visitor.visit_identifier(self);
    }
}

impl<'input> CustomRuleContext<'input> for IdentifierContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YggdrasilAntlrParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_identifier
    }
    // fn type_rule_index() -> usize where Self: Sized { RULE_identifier }
}
antlr_rust::tid! {IdentifierContextExt<'a>}

impl<'input> IdentifierContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YggdrasilAntlrParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<IdentifierContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(parent, invoking_state, IdentifierContextExt { ph: PhantomData }))
    }
}

pub trait IdentifierContextAttrs<'input>:
    YggdrasilAntlrParserContext<'input> + BorrowMut<IdentifierContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token RAW_ID
    /// Returns `None` if there is no child corresponding to token RAW_ID
    fn RAW_ID(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RAW_ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UNICODE_ID
    /// Returns `None` if there is no child corresponding to token UNICODE_ID
    fn UNICODE_ID(&self) -> Option<Rc<TerminalNode<'input, YggdrasilAntlrParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNICODE_ID, 0)
    }
}

impl<'input> IdentifierContextAttrs<'input> for IdentifierContext<'input> {}

impl<'input, I, H> YggdrasilAntlrParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn identifier(&mut self) -> Result<Rc<IdentifierContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = IdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_identifier);
        let mut _localctx: Rc<IdentifierContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            // recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(423);
                _la = recog.base.input.la(1);
                if { !(_la == RAW_ID || _la == UNICODE_ID) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                }
                else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> = Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str = "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x2e\u{1ac}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x03\
	\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x07\
	\x02\x52\x0a\x02\x0c\x02\x0e\x02\x55\x0b\x02\x03\x02\x03\x02\x03\x03\x03\
	\x03\x03\x03\x05\x03\x5c\x0a\x03\x03\x03\x05\x03\x5f\x0a\x03\x03\x04\x03\
	\x04\x07\x04\x63\x0a\x04\x0c\x04\x0e\x04\x66\x0b\x04\x03\x04\x03\x04\x03\
	\x05\x03\x05\x03\x05\x03\x05\x05\x05\x6e\x0a\x05\x03\x05\x03\x05\x03\x06\
	\x03\x06\x03\x06\x03\x06\x07\x06\x76\x0a\x06\x0c\x06\x0e\x06\x79\x0b\x06\
	\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x05\x08\
	\u{83}\x0a\x08\x03\x09\x03\x09\x03\x09\x05\x09\u{88}\x0a\x09\x03\x0a\x07\
	\x0a\u{8b}\x0a\x0a\x0c\x0a\x0e\x0a\u{8e}\x0b\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x05\x0a\u{95}\x0a\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x05\
	\x0b\u{9b}\x0a\x0b\x03\x0b\x07\x0b\u{9e}\x0a\x0b\x0c\x0b\x0e\x0b\u{a1}\x0b\
	\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{af}\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{bb}\x0a\
	\x0c\x0c\x0c\x0e\x0c\u{be}\x0b\x0c\x03\x0d\x07\x0d\u{c1}\x0a\x0d\x0c\x0d\
	\x0e\x0d\u{c4}\x0b\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{cb}\
	\x0a\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x07\x0e\u{d1}\x0a\x0e\x0c\x0e\x0e\
	\x0e\u{d4}\x0b\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x07\x0f\u{da}\x0a\x0f\
	\x0c\x0f\x0e\x0f\u{dd}\x0b\x0f\x03\x0f\x05\x0f\u{e0}\x0a\x0f\x03\x10\x03\
	\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x05\
	\x10\u{ec}\x0a\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\
	\x07\x10\u{f5}\x0a\x10\x0c\x10\x0e\x10\u{f8}\x0b\x10\x03\x11\x07\x11\u{fb}\
	\x0a\x11\x0c\x11\x0e\x11\u{fe}\x0b\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\
	\x11\x03\x12\x03\x12\x03\x12\x05\x12\u{108}\x0a\x12\x03\x13\x07\x13\u{10b}\
	\x0a\x13\x0c\x13\x0e\x13\u{10e}\x0b\x13\x03\x13\x03\x13\x03\x13\x05\x13\
	\u{113}\x0a\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\x07\x14\u{11a}\x0a\
	\x14\x0c\x14\x0e\x14\u{11d}\x0b\x14\x03\x14\x03\x14\x03\x15\x07\x15\u{122}\
	\x0a\x15\x0c\x15\x0e\x15\u{125}\x0b\x15\x03\x15\x03\x15\x03\x15\x03\x15\
	\x03\x15\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x07\x16\u{132}\
	\x0a\x16\x0c\x16\x0e\x16\u{135}\x0b\x16\x03\x17\x07\x17\u{138}\x0a\x17\x0c\
	\x17\x0e\x17\u{13b}\x0b\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\
	\x18\x03\x18\x03\x18\x07\x18\u{145}\x0a\x18\x0c\x18\x0e\x18\u{148}\x0b\x18\
	\x03\x18\x03\x18\x03\x19\x07\x19\u{14d}\x0a\x19\x0c\x19\x0e\x19\u{150}\x0b\
	\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x1a\x07\x1a\u{157}\x0a\x1a\x0c\
	\x1a\x0e\x1a\u{15a}\x0b\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{165}\x0a\x1b\x03\x1b\x05\x1b\u{168}\
	\x0a\x1b\x03\x1c\x07\x1c\u{16b}\x0a\x1c\x0c\x1c\x0e\x1c\u{16e}\x0b\x1c\x03\
	\x1d\x03\x1d\x03\x1d\x05\x1d\u{173}\x0a\x1d\x03\x1e\x03\x1e\x03\x1e\x03\
	\x1e\x07\x1e\u{179}\x0a\x1e\x0c\x1e\x0e\x1e\u{17c}\x0b\x1e\x03\x1e\x05\x1e\
	\u{17f}\x0a\x1e\x05\x1e\u{181}\x0a\x1e\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{18c}\x0a\x1f\x03\x20\x03\
	\x20\x05\x20\u{190}\x0a\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\
	\x20\x03\x20\x03\x20\x03\x20\x03\x20\x05\x20\u{19c}\x0a\x20\x03\x21\x03\
	\x21\x03\x22\x03\x22\x03\x22\x07\x22\u{1a3}\x0a\x22\x0c\x22\x0e\x22\u{1a6}\
	\x0b\x22\x03\x23\x03\x23\x03\x24\x03\x24\x03\x24\x02\x05\x16\x1e\x2a\x25\
	\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\
	\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x02\
	\x07\x03\x02\x0b\x0c\x03\x02\x25\x26\x04\x02\x03\x03\x1b\x1b\x03\x02\x27\
	\x28\x03\x02\x29\x2a\x02\u{1d3}\x02\x53\x03\x02\x02\x02\x04\x58\x03\x02\
	\x02\x02\x06\x60\x03\x02\x02\x02\x08\x69\x03\x02\x02\x02\x0a\x71\x03\x02\
	\x02\x02\x0c\x7c\x03\x02\x02\x02\x0e\u{82}\x03\x02\x02\x02\x10\u{87}\x03\
	\x02\x02\x02\x12\u{8c}\x03\x02\x02\x02\x14\u{98}\x03\x02\x02\x02\x16\u{ae}\
	\x03\x02\x02\x02\x18\u{c2}\x03\x02\x02\x02\x1a\u{ce}\x03\x02\x02\x02\x1c\
	\u{d7}\x03\x02\x02\x02\x1e\u{eb}\x03\x02\x02\x02\x20\u{fc}\x03\x02\x02\x02\
	\x22\u{104}\x03\x02\x02\x02\x24\u{10c}\x03\x02\x02\x02\x26\u{116}\x03\x02\
	\x02\x02\x28\u{123}\x03\x02\x02\x02\x2a\u{12b}\x03\x02\x02\x02\x2c\u{139}\
	\x03\x02\x02\x02\x2e\u{141}\x03\x02\x02\x02\x30\u{14e}\x03\x02\x02\x02\x32\
	\u{158}\x03\x02\x02\x02\x34\u{160}\x03\x02\x02\x02\x36\u{16c}\x03\x02\x02\
	\x02\x38\u{16f}\x03\x02\x02\x02\x3a\u{174}\x03\x02\x02\x02\x3c\u{18b}\x03\
	\x02\x02\x02\x3e\u{19b}\x03\x02\x02\x02\x40\u{19d}\x03\x02\x02\x02\x42\u{19f}\
	\x03\x02\x02\x02\x44\u{1a7}\x03\x02\x02\x02\x46\u{1a9}\x03\x02\x02\x02\x48\
	\x52\x05\x08\x05\x02\x49\x52\x05\x04\x03\x02\x4a\x52\x05\x12\x0a\x02\x4b\
	\x52\x05\x18\x0d\x02\x4c\x52\x05\x20\x11\x02\x4d\x52\x05\x24\x13\x02\x4e\
	\x52\x05\x2c\x17\x02\x4f\x52\x05\x32\x1a\x02\x50\x52\x07\x06\x02\x02\x51\
	\x48\x03\x02\x02\x02\x51\x49\x03\x02\x02\x02\x51\x4a\x03\x02\x02\x02\x51\
	\x4b\x03\x02\x02\x02\x51\x4c\x03\x02\x02\x02\x51\x4d\x03\x02\x02\x02\x51\
	\x4e\x03\x02\x02\x02\x51\x4f\x03\x02\x02\x02\x51\x50\x03\x02\x02\x02\x52\
	\x55\x03\x02\x02\x02\x53\x51\x03\x02\x02\x02\x53\x54\x03\x02\x02\x02\x54\
	\x56\x03\x02\x02\x02\x55\x53\x03\x02\x02\x02\x56\x57\x07\x02\x02\x03\x57\
	\x03\x03\x02\x02\x02\x58\x5b\x07\x13\x02\x02\x59\x5c\x05\x46\x24\x02\x5a\
	\x5c\x05\x44\x23\x02\x5b\x59\x03\x02\x02\x02\x5b\x5a\x03\x02\x02\x02\x5c\
	\x5e\x03\x02\x02\x02\x5d\x5f\x05\x06\x04\x02\x5e\x5d\x03\x02\x02\x02\x5e\
	\x5f\x03\x02\x02\x02\x5f\x05\x03\x02\x02\x02\x60\x64\x07\x20\x02\x02\x61\
	\x63\x05\x46\x24\x02\x62\x61\x03\x02\x02\x02\x63\x66\x03\x02\x02\x02\x64\
	\x62\x03\x02\x02\x02\x64\x65\x03\x02\x02\x02\x65\x67\x03\x02\x02\x02\x66\
	\x64\x03\x02\x02\x02\x67\x68\x07\x21\x02\x02\x68\x07\x03\x02\x02\x02\x69\
	\x6a\x07\x12\x02\x02\x6a\x6d\x05\x46\x24\x02\x6b\x6c\x07\x05\x02\x02\x6c\
	\x6e\x05\x46\x24\x02\x6d\x6b\x03\x02\x02\x02\x6d\x6e\x03\x02\x02\x02\x6e\
	\x6f\x03\x02\x02\x02\x6f\x70\x05\x0a\x06\x02\x70\x09\x03\x02\x02\x02\x71\
	\x77\x07\x20\x02\x02\x72\x76\x05\x0c\x07\x02\x73\x76\x07\x06\x02\x02\x74\
	\x76\x07\x04\x02\x02\x75\x72\x03\x02\x02\x02\x75\x73\x03\x02\x02\x02\x75\
	\x74\x03\x02\x02\x02\x76\x79\x03\x02\x02\x02\x77\x75\x03\x02\x02\x02\x77\
	\x78\x03\x02\x02\x02\x78\x7a\x03\x02\x02\x02\x79\x77\x03\x02\x02\x02\x7a\
	\x7b\x07\x21\x02\x02\x7b\x0b\x03\x02\x02\x02\x7c\x7d\x05\x0e\x08\x02\x7d\
	\x7e\x07\x05\x02\x02\x7e\x7f\x05\x10\x09\x02\x7f\x0d\x03\x02\x02\x02\u{80}\
	\u{83}\x05\x44\x23\x02\u{81}\u{83}\x05\x46\x24\x02\u{82}\u{80}\x03\x02\x02\
	\x02\u{82}\u{81}\x03\x02\x02\x02\u{83}\x0f\x03\x02\x02\x02\u{84}\u{88}\x05\
	\x44\x23\x02\u{85}\u{88}\x05\x42\x22\x02\u{86}\u{88}\x07\x23\x02\x02\u{87}\
	\u{84}\x03\x02\x02\x02\u{87}\u{85}\x03\x02\x02\x02\u{87}\u{86}\x03\x02\x02\
	\x02\u{88}\x11\x03\x02\x02\x02\u{89}\u{8b}\x05\x34\x1b\x02\u{8a}\u{89}\x03\
	\x02\x02\x02\u{8b}\u{8e}\x03\x02\x02\x02\u{8c}\u{8a}\x03\x02\x02\x02\u{8c}\
	\u{8d}\x03\x02\x02\x02\u{8d}\u{8f}\x03\x02\x02\x02\u{8e}\u{8c}\x03\x02\x02\
	\x02\u{8f}\u{90}\x05\x36\x1c\x02\u{90}\u{91}\x07\x15\x02\x02\u{91}\u{94}\
	\x05\x46\x24\x02\u{92}\u{93}\x07\x11\x02\x02\u{93}\u{95}\x05\x46\x24\x02\
	\u{94}\u{92}\x03\x02\x02\x02\u{94}\u{95}\x03\x02\x02\x02\u{95}\u{96}\x03\
	\x02\x02\x02\u{96}\u{97}\x05\x14\x0b\x02\u{97}\x13\x03\x02\x02\x02\u{98}\
	\u{9a}\x07\x20\x02\x02\u{99}\u{9b}\x07\x0e\x02\x02\u{9a}\u{99}\x03\x02\x02\
	\x02\u{9a}\u{9b}\x03\x02\x02\x02\u{9b}\u{9f}\x03\x02\x02\x02\u{9c}\u{9e}\
	\x05\x16\x0c\x02\u{9d}\u{9c}\x03\x02\x02\x02\u{9e}\u{a1}\x03\x02\x02\x02\
	\u{9f}\u{9d}\x03\x02\x02\x02\u{9f}\u{a0}\x03\x02\x02\x02\u{a0}\u{a2}\x03\
	\x02\x02\x02\u{a1}\u{9f}\x03\x02\x02\x02\u{a2}\u{a3}\x07\x21\x02\x02\u{a3}\
	\x15\x03\x02\x02\x02\u{a4}\u{a5}\x08\x0c\x01\x02\u{a5}\u{a6}\x05\x46\x24\
	\x02\u{a6}\u{a7}\x07\x05\x02\x02\u{a7}\u{a8}\x05\x16\x0c\x09\u{a8}\u{af}\
	\x03\x02\x02\x02\u{a9}\u{aa}\x07\x10\x02\x02\u{aa}\u{af}\x05\x16\x0c\x08\
	\u{ab}\u{ac}\x07\x0a\x02\x02\u{ac}\u{af}\x05\x16\x0c\x07\u{ad}\u{af}\x05\
	\x3e\x20\x02\u{ae}\u{a4}\x03\x02\x02\x02\u{ae}\u{a9}\x03\x02\x02\x02\u{ae}\
	\u{ab}\x03\x02\x02\x02\u{ae}\u{ad}\x03\x02\x02\x02\u{af}\u{bc}\x03\x02\x02\
	\x02\u{b0}\u{b1}\x0c\x06\x02\x02\u{b1}\u{b2}\x07\x0d\x02\x02\u{b2}\u{bb}\
	\x05\x16\x0c\x07\u{b3}\u{b4}\x0c\x05\x02\x02\u{b4}\u{bb}\x05\x16\x0c\x06\
	\u{b5}\u{b6}\x0c\x04\x02\x02\u{b6}\u{b7}\x07\x0e\x02\x02\u{b7}\u{bb}\x05\
	\x16\x0c\x05\u{b8}\u{b9}\x0c\x0a\x02\x02\u{b9}\u{bb}\x05\x3c\x1f\x02\u{ba}\
	\u{b0}\x03\x02\x02\x02\u{ba}\u{b3}\x03\x02\x02\x02\u{ba}\u{b5}\x03\x02\x02\
	\x02\u{ba}\u{b8}\x03\x02\x02\x02\u{bb}\u{be}\x03\x02\x02\x02\u{bc}\u{ba}\
	\x03\x02\x02\x02\u{bc}\u{bd}\x03\x02\x02\x02\u{bd}\x17\x03\x02\x02\x02\u{be}\
	\u{bc}\x03\x02\x02\x02\u{bf}\u{c1}\x05\x34\x1b\x02\u{c0}\u{bf}\x03\x02\x02\
	\x02\u{c1}\u{c4}\x03\x02\x02\x02\u{c2}\u{c0}\x03\x02\x02\x02\u{c2}\u{c3}\
	\x03\x02\x02\x02\u{c3}\u{c5}\x03\x02\x02\x02\u{c4}\u{c2}\x03\x02\x02\x02\
	\u{c5}\u{c6}\x05\x36\x1c\x02\u{c6}\u{c7}\x07\x16\x02\x02\u{c7}\u{ca}\x05\
	\x46\x24\x02\u{c8}\u{c9}\x07\x11\x02\x02\u{c9}\u{cb}\x05\x46\x24\x02\u{ca}\
	\u{c8}\x03\x02\x02\x02\u{ca}\u{cb}\x03\x02\x02\x02\u{cb}\u{cc}\x03\x02\x02\
	\x02\u{cc}\u{cd}\x05\x1a\x0e\x02\u{cd}\x19\x03\x02\x02\x02\u{ce}\u{d2}\x07\
	\x20\x02\x02\u{cf}\u{d1}\x05\x1c\x0f\x02\u{d0}\u{cf}\x03\x02\x02\x02\u{d1}\
	\u{d4}\x03\x02\x02\x02\u{d2}\u{d0}\x03\x02\x02\x02\u{d2}\u{d3}\x03\x02\x02\
	\x02\u{d3}\u{d5}\x03\x02\x02\x02\u{d4}\u{d2}\x03\x02\x02\x02\u{d5}\u{d6}\
	\x07\x21\x02\x02\u{d6}\x1b\x03\x02\x02\x02\u{d7}\u{db}\x07\x0e\x02\x02\u{d8}\
	\u{da}\x05\x1e\x10\x02\u{d9}\u{d8}\x03\x02\x02\x02\u{da}\u{dd}\x03\x02\x02\
	\x02\u{db}\u{d9}\x03\x02\x02\x02\u{db}\u{dc}\x03\x02\x02\x02\u{dc}\u{df}\
	\x03\x02\x02\x02\u{dd}\u{db}\x03\x02\x02\x02\u{de}\u{e0}\x05\x22\x12\x02\
	\u{df}\u{de}\x03\x02\x02\x02\u{df}\u{e0}\x03\x02\x02\x02\u{e0}\x1d\x03\x02\
	\x02\x02\u{e1}\u{e2}\x08\x10\x01\x02\u{e2}\u{e3}\x05\x46\x24\x02\u{e3}\u{e4}\
	\x07\x05\x02\x02\u{e4}\u{e5}\x05\x1e\x10\x08\u{e5}\u{ec}\x03\x02\x02\x02\
	\u{e6}\u{e7}\x07\x10\x02\x02\u{e7}\u{ec}\x05\x1e\x10\x07\u{e8}\u{e9}\x07\
	\x0a\x02\x02\u{e9}\u{ec}\x05\x1e\x10\x06\u{ea}\u{ec}\x05\x3e\x20\x02\u{eb}\
	\u{e1}\x03\x02\x02\x02\u{eb}\u{e6}\x03\x02\x02\x02\u{eb}\u{e8}\x03\x02\x02\
	\x02\u{eb}\u{ea}\x03\x02\x02\x02\u{ec}\u{f6}\x03\x02\x02\x02\u{ed}\u{ee}\
	\x0c\x05\x02\x02\u{ee}\u{ef}\x07\x0d\x02\x02\u{ef}\u{f5}\x05\x1e\x10\x06\
	\u{f0}\u{f1}\x0c\x04\x02\x02\u{f1}\u{f5}\x05\x1e\x10\x05\u{f2}\u{f3}\x0c\
	\x09\x02\x02\u{f3}\u{f5}\x05\x3c\x1f\x02\u{f4}\u{ed}\x03\x02\x02\x02\u{f4}\
	\u{f0}\x03\x02\x02\x02\u{f4}\u{f2}\x03\x02\x02\x02\u{f5}\u{f8}\x03\x02\x02\
	\x02\u{f6}\u{f4}\x03\x02\x02\x02\u{f6}\u{f7}\x03\x02\x02\x02\u{f7}\x1f\x03\
	\x02\x02\x02\u{f8}\u{f6}\x03\x02\x02\x02\u{f9}\u{fb}\x05\x34\x1b\x02\u{fa}\
	\u{f9}\x03\x02\x02\x02\u{fb}\u{fe}\x03\x02\x02\x02\u{fc}\u{fa}\x03\x02\x02\
	\x02\u{fc}\u{fd}\x03\x02\x02\x02\u{fd}\u{ff}\x03\x02\x02\x02\u{fe}\u{fc}\
	\x03\x02\x02\x02\u{ff}\u{100}\x05\x36\x1c\x02\u{100}\u{101}\x07\x17\x02\
	\x02\u{101}\u{102}\x05\x46\x24\x02\u{102}\u{103}\x05\x1a\x0e\x02\u{103}\
	\x21\x03\x02\x02\x02\u{104}\u{105}\x07\x0c\x02\x02\u{105}\u{107}\x05\x46\
	\x24\x02\u{106}\u{108}\x07\x0f\x02\x02\u{107}\u{106}\x03\x02\x02\x02\u{107}\
	\u{108}\x03\x02\x02\x02\u{108}\x23\x03\x02\x02\x02\u{109}\u{10b}\x05\x34\
	\x1b\x02\u{10a}\u{109}\x03\x02\x02\x02\u{10b}\u{10e}\x03\x02\x02\x02\u{10c}\
	\u{10a}\x03\x02\x02\x02\u{10c}\u{10d}\x03\x02\x02\x02\u{10d}\u{10f}\x03\
	\x02\x02\x02\u{10e}\u{10c}\x03\x02\x02\x02\u{10f}\u{110}\x05\x36\x1c\x02\
	\u{110}\u{112}\x07\x18\x02\x02\u{111}\u{113}\x05\x46\x24\x02\u{112}\u{111}\
	\x03\x02\x02\x02\u{112}\u{113}\x03\x02\x02\x02\u{113}\u{114}\x03\x02\x02\
	\x02\u{114}\u{115}\x05\x26\x14\x02\u{115}\x25\x03\x02\x02\x02\u{116}\u{11b}\
	\x07\x20\x02\x02\u{117}\u{11a}\x05\x28\x15\x02\u{118}\u{11a}\x07\x06\x02\
	\x02\u{119}\u{117}\x03\x02\x02\x02\u{119}\u{118}\x03\x02\x02\x02\u{11a}\
	\u{11d}\x03\x02\x02\x02\u{11b}\u{119}\x03\x02\x02\x02\u{11b}\u{11c}\x03\
	\x02\x02\x02\u{11c}\u{11e}\x03\x02\x02\x02\u{11d}\u{11b}\x03\x02\x02\x02\
	\u{11e}\u{11f}\x07\x21\x02\x02\u{11f}\x27\x03\x02\x02\x02\u{120}\u{122}\
	\x05\x34\x1b\x02\u{121}\u{120}\x03\x02\x02\x02\u{122}\u{125}\x03\x02\x02\
	\x02\u{123}\u{121}\x03\x02\x02\x02\u{123}\u{124}\x03\x02\x02\x02\u{124}\
	\u{126}\x03\x02\x02\x02\u{125}\u{123}\x03\x02\x02\x02\u{126}\u{127}\x05\
	\x36\x1c\x02\u{127}\u{128}\x05\x46\x24\x02\u{128}\u{129}\x07\x05\x02\x02\
	\u{129}\u{12a}\x05\x3e\x20\x02\u{12a}\x29\x03\x02\x02\x02\u{12b}\u{12c}\
	\x08\x16\x01\x02\u{12c}\u{12d}\x05\x3e\x20\x02\u{12d}\u{133}\x03\x02\x02\
	\x02\u{12e}\u{12f}\x0c\x04\x02\x02\u{12f}\u{130}\x07\x0e\x02\x02\u{130}\
	\u{132}\x05\x2a\x16\x05\u{131}\u{12e}\x03\x02\x02\x02\u{132}\u{135}\x03\
	\x02\x02\x02\u{133}\u{131}\x03\x02\x02\x02\u{133}\u{134}\x03\x02\x02\x02\
	\u{134}\x2b\x03\x02\x02\x02\u{135}\u{133}\x03\x02\x02\x02\u{136}\u{138}\
	\x05\x34\x1b\x02\u{137}\u{136}\x03\x02\x02\x02\u{138}\u{13b}\x03\x02\x02\
	\x02\u{139}\u{137}\x03\x02\x02\x02\u{139}\u{13a}\x03\x02\x02\x02\u{13a}\
	\u{13c}\x03\x02\x02\x02\u{13b}\u{139}\x03\x02\x02\x02\u{13c}\u{13d}\x05\
	\x36\x1c\x02\u{13d}\u{13e}\x07\x19\x02\x02\u{13e}\u{13f}\x05\x46\x24\x02\
	\u{13f}\u{140}\x05\x2e\x18\x02\u{140}\x2d\x03\x02\x02\x02\u{141}\u{146}\
	\x07\x20\x02\x02\u{142}\u{145}\x05\x30\x19\x02\u{143}\u{145}\x07\x06\x02\
	\x02\u{144}\u{142}\x03\x02\x02\x02\u{144}\u{143}\x03\x02\x02\x02\u{145}\
	\u{148}\x03\x02\x02\x02\u{146}\u{144}\x03\x02\x02\x02\u{146}\u{147}\x03\
	\x02\x02\x02\u{147}\u{149}\x03\x02\x02\x02\u{148}\u{146}\x03\x02\x02\x02\
	\u{149}\u{14a}\x07\x21\x02\x02\u{14a}\x2f\x03\x02\x02\x02\u{14b}\u{14d}\
	\x05\x34\x1b\x02\u{14c}\u{14b}\x03\x02\x02\x02\u{14d}\u{150}\x03\x02\x02\
	\x02\u{14e}\u{14c}\x03\x02\x02\x02\u{14e}\u{14f}\x03\x02\x02\x02\u{14f}\
	\u{151}\x03\x02\x02\x02\u{150}\u{14e}\x03\x02\x02\x02\u{151}\u{152}\x05\
	\x46\x24\x02\u{152}\u{153}\x07\x05\x02\x02\u{153}\u{154}\x05\x42\x22\x02\
	\u{154}\x31\x03\x02\x02\x02\u{155}\u{157}\x05\x34\x1b\x02\u{156}\u{155}\
	\x03\x02\x02\x02\u{157}\u{15a}\x03\x02\x02\x02\u{158}\u{156}\x03\x02\x02\
	\x02\u{158}\u{159}\x03\x02\x02\x02\u{159}\u{15b}\x03\x02\x02\x02\u{15a}\
	\u{158}\x03\x02\x02\x02\u{15b}\u{15c}\x05\x36\x1c\x02\u{15c}\u{15d}\x07\
	\x1a\x02\x02\u{15d}\u{15e}\x05\x46\x24\x02\u{15e}\u{15f}\x05\x2e\x18\x02\
	\u{15f}\x33\x03\x02\x02\x02\u{160}\u{164}\x09\x02\x02\x02\u{161}\u{165}\
	\x07\x19\x02\x02\u{162}\u{165}\x07\x1a\x02\x02\u{163}\u{165}\x05\x42\x22\
	\x02\u{164}\u{161}\x03\x02\x02\x02\u{164}\u{162}\x03\x02\x02\x02\u{164}\
	\u{163}\x03\x02\x02\x02\u{165}\u{167}\x03\x02\x02\x02\u{166}\u{168}\x05\
	\x3a\x1e\x02\u{167}\u{166}\x03\x02\x02\x02\u{167}\u{168}\x03\x02\x02\x02\
	\u{168}\x35\x03\x02\x02\x02\u{169}\u{16b}\x05\x46\x24\x02\u{16a}\u{169}\
	\x03\x02\x02\x02\u{16b}\u{16e}\x03\x02\x02\x02\u{16c}\u{16a}\x03\x02\x02\
	\x02\u{16c}\u{16d}\x03\x02\x02\x02\u{16d}\x37\x03\x02\x02\x02\u{16e}\u{16c}\
	\x03\x02\x02\x02\u{16f}\u{170}\x07\x0b\x02\x02\u{170}\u{172}\x05\x42\x22\
	\x02\u{171}\u{173}\x05\x3a\x1e\x02\u{172}\u{171}\x03\x02\x02\x02\u{172}\
	\u{173}\x03\x02\x02\x02\u{173}\x39\x03\x02\x02\x02\u{174}\u{180}\x07\x1c\
	\x02\x02\u{175}\u{17a}\x05\x16\x0c\x02\u{176}\u{177}\x07\x04\x02\x02\u{177}\
	\u{179}\x05\x16\x0c\x02\u{178}\u{176}\x03\x02\x02\x02\u{179}\u{17c}\x03\
	\x02\x02\x02\u{17a}\u{178}\x03\x02\x02\x02\u{17a}\u{17b}\x03\x02\x02\x02\
	\u{17b}\u{17e}\x03\x02\x02\x02\u{17c}\u{17a}\x03\x02\x02\x02\u{17d}\u{17f}\
	\x07\x04\x02\x02\u{17e}\u{17d}\x03\x02\x02\x02\u{17e}\u{17f}\x03\x02\x02\
	\x02\u{17f}\u{181}\x03\x02\x02\x02\u{180}\u{175}\x03\x02\x02\x02\u{180}\
	\u{181}\x03\x02\x02\x02\u{181}\u{182}\x03\x02\x02\x02\u{182}\u{183}\x07\
	\x1d\x02\x02\u{183}\x3b\x03\x02\x02\x02\u{184}\u{18c}\x07\x09\x02\x02\u{185}\
	\u{18c}\x07\x07\x02\x02\u{186}\u{187}\x07\x07\x02\x02\u{187}\u{18c}\x07\
	\x09\x02\x02\u{188}\u{18c}\x07\x08\x02\x02\u{189}\u{18a}\x07\x08\x02\x02\
	\u{18a}\u{18c}\x07\x09\x02\x02\u{18b}\u{184}\x03\x02\x02\x02\u{18b}\u{185}\
	\x03\x02\x02\x02\u{18b}\u{186}\x03\x02\x02\x02\u{18b}\u{188}\x03\x02\x02\
	\x02\u{18b}\u{189}\x03\x02\x02\x02\u{18c}\x3d\x03\x02\x02\x02\u{18d}\u{18f}\
	\x07\x1c\x02\x02\u{18e}\u{190}\x07\x0e\x02\x02\u{18f}\u{18e}\x03\x02\x02\
	\x02\u{18f}\u{190}\x03\x02\x02\x02\u{190}\u{191}\x03\x02\x02\x02\u{191}\
	\u{192}\x05\x16\x0c\x02\u{192}\u{193}\x07\x1d\x02\x02\u{193}\u{19c}\x03\
	\x02\x02\x02\u{194}\u{19c}\x05\x38\x1d\x02\u{195}\u{19c}\x05\x44\x23\x02\
	\u{196}\u{19c}\x05\x46\x24\x02\u{197}\u{19c}\x05\x40\x21\x02\u{198}\u{19c}\
	\x07\x22\x02\x02\u{199}\u{19c}\x07\x23\x02\x02\u{19a}\u{19c}\x07\x24\x02\
	\x02\u{19b}\u{18d}\x03\x02\x02\x02\u{19b}\u{194}\x03\x02\x02\x02\u{19b}\
	\u{195}\x03\x02\x02\x02\u{19b}\u{196}\x03\x02\x02\x02\u{19b}\u{197}\x03\
	\x02\x02\x02\u{19b}\u{198}\x03\x02\x02\x02\u{19b}\u{199}\x03\x02\x02\x02\
	\u{19b}\u{19a}\x03\x02\x02\x02\u{19c}\x3f\x03\x02\x02\x02\u{19d}\u{19e}\
	\x09\x03\x02\x02\u{19e}\x41\x03\x02\x02\x02\u{19f}\u{1a4}\x05\x46\x24\x02\
	\u{1a0}\u{1a1}\x09\x04\x02\x02\u{1a1}\u{1a3}\x05\x46\x24\x02\u{1a2}\u{1a0}\
	\x03\x02\x02\x02\u{1a3}\u{1a6}\x03\x02\x02\x02\u{1a4}\u{1a2}\x03\x02\x02\
	\x02\u{1a4}\u{1a5}\x03\x02\x02\x02\u{1a5}\x43\x03\x02\x02\x02\u{1a6}\u{1a4}\
	\x03\x02\x02\x02\u{1a7}\u{1a8}\x09\x05\x02\x02\u{1a8}\x45\x03\x02\x02\x02\
	\u{1a9}\u{1aa}\x09\x06\x02\x02\u{1aa}\x47\x03\x02\x02\x02\x33\x51\x53\x5b\
	\x5e\x64\x6d\x75\x77\u{82}\u{87}\u{8c}\u{94}\u{9a}\u{9f}\u{ae}\u{ba}\u{bc}\
	\u{c2}\u{ca}\u{d2}\u{db}\u{df}\u{eb}\u{f4}\u{f6}\u{fc}\u{107}\u{10c}\u{112}\
	\u{119}\u{11b}\u{123}\u{133}\u{139}\u{144}\u{146}\u{14e}\u{158}\u{164}\u{167}\
	\u{16c}\u{172}\u{17a}\u{17e}\u{180}\u{18b}\u{18f}\u{19b}\u{1a4}";
