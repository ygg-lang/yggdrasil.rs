#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub statements: Vec<StatementNode>,
    pub position: std::ops::Range<usize>,
}

pub enum StatementNode {
    Class(Box<ClassStatementNode>),
}
// class ClassStatement {
//     Modifiers ^KW_CLASS Identifier OutType? RuleBody
// }
pub struct ClassStatementNode {
    pub modifiers: ModifiersNode,
    pub identifier: IdentifierNode,
    pub position: std::ops::Range<usize>,
}

pub struct ModifiersNode {}

pub struct OutTypeNode {}

pub struct IdentifierNode {
    pub name: String,
    pub position: std::ops::Range<usize>,
}
