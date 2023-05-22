use super::*;
use convert_case::{Case, Casing};
use yggdrasil_error::Validation;
use yggdrasil_ir::rule::{GrammarBody, YggdrasilIdentifier, YggdrasilVariant};
use yggdrasil_parser::bootstrap::IdentifierNode;

/// Automatically insert or remove tags
#[derive(Default)]
pub struct RemarkTags {}

impl CodeOptimizer for RemarkTags {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let mut info = info.clone();
        for rule in info.rules.values_mut() {
            let rule_name = rule.name.text.as_str();
            match &mut rule.body {
                GrammarBody::Class { term } => self.remark(term, rule.captures.auto),
                GrammarBody::Union { branches } => {
                    self.remark_union_root(branches, rule_name);
                    for variant in branches.iter_mut() {
                        self.remark(&mut variant.branch, rule.captures.auto)
                    }
                }
                GrammarBody::Climb { .. } => {}
            }
        }

        Validation::Success { value: info, diagnostics: vec![] }
    }
}

impl RemarkTags {
    fn remark(&self, expr: &mut YggdrasilExpression, scope: bool) {
        let mark = if expr.remark { !scope } else { scope };
        expr.remark = false;
        match &mut expr.body {
            ExpressionBody::Call(_) => {
                // if v.name.eq("box") {
                //     let head = v.arguments.first_mut().unwrap();
                //     expr.
                // }
            }
            // ^(a b c)
            ExpressionBody::Concat(v) => {
                for item in &mut v.sequence {
                    self.remark(item, mark)
                }
            }
            // ^(a|b|c)
            ExpressionBody::Choice(v) => {
                for item in &mut v.branches {
                    self.remark(item, mark)
                }
            }
            // ^(a+)
            ExpressionBody::Unary(v) => self.remark(&mut v.base, mark),
            ExpressionBody::Rule(v) if mark => match &mut expr.tag {
                Some(_) => {}
                None => expr.tag = Some(v.name.clone()),
            },
            ExpressionBody::Text(_) => {}
            _ => {}
        }
    }
    fn remark_union_root(&self, expr: &mut [YggdrasilVariant], rule: &str) {
        for (index, variant) in expr.iter_mut().enumerate() {
            match variant.tag {
                Some(_) => {}
                None => match &variant.branch.body {
                    ExpressionBody::Rule(r) => variant
                        .remark(YggdrasilIdentifier { text: r.name.text.to_case(Case::Pascal), range: r.name.range.clone() }),
                    _ => variant.remark(YggdrasilIdentifier {
                        text: format!("{rule}{index}").to_case(Case::Pascal),
                        range: Default::default(),
                    }),
                },
            }
        }
    }
}
