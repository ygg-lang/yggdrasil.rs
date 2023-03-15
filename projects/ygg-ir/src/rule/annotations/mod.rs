use super::*;

mod modifiers;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilAnnotations {
    pub macros: Vec<YggdrasilMacroCall>,
    pub modifiers: YggdrasilModifiers,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilModifiers {
    pub identifiers: Vec<YggdrasilIdentifier>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilMacroCall {
    pub name: YggdrasilNamepath,
    pub arguments: Vec<YggdrasilMacroArgument>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YggdrasilMacroArgument {
    pub key: Option<YggdrasilIdentifier>,
    pub value: YggdrasilExpression,
}

impl From<YggdrasilMacroCall> for YggdrasilExpression {
    fn from(value: YggdrasilMacroCall) -> Self {
        ExpressionKind::Call(value).into()
    }
}

impl YggdrasilAnnotations {
    pub fn get_atomic(&self) -> GrammarAtomic {
        self.modifiers.get_atomic().unwrap_or(GrammarAtomic::Combined)
    }
    pub fn get_ignored(&self) -> bool {
        self.modifiers.get_ignored().unwrap_or(false)
    }
    pub fn get_entry(&self) -> bool {
        self.modifiers.get_entry().unwrap_or(false)
    }

    pub fn get_keep(&self) -> bool {
        self.modifiers.get_keep().unwrap_or(false)
    }
}
