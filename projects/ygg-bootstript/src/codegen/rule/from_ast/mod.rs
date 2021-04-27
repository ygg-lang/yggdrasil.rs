use self::diagnostic::{duplicate_declaration_error, top_area_error};
use super::*;
use crate::{
    ast::{AssignStatement, ChoiceExpression, ChoiceTag, Data, Expression, Program, Statement},
    ygg::{ast::ConcatExpression, YGGError},
    HintItems, Result,
};
use convert_case::{Case, Casing};
use lsp_types::{CodeDescription, Url};
use std::{collections::HashSet, ops::AddAssign};


mod diagnostic;

impl Program {
    pub fn build_grammar(self, url: Url) -> Result<(GrammarState, HintItems)> {
        let mut is_top_area = true;
        let mut is_grammar = None;
        let mut grammar_pos = None;
        let mut name = None;
        let mut map = Map::<String, YGGRule>::default();
        let mut ignores = vec![];
        let mut ignores_pos = None;
        let mut diag = vec![];
        for stmt in self.statement {
            match stmt {
                Statement::GrammarStatement(s) => {
                    if !is_top_area {
                        diag.push(top_area_error(
                            "Grammar",
                            "Grammar statement must be declared at the top",
                            convert_range(s.range),
                        ))
                    }
                    match is_grammar {
                        Some(true) => diag.push(duplicate_declaration_error(
                            "Grammar",
                            "Already declaration as `grammar!`",
                            convert_range(s.range),
                            &url,
                            grammar_pos,
                        )),
                        Some(false) => diag.push(duplicate_declaration_error(
                            "Grammar",
                            "Already declaration as `fragment!`",
                            convert_range(s.range),
                            &url,
                            grammar_pos,
                        )),
                        None => {
                            is_grammar = Some(true);
                            grammar_pos = Some(convert_range(s.range));
                            name = Some(s.id.data)
                        }
                    }
                }
                Statement::FragmentStatement(s) => {
                    if !is_top_area {
                        diag.push(top_area_error(
                            "Fragment",
                            "Fragment statement must be declared at the top",
                            convert_range(s.range),
                        ))
                    }
                    match is_grammar {
                        Some(true) => diag.push(duplicate_declaration_error(
                            "Fragment",
                            "Already declaration as `grammar!`",
                            convert_range(s.range),
                            &url,
                            grammar_pos,
                        )),
                        Some(false) => diag.push(duplicate_declaration_error(
                            "Fragment",
                            "Already declaration as `fragment!`",
                            convert_range(s.range),
                            &url,
                            grammar_pos,
                        )),
                        None => {
                            is_grammar = Some(false);
                            grammar_pos = Some(convert_range(s.range));
                            name = Some(s.id.data)
                        }
                    }
                }
                Statement::IgnoreStatement(s) => {
                    if !is_top_area {
                        diag.push(top_area_error(
                            "Ignore",
                            "Ignore statement must be declared at the top",
                            convert_range(s.range),
                        ))
                    }
                    if !ignores.is_empty() {
                        diag.push(duplicate_declaration_error(
                            "Ignore",
                            "Already declaration ignore statement",
                            convert_range(s.range),
                            &url,
                            grammar_pos,
                        ))
                    }
                    else {
                        ignores = s.rules;
                        ignores_pos = Some(s.range)
                    }
                }
                Statement::AssignStatement(s) => {
                    is_top_area = false;
                    let rule = YGGRule::from(*s);
                    match map.get(&rule.name) {
                        Some(old) => diag.push(duplicate_declaration_error(
                            "Rule",
                            format!("Already declaration as Rule `{}`", old.name),
                            rule.range,
                            &url,
                            Some(old.range),
                        )),
                        None => {
                            map.insert(rule.name.to_owned(), rule);
                        }
                    }
                }
                Statement::EmptyStatement(_) => continue,
            }
        }
        let state = GrammarState { name: name.ok_or(YGGError::info_missing("name not found"))?, map, ignores, url };

        let hint = HintItems { diagnostic: diag, code_lens: vec![] };

        Ok((state, hint))
    }
}

impl From<AssignStatement> for YGGRule {
    fn from(s: AssignStatement) -> Self {
        let mut name = s.id.data;
        let mut structure = None;
        let force_inline = name.starts_with("_");
        match force_inline {
            // trim first _
            true => name = String::from(&name[1..=name.len()]),
            false => structure = Some(name.to_case(Case::UpperCamel)),
        }
        let mut eliminate_unmarked = false;
        let mut eliminate_unnamed = false;
        match s.eq.as_str() {
            "_=" => eliminate_unnamed = true,
            "^=" => eliminate_unmarked = true,
            _ => (),
        }
        let expression = RefinedExpression::from(s.rhs);
        Self {
            name,
            structure_name: structure,
            force_inline,
            eliminate_unmarked,
            eliminate_unnamed,
            expression,
            range: convert_range(s.id.range),
        }
    }
}

impl From<Expression> for RefinedExpression {
    fn from(raw: Expression) -> Self {
        match raw {
            Expression::Data(e) => Self::Data(box RefinedData::from(*e)),
            Expression::UnarySuffix(_) => {
                unimplemented!()
            }
            Expression::UnaryPrefix(_) => {
                unimplemented!()
            }
            Expression::ConcatExpression(e) => Self::Concat(box RefinedConcat::from(*e)),
            Expression::ChoiceExpression(e) => Self::Choice(box RefinedChoice::from(*e)),
            Expression::FieldExpression(_) => {
                unimplemented!()
            }
        }
    }
}

impl From<ConcatExpression> for RefinedConcat {
    fn from(e: ConcatExpression) -> Self {
        let lhs = RefinedExpression::from(e.lhs);
        let rhs = RefinedExpression::from(e.rhs);
        let mut base = Self::from(lhs);
        base += rhs;
        return base;
    }
}

impl From<RefinedExpression> for RefinedConcat {
    fn from(e: RefinedExpression) -> Self {
        match e {
            RefinedExpression::Concat(c) => Self { inner: c.inner },
            _ => Self { inner: vec![e] },
        }
    }
}

impl AddAssign<RefinedExpression> for RefinedConcat {
    fn add_assign(&mut self, rhs: RefinedExpression) {
        match rhs {
            RefinedExpression::Concat(c) => self.inner.extend(c.inner),
            _ => self.inner.push(rhs),
        }
    }
}

impl From<ChoiceExpression> for RefinedChoice {
    fn from(_: ChoiceExpression) -> Self {
        todo!()
        // let lhs = RefinedExpression::from(e.lhs);
        // let rhs = RefinedExpression::from(e.rhs);
        // let mut base = Self::from(lhs);
        // base += rhs;
        // return base;
    }
}

impl From<RefinedExpression> for RefinedChoice {
    fn from(_: RefinedExpression) -> Self {
        todo!()
    }
}

impl From<ChoiceTag> for RefinedTag {
    fn from(e: ChoiceTag) -> Self {
        Self { expr: e.expr.into() }
    }
}

impl From<Data> for RefinedData {
    fn from(data: Data) -> Self {
        match data {
            Data::Identifier(atom) => Self::Identifier(*atom),
            Data::Integer(atom) => Self::Integer(atom.data),
            Data::String(atom) => Self::String(atom.data),
            Data::Regex => {
                unimplemented!()
            }
        }
    }
}
