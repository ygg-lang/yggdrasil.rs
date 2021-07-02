use std::fmt::{Debug, Formatter, Write};
use pest_meta::ast::{Expr, Rule, RuleType};

pub trait FromPest {
    fn build_ygg(&self, f: &mut Formatter<'_>, soft: bool) -> std::fmt::Result {
        let _ = soft;
        let _ = f;
        unimplemented!()
    }
}

impl FromPest for Rule {
    fn build_ygg(&self, f: &mut Formatter<'_>, _: bool) -> std::fmt::Result {
        let mut soft_concat = false;
        let kind = match self.ty {
            RuleType::Normal => {
                soft_concat = true;
                ""
            }
            RuleType::Silent => {
                soft_concat = true;
                "_"
            }
            RuleType::Atomic => { "" }
            RuleType::CompoundAtomic => { "" }
            RuleType::NonAtomic => {
                soft_concat = true;
                ""
            }
        };
        write!(f, "{name} {kind}= ", name = self.name, kind = kind)?;
        FromPest::build_ygg(&self.expr, f, soft_concat);
        write!(f, ";")
    }
}

impl FromPest for Expr {
    fn build_ygg(&self, f: &mut Formatter<'_>, soft: bool) -> std::fmt::Result {
        match self {
            Expr::Str(s) => {
                f.write_str(s)
            }
            Expr::Insens(s) => {
                write!(f, "/{}/i", s)
            }
            Expr::Range(a, b) => {
                write!(f, "[{}-{}]", a, b)
            }
            Expr::Ident(v) => {
                f.write_str(v)
            }
            Expr::PeekSlice(a, b) => {}
            Expr::PosPred(a) => {}
            Expr::NegPred(a) => {}
            Expr::Seq(a, b) => {
                a.build_ygg(f, soft)?
            }
            Expr::Choice(a, b) => {}
            Expr::Opt(a) => {}
            Expr::Rep(a) => {}
            Expr::RepOnce(a) => {}
            Expr::RepExact(a, b) => {}
            Expr::RepMin(a, b) => {}
            Expr::RepMax(a, b) => {}
            Expr::RepMinMax(e, a, b) => {

            }
            Expr::Skip(a) => {

            }
            Expr::Push(push) => {
                write!(f, "@push(")?;
                push.build_ygg(f, soft)?;
                write!(f, ")")?
            }
        }
        Ok(())
    }
}