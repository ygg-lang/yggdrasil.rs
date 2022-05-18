use std::fmt::{Debug, Write};

use pest_meta::ast::RuleType;
use pest_meta::optimizer::{OptimizedExpr, OptimizedRule};
use pest_meta::parse_and_optimize;

use yggdrasil_ir::{ExpressionNode, GrammarRule};

pub struct PestConverter {}

trait FromPest {
    fn build_ygg(&self, f: impl Write, soft: bool) -> std::fmt::Result {
        let _ = soft;
        let _ = f;
        unimplemented!()
    }
}

#[test]
pub fn test() {
    let cvt = PestConverter::default();
    cvt.parse_pest(include_str!("../../tests/pest.pest"));
}

impl Default for PestConverter {
    fn default() -> Self {
        Self {}
    }
}

impl PestConverter {
    fn parse_pest(&self, text: &str) {
        let (a, rules) = parse_and_optimize(text).unwrap();
        println!("{:?}", a);
        for (index, rule) in rules.iter().enumerate() {
            let out = self.visit_rule(rule, index);
            println!("{:?}", rule);
            println!("{:#?}", out);
        }
    }
}

impl PestConverter {
    fn visit_rule(&self, rule: &OptimizedRule, index: usize) -> GrammarRule {
        let entry = index == 0;
        let atomic = match rule.ty {
            RuleType::Atomic => { true }
            RuleType::CompoundAtomic => { true }
            _ => false
        };
        let name = rule.name.clone();
        rule.expr

        GrammarRule {
            name,
            r#type: "".to_string(),
            document: "".to_string(),
            derives: Default::default(),
            auto_inline: false,
            auto_boxed: false,
            atomic,
            entry,
            union: false,
            keep: false,
            body: ExpressionNode::empty(),
            range: Default::default(),
        }
    }
    fn visit_expr(&self, expr: &OptimizedExpr) -> ExpressionNode {
        match expr {
            pest_meta::ast::Expr::Str(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::Ident(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::Seq(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::Choice(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::Opt(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::Rep(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::RepOnce(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::RepMin(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::RepMinMax(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::Push(s) => {
                ExpressionNode::empty()
            }
            pest_meta::ast::Expr::Pop(s) => {
                ExpressionNode::empty()
            }
        }
    }
}

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
