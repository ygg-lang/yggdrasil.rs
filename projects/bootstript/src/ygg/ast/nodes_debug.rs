use super::*;
use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};
use std::ops::AddAssign;

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
            Self::EmptyStatement(_) => f.write_str("EmptyStatement"),
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::ErrorNode => f.write_str("ErrorNode"),
            Expression::Data(v) => {
                f.write_str("Data::")?;
                Debug::fmt(v, f)
            }
            Expression::Priority(v) => f
                .debug_tuple("Priority") //
                .field(&v)
                .finish(),
            Expression::UnarySuffix(v) => f
                .debug_tuple("UnarySuffix") //
                .field(&v)
                .finish(),
            Expression::UnaryPrefix(v) => f
                .debug_tuple("UnaryPrefix") //
                .field(&v)
                .finish(),
            Expression::ConcatExpression(v) => {
                f.write_str("ConcatExpression ")?;
                let mut list = f.debug_list();
                list.entry(&v.base);
                for (o, e) in v.op.iter().zip(v.expr.iter()) {
                    list.entry(o);
                    list.entry(e);
                }
                list.finish()
            }
            Expression::FieldExpression(v) => f
                .debug_struct("ConcatExpression") //
                .field("lhs", &v.lhs)
                .field("op", &v.op)
                .field("rhs", &v.rhs)
                .finish(),
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
        }
    }
}

impl From<Expression> for ConcatExpression {
    fn from(expr: Expression) -> Self {
        match expr {
            Expression::ConcatExpression(c) => {
                *c
            }
            _ => Self { base: expr, op: vec![], expr: vec![], range: Range {
                start_byte: 0,
                end_byte: 0,
                start_point: Default::default(),
                end_point: Default::default()
            } },
        }
    }
}

impl AddAssign<Expression> for ConcatExpression {
    fn add_assign(&mut self, rhs: Expression) {
        match rhs {
            Expression::ConcatExpression(c) => { self.add_assign(*c)}
            _ => {
                self.op.push(String::from("~"));
                self.expr.push(rhs);
            }
        }
    }
}

impl AddAssign<ConcatExpression> for ConcatExpression {
    fn add_assign(&mut self, rhs: ConcatExpression) {
        self.op.push(String::from("~"));
        self.expr.push(rhs.base);
        self.op.extend(rhs.op);
        self.expr.extend(rhs.expr);
    }
}