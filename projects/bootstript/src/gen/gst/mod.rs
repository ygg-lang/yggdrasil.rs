mod downgrade;
mod parsing;

use super::{atomic::*, *};

pub type AnyNode<M> = GSTNode<Box<dyn GSTNodeAny>, M>;

pub type AnyVec<M> = Vec<AnyNode<M>>;

pub struct GSTNode<Type, Meta = ()> {
    pub data: Type,
    pub meta: Meta,
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

pub trait GSTNodeAny {

}



pub trait GSTVisitor {
    type MetaData;
    // may const
    fn hide_ignores(&self) -> bool {
        true
    }
    fn visit_program(&mut self, node: &Node) -> Result<Program<Self::MetaData>>;
    fn visit_grammar_statement(&mut self, node: &Node) -> Result<GrammarStatement<Self::MetaData>>;
    fn visit_eos(&mut self, node: &Node) -> Result<Eos>;
}

pub struct Program<M> {
    pub(crate) children: AnyVec<M>,
}

pub struct GrammarStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
    pub(crate) children: AnyVec<M>,
}

pub struct FragmentStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
    pub(crate) children: AnyVec<M>,
}

pub struct AssignStatement<M> {
    pub id: GSTNode<Identifier, M>,
    pub eos: GSTNode<Eos, M>,
    pub(crate) children: AnyVec<M>,
}
