mod downgrade;
mod parsing;

use super::{atomic::*, *};


pub struct GSTNode<Type, Meta = ()> {
    pub data: Type,
    pub meta: Option<Meta>,
}

pub trait GSTListener {
    type MetaData;
    /// Function called before visiting child nodes
    /// If [enter_node] returns `false` the child nodes will be skipped
    fn enter_node(&self, node: &Node) -> Result<bool> {
        println!("Enter: {:#?}", node);
        Ok(true)
    }

    /// Function called after all child nodes have been visited
    fn leave_node(&self, node: &Node) -> Result<()> {
        let kind = SyntaxKind::node_kind(node);
        println!("Leave: {:#?}", kind);
        Ok(())
    }
}


pub trait HiddenFilter {
    /// ture = hide
    /// false = dont hide
    fn hide_item_filter(&self, node: &Node) -> bool {
        use SyntaxKind::*;
        match SyntaxKind::node_kind(node) {
            OP_LBRACE if self.hide_unnamed_operators() => {true}
            OP_COMMA if self.hide_unnamed_operators() => {true}
            OP_RBRACE if self.hide_unnamed_operators() => {true}
            OP_EQ if self.hide_unnamed_operators() => {true}
            LineComment if self.hide_ignores() || self.hide_comment() => {true}
            _ => true
        }
    }
    fn hide_ignores(&self) -> bool {
        true
    }
    fn hide_comment(&self) -> bool {
        true
    }
    fn hide_unnamed_operators(&self) -> bool {
        true
    }
}

pub trait GSTVisitor: HiddenFilter {
    type MetaData;

    fn visit_program(&mut self, node: &Node) -> Option<Self::MetaData>;
    fn visit_grammar_statement(&mut self, node: &Node) -> Option<Self::MetaData>;
    fn visit_eos(&mut self, node: &Node) -> Option<Self::MetaData>;
}

pub struct Program<M> {
    pub(crate) children: Vec<AuxNode1<M>>,
}

pub enum AuxNode1<M> {
    GrammarStatement(Box<GrammarStatement<M>>),
    FragmentStatement(Box<FragmentStatement<M>>),
    AssignStatement(Box<AssignStatement<M>>),
}

pub struct GrammarStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
    pub(crate) children: Vec<AuxNode2>,
}

pub enum AuxNode2 {
    GrammarStatement(String),
    OPCOMMA,
    OpLeft,
    OpRight,
}

pub struct FragmentStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
    pub(crate) children: Vec<AuxNode2>,
}

pub struct AssignStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
    pub(crate) children: Vec<AuxNode2>,
}
