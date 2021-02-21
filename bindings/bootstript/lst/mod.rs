use rowan::{GreenNodeBuilder, NodeOrToken};
use std::iter::Peekable;
use self::SyntaxKind::*;
use tree_sitter::Node;
use lsp_types::Diagnostic;
use crate::Result;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
enum SyntaxKind {
    sym_Id = 1,
    anon_sym_LBRACE = 2,
    anon_sym_COMMA = 3,
    anon_sym_RBRACE = 4,
    sym_Grammar = 5,
    sym_whitespace = 6,
    sym_Fragment = 7,
    sym_Ignore = 8,
    anon_sym_EQ = 9,
    anon_sym_CARET = 10,
    anon_sym_BANG = 11,
    anon_sym_TILDE = 12,
    anon_sym_PIPE = 13,
    anon_sym_LT_DASH = 14,
    sym_Unsigned = 15,
    sym__sign = 16,
    anon_sym_SQUOTE = 17,
    aux_sym_String_token1 = 18,
    anon_sym_DQUOTE = 19,
    sym_Regex = 20,
    sym_Eos = 21,
    sym_Program = 22,
    sym_GrammarStatement = 23,
    sym_FragmentStatement = 24,
    sym_AssignStatement = 25,
    sym__expression = 26,
    sym_UnaryExpression = 27,
    sym_BinaryExpression = 28,
    sym_String = 29,
    aux_sym_Program_repeat1 = 30,
    aux_sym__grammar_exts_repeat1 = 31,
}

pub trait ASTVisitor {
    fn visit_program(&mut self, node: &Node) -> Result<Program>;
    fn visit_grammar_statement(&mut self, node: &Node) -> Result<GrammarStatement>;
    fn visit_eos(&mut self, node: &Node) -> Result<Eos>;
}

pub struct ASTNode<Type, Meta = ()> {
    data: Type,
    meta: Meta
}

pub struct MyVisitor<M> {
    warns: Vec<Diagnostic>,
}

impl ASTVisitor for MyVisitor<M> {
    fn visit_program(&mut self, node: &Node) -> Result<Program<T, M>> {
        todo!()
    }

    fn visit_grammar_statement(&mut self, node: &Node) -> Result<GrammarStatement<T, M>> {
        todo!()
    }

    fn visit_eos(&mut self, node: &Node) -> Result<Eos> {
        todo!()
    }
}

pub struct Program<T, M> {
    pub(crate) inner: Vec<ASTNode<AuxNode1<T, M>, M>>
}

pub enum AuxNode1<T, M> {
    GrammarStatement(Box<GrammarStatement<T, M>>),
    FragmentStatement(Box<FragmentStatement<T, M>>),
    AssignStatement(Box<AssignStatement<T, M>>),
}



pub type ASTSequence<T, M> = Vec<ASTNode<T, M>>;


pub struct GrammarStatement<T, M> {
    pub id: ASTNode<Identifier, M>,
    pub eos: ASTNode<Eos, M>,
    pub(crate) inner: Vec<ASTNode<T, M>>,
}

pub struct Identifier {
    pub(crate) inner: String
}

pub struct Eos(pub bool);


fn main() {
    let ast = Parser {
        builder: GreenNodeBuilder::new(),
        iter: vec![
            // 1 + 2 * 3 + 4
            (NUMBER, "1".into()),
            (WHITESPACE, " ".into()),
            (ADD, "+".into()),
            (WHITESPACE, " ".into()),
            (NUMBER, "2".into()),
            (WHITESPACE, " ".into()),
            (MUL, "*".into()),
            (WHITESPACE, " ".into()),
            (NUMBER, "3".into()),
            (WHITESPACE, " ".into()),
            (ADD, "+".into()),
            (WHITESPACE, " ".into()),
            (NUMBER, "4".into()),
        ]
            .into_iter()
            .peekable(),
    }
        .parse();
    print(0, ast.into());
}

pub fn traverse(tree: &Tree) {
    let cursor = &mut tree.walk();
    println!("in!");
    println!("{:#?}", cursor.node());
    let mut recurse = true;
    loop {
        if recurse && cursor.goto_first_child() {
            recurse = true;
        } else {
            println!("out!");
            if cursor.goto_next_sibling() {
                println!("in!");
                println!("{:#?}", cursor.node());
                recurse = true;
            } else if cursor.goto_parent() {
                recurse = false;
            } else {
                break;
            }
        }
    }
}