use super::*;
use convert_case::{Case, Casing};
use yggdrasil_error::Validation;
use yggdrasil_ir::rule::{GrammarBody, YggdrasilIdentifier};

/// Automatically insert or remove tags
#[derive(Default)]
pub struct RemarkTags {}

impl CodeOptimizer for RemarkTags {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let mut info = info.clone();
        for rule in info.rules.values_mut() {
            let rule_name = rule.name.text.as_str();
            match &mut rule.body {
                GrammarBody::Empty => {}
                GrammarBody::Class { term } => self.remark(term, true),
                GrammarBody::Union { branches } => {
                    self.remark_union_root(rule_name, branches);
                    for branch in branches {
                        self.remark(branch, true)
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
                None => {
                    expr.tag = Some(v.name.clone());
                }
            },
            ExpressionBody::Text(_) => {}
            _ => {}
        }
    }
    fn remark_union_root(&self, rule: &str, expr: &mut [YggdrasilExpression]) {
        for (index, branch) in expr.iter_mut().enumerate() {
            match branch.tag {
                Some(_) => {}
                None => match &branch.body {
                    ExpressionBody::Rule(r) => {
                        branch.tag =
                            Some(YggdrasilIdentifier { text: r.name.text.to_case(Case::Pascal), range: r.name.range.clone() })
                    }
                    _ => {
                        branch.tag = Some(YggdrasilIdentifier {
                            text: format!("{rule}{index}").to_case(Case::Pascal),
                            range: Default::default(),
                        })
                    }
                },
            }
        }
    }
}
