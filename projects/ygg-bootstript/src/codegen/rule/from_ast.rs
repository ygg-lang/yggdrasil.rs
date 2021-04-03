use super::*;
use convert_case::{Case, Casing};

use crate::ast::AssignStatement;

impl From<AssignStatement> for YGGRule {
    fn from(s: AssignStatement) -> Self {
        let mut name = s.id.data;
        let mut structure = None;
        let force_inline = name.starts_with("_");
        match force_inline {
            true => name = String::from(&name[1..=name.len()]),
            false => structure = Some(name.to_case(Case::UpperCamel)),
        }
        let mut eliminate_unmarked = false;
        let mut eliminate_unnamed = false;
        match s.eq.as_str() {
            "_=" => eliminate_unnamed = true,
            "^=" => eliminate_unmarked = true,
            _ => unreachable!(),
        }

        Self { name, structure, force_inline, eliminate_unmarked, eliminate_unnamed }
    }
}
