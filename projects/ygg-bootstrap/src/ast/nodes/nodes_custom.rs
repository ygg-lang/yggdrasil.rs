use super::*;
use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

impl Debug for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.statement.iter()).finish()
    }
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grammar(v) => f
                .debug_struct("GrammarStatement") //
                .field("id", &v.id)
                .field("ext", &v.ext)
                .finish(),
            Self::Fragment(v) => f
                .debug_struct("FragmentStatement") //
                .field("id", &v.id)
                .finish(),
            Self::Import(v) => f
                .debug_struct("ImportStatement") //
                .field("id", &v.path)
                .field("symbol_alias", &v.symbol_alias)
                .finish(),
            Self::Assign(v) => f
                .debug_struct("AssignStatement") //
                .field("id", &v.id.data)
                .field("eq", &v.eq)
                .field("rhs", &v.rhs)
                .finish(),
            Self::Ignore(v) => f
                .debug_struct("IgnoreStatement") //
                .field("rules", &v.rules)
                .finish(),
            Statement::CommentDocument(_) => Ok(()),
        }
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Symbol(v) => Debug::fmt(v, f),
            Data::Integer(v) => {
                f.write_str("Integer(")?;
                write!(f, "{}", v.data)?;
                f.write_str(")")
            }
            Data::String(v) => Debug::fmt(v, f),
            Data::Regex => f.debug_tuple("Regex").finish(),
            Data::Macro => f.debug_tuple("Macro").finish(),
        }
    }
}

impl Eq for SymbolPath {}
impl PartialEq<Self> for SymbolPath {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}
impl Hash for SymbolPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state)
    }
}
impl Debug for SymbolPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SymbolPath(")?;
        let s = self.symbol.iter().map(|s| s.data.to_owned()).collect::<Vec<_>>().join("::");
        f.write_str(&s)?;
        f.write_str(")")
    }
}

impl Debug for SymbolAlias {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SymbolAlias(")?;
        f.write_str(&self.from.data)?;
        if let Some(s) = &self.into {
            f.write_str(" as ")?;
            f.write_str(&s.data)?;
        };
        f.write_str(")")
    }
}

impl Eq for Symbol {}
impl PartialEq<Self> for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Symbol(")?;
        write!(f, "{}", self.data)?;
        f.write_str(")")
    }
}

impl Eq for StringLiteral {}
impl PartialEq<Self> for StringLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Hash for StringLiteral {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

impl Debug for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("StringLiteral(")?;
        write!(f, "{:?}", self.data)?;
        f.write_str(")")
    }
}
