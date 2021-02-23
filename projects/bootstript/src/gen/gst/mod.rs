mod downgrade;
mod parsing;

use super::{atomic::*, *};

pub struct GSTNode<Type, Meta = ()> {
    pub data: Type,
    pub meta: Meta,
}

pub trait GSTVisitor {
    type MetaData;
    /// Function called before visiting child nodes
    /// If [enter_node] returns `false` the child nodes will be skipped
    fn enter_node(&self, node: &Node) -> Result<bool> {
        println!("In: {:#?}", node);
        Ok(true)
    }

    /// Function called after all child nodes have been visited
    fn leave_node(&self, node: &Node) -> Result<()> {
        let kind = SyntaxKind::node_kind(node);
        println!("Out: {:#?}", kind);
        Ok(())
        // use SyntaxKind::*;

        // match SyntaxKind::node_kind(&cursor.node()) {
        //     sym_Program => {
        //         let data = self.visitor.visit_program(&cursor.node())?;
        //
        //     }
        //     _ => panic!("{:?}", kind)
        // }
    }
    fn visit_program(&mut self, node: &Node) -> Result<Program<Self::MetaData>>;
    fn visit_grammar_statement(&mut self, node: &Node) -> Result<GrammarStatement<Self::MetaData>>;
    fn visit_eos(&mut self, node: &Node) -> Result<Eos>;
}

pub struct Program<M> {
    pub(crate) inner: Vec<GSTNode<AuxNode1<M>, M>>,
}

pub enum AuxNode1<M> {
    GrammarStatement(Box<GrammarStatement<M>>),
    FragmentStatement(Box<FragmentStatement<M>>),
    AssignStatement(Box<AssignStatement<M>>),
}

pub struct GrammarStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
}

pub struct FragmentStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
}

pub struct AssignStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
}
