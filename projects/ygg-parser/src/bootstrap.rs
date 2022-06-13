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
    pub rule_body: RuleBodyNode,
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
    pub string: String,
    pub position: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub struct RuleBodyNode {
    pub expr: ExprNode,
    pub position: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub enum ExprNode {
    Choice { lhs: Box<ExprNode>, rhs: Box<ExprNode> },
    SoftConcat { lhs: Box<ExprNode>, rhs: Box<ExprNode> },
    Concat { lhs: Box<ExprNode>, rhs: Box<ExprNode> },
    Repeat { expr: Box<ExprNode> },
    Optional { expr: Box<ExprNode> },
    Group { expr: Box<ExprNode> },
    Identifier { identifier: IdentifierNode },
}

#[derive(Debug, Clone)]
pub struct ExprChoiceNode {
    items: Vec<ExprSoftConcatNode>,
}

#[derive(Debug, Clone)]
pub struct ExprSoftConcatNode {
    items: Vec<ExprSoftConcatNode>,
}

#[derive(Debug, Clone)]
pub struct ExprConcatNode {
    items: Vec<ExprRepeatNode>,
}

#[derive(Debug, Clone)]
pub struct ExprRepeatNode {
    expr: ExprRest,
    suffix: Vec<ExprSuffix>,
}

#[derive(Debug, Clone)]
pub struct ExprSuffix {
    expr: char,
}

#[derive(Debug, Clone)]
pub enum ExprRest {
    Negative { expr: Box<ExprNode> },
    Group { expr: Box<ExprNode> },
    Identifier { identifier: IdentifierNode },
}

mod private_area {
    use super::*;

    struct IgnoredNode {
        pub position: std::ops::Range<usize>,
    }

    impl ProgramNode {
        pub fn consume(i: YState) -> YResult<ProgramNode> {
            let (o, statement) = i.match_repeats(Self::consume_aux)?;
            o.finish(ProgramNode { statement, position: o.away_from(i) })
        }
        /// `statement ignore`
        fn consume_aux(s0: YState) -> YResult<StatementNode> {
            let s1 = s0.clone();
            let s2 = s1.skip(IgnoredNode::consume);
            let (s3, statement) = StatementNode::consume(s2)?;
            let s4 = s3.skip(IgnoredNode::consume);
            s4.finish(statement)
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
        /// m1 m2 class {}
        pub fn consume(s0: YState) -> YResult<ClassStatementNode> {
            let s1 = s0.clone();
            let (s2, modifiers) = ModifiersNode::consume(s1)?;
            let s3 = s2.skip(IgnoredNode::consume);
            let (s4, _) = KwClass::consume(s3)?;
            let s5 = s4.skip(IgnoredNode::consume);
            let (s6, identifier) = IdentifierNode::consume(s5)?;
            let s9 = s6.skip(IgnoredNode::consume);
            let (s10, rule_body) = RuleBodyNode::consume(s9)?;
            s10.finish(ClassStatementNode { modifiers, identifier, rule_body, position: s10.away_from(s0) })
        }
    }

    impl KwClass {
        fn consume(s0: YState) -> YResult<Self> {
            let (o, _) = s0.match_string("class", false)?;
            o.finish(KwClass { string: "class".to_string(), position: o.away_from(s0) })
        }
    }

    impl OutTypeNode {
        fn consume(s0: YState) -> YResult<OutTypeNode> {
            let s1 = s0.clone();
            let (s2, _) = s1.match_string(":", false)?;
            let (s3, typing) = NamepathNode::consume(s2)?;
            s3.finish(OutTypeNode { typing, position: s1.away_from(s0) })
        }
    }

    impl RuleBodyNode {
        fn consume(s0: YState) -> YResult<RuleBodyNode> {
            let s1 = s0.clone();
            let (s2, _) = s1.match_string("{", false)?;
            let s3 = s2.skip(IgnoredNode::consume);
            let (s4, expr) = ExprNode::consume(s3)?;
            let s5 = s4.skip(IgnoredNode::consume);
            let (s6, _) = s5.match_string("}", false)?;
            s6.finish(RuleBodyNode { expr, position: s6.away_from(s0) })
        }
    }

    impl ExprNode {}

    // item:ExprSoftConcatNode ignore ('|' ignore item:ExprSoftConcatNode ignore)*
    impl ExprChoiceNode {
        // fold from left
        fn ascent(mut self) -> ExprNode {
            let mut lhs = self.items.remove(0).ascent();
            for rhs in self.items {
                lhs = ExprNode::Choice { lhs: Box::new(lhs.ascent()), rhs: Box::new(rhs.ascent()) };
            }
            lhs
        }
        fn consume(i: YState) -> YResult<ExprChoiceNode> {
            let mut items = vec![];
            let o = i.clone();
            let (o, item_1) = ExprSoftConcatNode::consume(o)?;
            items.push(item_1);
            let (o, item_2) = o.match_repeats(Self::consume_aux1)?;
            items.extend(item_2);
            o.finish(ExprChoiceNode { items })
        }
        fn consume_aux1(i: YState) -> YResult<ExprSoftConcatNode> {
            let o = i.clone();
            let (o, _) = o.match_string("|", false)?;
            let o = o.skip(IgnoredNode::consume);
            let (o, item) = ExprSoftConcatNode::consume(o)?;
            let o = o.skip(IgnoredNode::consume);
            o.finish(item)
        }
    }
    // item:ExprConcatNode ignore ('~' ignore item:ExprConcatNode ignore)*
    impl ExprSoftConcatNode {
        pub fn ascent(mut self) -> ExprNode {
            let mut lhs = self.items.remove(0).ascent();
            for rhs in self.items {
                lhs = ExprNode::SoftConcat { lhs: Box::new(lhs.ascent()), rhs: Box::new(rhs.ascent()) };
            }
            lhs
        }
        fn consume(i: YState) -> YResult<Self> {}

        // '~' ignore item:ExprConcatNode ignore
        fn consume_aux1(i: YState) -> YResult<ExprConcatNode> {
            let o = i.clone();
            let (o, _) = o.match_string("~", false)?;
            let o = o.skip(IgnoredNode::consume);
            let (o, item) = ExprConcatNode::consume(o)?;
            let o = o.skip(IgnoredNode::consume);
            o.finish(item)
        }
    }

    // item:ExprRepeatNode ignore (item:ExprRepeatNode ignore)*
    impl ExprConcatNode {
        pub fn ascent(mut self) -> ExprNode {
            let mut lhs = self.items.remove(0).ascent();
            for rhs in self.items {
                lhs = ExprNode::Concat { lhs: Box::new(lhs.ascent()), rhs: Box::new(rhs.ascent()) };
            }
            lhs
        }

        // item:ExprRepeatNode ignore (item:ExprRepeatNode ignore)*
        fn consume(i: YState) -> YResult<Self> {
            let mut items = vec![];
            let o = i.clone();
            let (o, item_1) = ExprRepeatNode::consume(o)?;
            items.push(item_1);
            let (o, item_2) = o.match_repeats(Self::consume_aux1)?;
            items.extend(item_2);
            o.finish(ExprConcatNode { items })
        }

        /// `item:ExprRepeatNode ignore`
        fn consume_aux1(i: YState) -> YResult<ExprConcatNode> {
            let o = i.clone();
            let (o, item) = ExprRepeatNode::consume(o)?;
            let o = o.skip(IgnoredNode::consume);
            o.finish(item)
        }
    }

    impl NamepathNode {
        fn consume(s0: YState) -> YResult<NamepathNode> {
            let (s1, items) = s0.match_repeats(Self::consume_aux1)?;
            s1.finish(NamepathNode { items, position: s1.away_from(s0) })
        }
        fn consume_aux1(state: YState) -> YResult<IdentifierNode> {
            let (state, identifier) = IdentifierNode::consume(state)?;
            state.skip(IgnoredNode::consume);
            state.finish(identifier)
        }
    }

    /// `(identifiers ignore)*`
    impl ModifiersNode {
        pub fn consume(s0: YState) -> YResult<ModifiersNode> {
            let (s1, items) = s0.match_repeats(Self::consume_aux1)?;
            s1.finish(ModifiersNode { items, position: s1.away_from(s0) })
        }
        fn consume_aux1(s0: YState) -> YResult<IdentifierNode> {
            let s1 = s0.clone();
            let (s2, _) = s1.match_negative(Self::consume_aux2, "ModifiersNode::consume_aux1")?;
            let (s3, identifier) = IdentifierNode::consume(s2)?;
            let s4 = s3.skip(IgnoredNode::consume);
            s4.finish(identifier)
        }
        fn consume_aux2(s0: YState) -> YResult<()> {
            let s1 = s0.clone();
            let (s1, _) = s1.begin_choice().maybe(KwClass::consume).end_choice()?;
            s1.finish(())
        }
    }
    /// xid_start
    impl IdentifierNode {
        fn consume(s0: YState) -> YResult<IdentifierNode> {
            let (s1, name) = s0.match_string_if(|c| c.is_alphabetic(), "<alphabetic>")?;
            s1.finish(IdentifierNode { string: name, position: s1.away_from(s0) })
        }
    }

    impl IgnoredNode {
        fn consume(state: YState) -> YResult<()> {
            let (state, _) = state //
                .begin_choice()
                .maybe(Self::consume_whitespace)
                .maybe(Self::consume_comment)
                .end_choice()?;
            state.finish(())
        }
        fn consume_whitespace(state: YState) -> YResult<()> {
            let (state, _) = state.match_string_if(|c| c.is_whitespace() || c == '\r' || c == '\n', "<Whitespace>")?;
            state.finish(())
        }
        fn consume_comment(state: YState) -> YResult<()> {
            let (state, _) = state.match_comment_line("//")?;
            state.finish(())
        }
    }
}
