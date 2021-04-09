use std::convert::TryFrom;
use std::ops::AddAssign;
use std::str::FromStr;
use super::*;
use convert_case::{Case, Casing};
use lsp_types::{CodeDescription, NumberOrString, Position, Range, Url};

use crate::ast::{AssignStatement, ChoiceExpression, ChoiceTag, Data, Expression, Program, Statement};
use crate::ygg::ast::ConcatExpression;
use crate::ygg::YGGError;

impl TryFrom<Program> for GrammarManager {
    type Error = YGGError;

    fn try_from(value: Program) -> Result<Self, Self::Error> {
        let mut is_top_area = true;
        let mut is_grammar = Default::default();
        let mut name = Default::default();
        let mut map = Map::default();
        let mut ignores = Default::default();
        let mut diag = vec![];
        for s in value.statement {
            match s {
                Statement::GrammarStatement(g) => {
                    if !is_top_area {
                        diag.push(Diagnostic {
                            range: convert_range(g.range),
                            severity: Some(DiagnosticSeverity::Warning),
                            code: Some(NumberOrString::String(String::from("SSS"))),
                            code_description: Some(CodeDescription { href: Url::from_str("https://example.com").unwrap() }),
                            source: Some(String::from("SSS")),
                            message: String::from("Grammar statement must be declared at the top"),
                            related_information: None,
                            tags: None,
                            data: None,
                        })
                    }
                    is_grammar = Some(true);
                    name = Some(g.id.data)
                }
                Statement::FragmentStatement(_) => {
                    is_grammar = Some(false)
                }
                Statement::IgnoreStatement(s) => {
                    ignores = s.rules
                }
                Statement::AssignStatement(s) => {
                    is_top_area = false;
                    let rule = YGGRule::from(*s);
                    map.insert(rule.name.to_owned(), rule);
                }
                Statement::EmptyStatement(_) => continue,
            }
        }
        Ok(Self {
            name: name.ok_or(YGGError::info_missing("name not found"))?,
            map,
            ignores,
            diag,
        })
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

        Self { name, structure_name: structure, force_inline, eliminate_unmarked, eliminate_unnamed, expression }
    }
}

impl From<Expression> for RefinedExpression {
    fn from(raw: Expression) -> Self {
        match raw {
            Expression::Data(e) => {
                Self::Data(box RefinedData::from(*e))
            }
            Expression::UnarySuffix(_) => { unimplemented!() }
            Expression::UnaryPrefix(_) => { unimplemented!() }
            Expression::ConcatExpression(e) => {
                Self::Concat(box RefinedConcat::from(*e))
            }
            Expression::ChoiceExpression(e) => {
                Self::Choice(box RefinedChoice::from(*e))
            }
            Expression::FieldExpression(_) => { unimplemented!() }
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
            RefinedExpression::Concat(c) => {
                Self {
                    inner: c.inner
                }
            }
            _ => Self {
                inner: vec![e]
            }
        }
    }
}

impl AddAssign<RefinedExpression> for RefinedConcat {
    fn add_assign(&mut self, rhs: RefinedExpression) {
        match rhs {
            RefinedExpression::Concat(c) => {
                self.inner.extend(c.inner)
            }
            _ => {
                self.inner.push(rhs)
            }
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
        Self {
            expr: e.expr.into()
        }
    }
}

impl From<Data> for RefinedData {
    fn from(data: Data) -> Self {
        match data {
            Data::Identifier(atom) => {
                let mut id = atom.data.as_str();
                let mut inline = false;
                if atom.data.starts_with("_") {
                    id = &atom.data[1..=id.len()];
                    inline = true
                }
                RefinedData::Identifier {
                    id: String::from(id),
                    inline,
                }
            }
            Data::Integer(atom) => { RefinedData::Integer(atom.data)}
            Data::String(atom) => {RefinedData::String(atom.data)}
            Data::Regex => {unimplemented!()}
        }
    }
}
