use super::*;
use std::{
    fmt::{Debug, Formatter},
    ops::{AddAssign, Deref, DerefMut},
};

impl Debug for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.statement.iter()).finish()
    }
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GrammarStatement(v) => f
                .debug_struct("GrammarStatement") //
                .field("id", &v.id.data)
                .field("ext", &v.ext)
                .finish(),
            Self::FragmentStatement(v) => f
                .debug_struct("FragmentStatement") //
                .field("id", &v.id.data)
                .finish(),
            Self::AssignStatement(v) => f
                .debug_struct("AssignStatement") //
                .field("id", &v.id.data)
                .field("eq", &v.eq)
                .field("rhs", &v.rhs)
                .finish(),
            Self::IgnoreStatement(v) => f
                .debug_struct("IgnoreStatement") //
                .field("rules", &v.rules)
                .finish(),
            Statement::CommentDocument(_) => Ok(()),
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Data(v) => {
                f.write_str("Data::")?;
                Debug::fmt(v, f)
            }
            Expression::UnarySuffix(v) => f
                .debug_tuple("Unary::Suffix") //
                .field(&v.base)
                .field(&v.suffix)
                .finish(),
            Expression::UnaryPrefix(v) => f
                .debug_tuple("Unary::Prefix") //
                .field(&v.prefix)
                .field(&v.base)
                .finish(),
            Expression::FieldExpression(v) => f
                .debug_struct("FieldExpression") //
                .field("lhs", &v.lhs)
                .field("op", &v.op)
                .field("rhs", &v.rhs)
                .finish(),
            Expression::ConcatExpression(v) => f
                .debug_struct("ConcatExpression") //
                .field("lhs", &v.lhs)
                .field("op", &v.op)
                .field("rhs", &v.rhs)
                .finish(),
            Expression::ChoiceExpression(v) => f
                .debug_struct("ChoiceExpression") //
                .field("lhs", &v.lhs)
                .field("op", &v.op)
                .field("rhs", &v.rhs)
                .finish(),
            /* Expression::ConcatExpression(v) => {
             *     f.write_str("ConcatExpression ")?;
             *     let mut list = f.debug_list();
             *     list.entry(&v.lhs);
             *     for (o, e) in v.op.iter().zip(v.rhs.iter()) {
             *         list.entry(o);
             *         list.entry(e);
             *     }
             *     list.finish()
             * }
             * Expression::ChoiceExpression(v) => {
             *     f.write_str("ChoiceExpression ")?;
             *     let mut list = f.debug_list();
             *     list.entry(&v.lhs);
             *     for (o, e) in v.op.iter().zip(v.rhs.iter()) {
             *         list.entry(o);
             *         list.entry(e);
             *     }
             *     list.finish()
             * } */
        }
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Identifier(v) => f.debug_tuple("Identifier").field(&v.data).finish(),
            Data::Integer(v) => f.debug_tuple("Integer").field(&v.data).finish(),
            Data::String(v) => f.debug_tuple("String").field(&v.data).finish(),
            Data::Regex => f.debug_tuple("Regex").finish(),
            Data::Macro => f.debug_tuple("Macro").finish(),
        }
    }
}
