use std::fmt::{Debug, Formatter, Write};
use pest_meta::ast::{Expr, Rule, RuleType};
use pest_meta::{parser::{parse,self},};

pub trait FromPest {
    fn build_ygg(&self, f: impl Write, soft: bool) -> std::fmt::Result {
        let _ = soft;
        let _ = f;
        unimplemented!()
    }
}

pub fn convert_pest(input: &str) -> std::fmt::Result {
    parse(parser::Rule )
}


//
// impl FromPest for Rule {
//     fn build_ygg(&self, f: impl Write, _: bool) -> std::fmt::Result {
//         let mut soft_concat = false;
//         let kind = match self.ty {
//             RuleType::Normal => {
//                 soft_concat = true;
//                 ""
//             }
//             RuleType::Silent => {
//                 soft_concat = true;
//                 "_"
//             }
//             RuleType::Atomic => { "" }
//             RuleType::CompoundAtomic => { "" }
//             RuleType::NonAtomic => {
//                 soft_concat = true;
//                 ""
//             }
//         };
//         write!(f, "{name} {kind}= ", name = self.name, kind = kind)?;
//         FromPest::build_ygg(&self.expr, f, soft_concat);
//         write!(f, ";")
//     }
// }
//
// impl FromPest for Expr {
//     fn build_ygg(&self, f: impl Write, soft: bool) -> std::fmt::Result {
//         match self {
//             Expr::Str(s) => {
//                 f.write_str(s)
//             }
//             Expr::Insens(s) => {
//                 write!(f, "/{}/i", s)
//             }
//             Expr::Range(a, b) => {
//                 write!(f, "[{}-{}]", a, b)
//             }
//             Expr::Ident(v) => {
//                 f.write_str(v)
//             }
//             Expr::PeekSlice(a, b) => {write!(f, "unimplemented!")}
//             Expr::PosPred(a) => {write!(f, "unimplemented!")}
//             Expr::NegPred(a) => {write!(f, "unimplemented!")}
//             Expr::Seq(a, b) => {
//                 a.build_ygg(f, soft)?
//             }
//             Expr::Choice(a, b) => {write!(f, "unimplemented!")}
//             Expr::Opt(a) => {write!(f, "unimplemented!")}
//             Expr::Rep(a) => {write!(f, "unimplemented!")}
//             Expr::RepOnce(a) => {write!(f, "unimplemented!")}
//             Expr::RepExact(a, b) => {write!(f, "unimplemented!")}
//             Expr::RepMin(a, b) => {write!(f, "unimplemented!")}
//             Expr::RepMax(a, b) => {write!(f, "unimplemented!")}
//             Expr::RepMinMax(e, a, b) => {
//                 e.build_ygg(f,soft)?;
//                 write!(f, "{{{},{}}}", a,b )
//             }
//             Expr::Skip(a) => {
//                 write!(f, "unimplemented!")
//             }
//             Expr::Push(push) => {
//                 write!(f, "@push(")?;
//                 push.build_ygg(f, soft)?;
//                 write!(f, ")")?
//             }
//         }
//         Ok(())
//     }
// }