use yggdrasil_rt::{
    ast_mode::{YResult, YState},
    results::StopBecause,
};

pub fn parse_program(input: &str) -> Result<ProgramNode, StopBecause> {
    let mut state = YState::new(input);
    let (state, program) = ProgramNode::consume(state)?;
    state.match_eof()?;
    Ok(program)
}

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub statement: Vec<StatementNode>,
    pub position: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    Class(Box<ClassStatementNode>),
}
// class ClassStatement {
//     Modifiers ^KW_CLASS Identifier OutType? RuleBody
// }
#[derive(Debug, Clone)]
pub struct ClassStatementNode {
    pub modifiers: ModifiersNode,
    pub identifier: IdentifierNode,
    pub out_type: Option<OutTypeNode>,
    pub position: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub struct KwClass {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct ModifiersNode {
    pub items: Vec<IdentifierNode>,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct OutTypeNode {
    pub typing: NamepathNode,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct NamepathNode {
    pub items: Vec<IdentifierNode>,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub name: String,
    pub position: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub struct RuleBodyNode {
    pub items: Vec<char>,
    pub position: std::ops::Range<usize>,
}

mod private_area {
    use super::*;

    struct IgnoredNode {
        pub position: std::ops::Range<usize>,
    }

    impl ProgramNode {
        pub(super) fn consume(i: YState) -> YResult<ProgramNode> {
            let (o, statement) = i.match_repeats(StatementNode::consume)?;
            o.finish(ProgramNode { statement: statement, position: o.away_from(i) })
        }
    }

    impl StatementNode {
        fn consume(state: YState) -> YResult<Self> {
            let (state, statement) = state //
                .begin_choice()
                .maybe(Self::consume_class)
                .end_choice()?;
            state.finish(statement)
        }
        fn consume_class(state: YState) -> YResult<Self> {
            let (state, class) = ClassStatementNode::consume(state)?;
            state.finish(Self::Class(Box::new(class)))
        }
    }

    impl ClassStatementNode {
        fn consume(state: YState) -> YResult<ClassStatementNode> {
            let (state, modifiers) = ModifiersNode::consume(state)?;
            // let (state, _) = KwClass::consume(state)?;
            // let (state, identifier) = IdentifierNode::consume(state)?;
            // let (state, out_type) = state.parse_optional(OutTypeNode::consume)?;
            // let (state, rule_body) = RuleBodyNode::consume(state)?;
            // state.finish(ClassStatementNode { modifiers, identifier, out_type, position: state.range() })
            todo!()
        }
    }

    impl KwClass {
        fn consume(i: YState) -> YResult<KwClass> {
            let start = i.start_offset;
            let (o, _) = i.match_string("class", false)?;
            o.finish(KwClass { string: "class".to_string(), position: o.away_from(i) })
        }
    }

    /// `(identifiers ignore)*`
    impl ModifiersNode {
        fn consume(s0: YState) -> YResult<ModifiersNode> {
            let (s1, items) = s0.match_repeats(Self::consume_aux1)?;
            s1.finish(ModifiersNode { items, position: s1.away_from(s0) })
        }
        fn consume_aux1(state: YState) -> YResult<IdentifierNode> {
            let (state, identifier) = IdentifierNode::consume(state)?;
            state.skip(IgnoredNode::consume);
            state.finish(identifier)
        }
    }
    /// xid_start
    impl IdentifierNode {
        fn consume(state: YState) -> YResult<IdentifierNode> {
            let (state, name) = state.match_string(|s| s.match_char_if(|c| c.is_alphabetic(), "{alphabetic}"))?;
            state.finish(IdentifierNode { name, position: state.range() })
        }
    }

    impl IgnoredNode {
        fn consume(state: YState) -> YResult<()> {
            state //
                .begin_choice()
                .maybe(Self::consume_whitespace)
                .maybe(Self::consume_comment)
                .end_choice()?
                .finish(())
        }
        fn consume_whitespace(state: YState) -> YResult<()> {
            let (state, _) = state.match_repeats(|s| s.match_char_if(|c| c.is_whitespace(), "<Whitespace>"))?;
            state.finish(())
        }
        fn consume_comment(state: YState) -> YResult<()> {
            let (state, _) = state.match_comment_line("//")?;
            state.finish(())
        }
    }
}
