mod parsing;
mod downgrade;

use super::*;

pub struct GSTNode<Type, Meta = ()> {
    pub data: Type,
    pub meta: Meta
}

pub trait GSTVisitor<M> {
    fn visit_program(&mut self, node: &Node) -> Result<Program<M>>;
    fn visit_grammar_statement(&mut self, node: &Node) -> Result<GrammarStatement<M>>;
    fn visit_eos(&mut self, node: &Node) -> Result<Eos>;
}

pub struct Program<M> {
    pub(crate) inner: Vec<GSTNode<AuxNode1<M>, M>>
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

pub struct Identifier(pub String);

pub struct Eos(pub String);

