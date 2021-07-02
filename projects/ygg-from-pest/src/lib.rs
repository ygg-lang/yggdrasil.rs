use std::fmt::{Debug, Formatter, Write};
use pest_meta::ast::Rule;

pub trait FromPest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl FromPest for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str()
    }
}

impl FromPest {
    
}