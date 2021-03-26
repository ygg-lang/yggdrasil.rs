use super::*;
use convert_case::{Case, Casing};

use crate::ast::AssignStatement;


impl From<AssignStatement> for Rule {
    fn from(s: AssignStatement) -> Self {
        let mut name = s.id.data;
        let structure = name.to_case(Case::UpperCamel);
        let force_inline = name.starts_with("_");
        if force_inline {
            name = String::from(&name[1..=name.len()])
        }

        Self {
            name,
            structure,
            force_inline,
            eliminate_unmarked: false,
            eliminate_unnamed: false
        }
    }
}