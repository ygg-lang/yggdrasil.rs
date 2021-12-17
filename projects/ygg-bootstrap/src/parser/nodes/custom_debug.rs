use super::*;
use std::fmt::{Debug, Display, Formatter};

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
            Statement::MacroCall(_) => Ok(()),
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Data(e) => Debug::fmt(e, f),
            Expression::Concat { is_soft, lhs, rhs } => {
                let w = &mut match is_soft {
                    true => f.debug_struct("SoftConcat"),
                    false => f.debug_struct("Concat"),
                };
                w.field("lhs", lhs);
                w.field("rhs", rhs);
                w.finish()
            }
            Expression::Choice { lhs, rhs } => {
                let w = &mut f.debug_struct("Choice");
                w.field("lhs", lhs);
                w.field("rhs", rhs);
                w.finish()
            }
            Expression::MarkNode { lhs, rhs } => {
                let w = &mut f.debug_struct("MarkNodeAs");
                w.field("base", rhs);
                w.field("as", lhs);
                w.finish()
            }
            Expression::MarkNodeShort(e) => {
                let w = &mut f.debug_struct("MarkNode");
                w.field("base", e);
                w.finish()
            }
            Expression::MarkType { lhs, rhs } => {
                let w = &mut f.debug_struct("MarkType");
                w.field("lhs", lhs);
                w.field("type", rhs);
                w.finish()
            }
            Expression::MarkBranch { base: lhs, kind, name } => {
                let w = &mut f.debug_struct("MarkBranch");
                w.field("lhs", lhs);
                match kind {
                    None => {
                        w.field("type", &format!("${}", name.data));
                    }
                    Some(c) => {
                        w.field("type", &format!("{}${}", c, name.data));
                    }
                }
                w.finish()
            }
            Expression::MustNot(e) => f.debug_tuple("MustNot").field(e).finish(),
            Expression::MustOne(e) => f.debug_tuple("MustOne").field(e).finish(),
            Expression::Maybe(e) => f.debug_tuple("Maybe").field(e).finish(),
            Expression::Many(e) => f.debug_tuple("Many").field(e).finish(),
            Expression::ManyNonNull(e) => f.debug_tuple("ManyNonNull").field(e).finish(),
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
            Data::Macro(v) => Debug::fmt(v, f),
        }
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

impl Display for SymbolPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.symbol.iter().map(|s| s.data.to_owned()).collect::<Vec<_>>().join("::");
        f.write_str(&s)
    }
}

impl Debug for SymbolAlias {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SymbolAlias(")?;
        f.write_str(&self.from.data)?;
        f.write_str(" as ")?;
        f.write_str(&self.into.data)?;
        f.write_str(")")
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Symbol(")?;
        write!(f, "{}", self.data)?;
        f.write_str(")")
    }
}

impl Debug for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("StringLiteral(")?;
        write!(f, "{:?}", self.data)?;
        f.write_str(")")
    }
}
