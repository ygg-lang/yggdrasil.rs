use fs::read_to_string;
use std::{
    fmt::{Arguments, Write},
    fs,
    fs::File,
    io::Write as _,
    path::Path,
};

use peginator::{
    codegen::{CodegenGrammar, CodegenSettings},
    grammar::Grammar,
    PegParser,
};
use proc_macro2::TokenStream;

use yggdrasil_error::{Validation, YggdrasilError};
use yggdrasil_ir::{
    ChoiceExpression, CodeGenerator, ConcatExpression, DataKind, ExpressionKind, ExpressionNode, FunctionExpression,
    GrammarInfo, GrammarRule, Operator, RuleReference, UnaryExpression,
};

use crate::parser::GrammarParser;

mod build_data;
mod build_symbol;

pub struct PegCodegen {
    buffer: String,
}

impl Default for PegCodegen {
    fn default() -> Self {
        Self { buffer: "".to_string() }
    }
}

impl PegCodegen {
    pub fn codegen(&mut self, src: impl AsRef<Path>) -> Result<(), YggdrasilError> {
        self.buffer.clear();
        let path = src.as_ref().to_path_buf().canonicalize()?;
        let dir = match path.parent() {
            Some(s) => s,
            None => return Err(YggdrasilError::runtime_error("ygg dir not found")),
        };
        let mut peg = File::create(path.with_extension("ebnf"))?;
        let text = read_to_string(&path)?;
        let info = match GrammarParser::parse(&text) {
            Validation::Success { value, diagnostics } => value,
            Validation::Failure { fatal, diagnostics } => return Err(fatal),
        };
        let tokens = match self.generate(&info) {
            Validation::Success { value, diagnostics } => value,
            Validation::Failure { fatal, diagnostics } => return Err(fatal),
        };
        write!(peg, "{}", self.buffer)?;
        Ok(())
    }
}

impl CodeGenerator for PegCodegen {
    type Output = TokenStream;

    fn generate(&mut self, info: &GrammarInfo) -> Validation<Self::Output> {
        let mut errors = vec![];
        for (_, rule) in &info.rules {
            match rule.write_peg(self, info) {
                Ok(_) => {}
                Err(e) => return Validation::Failure { fatal: YggdrasilError::from(e), diagnostics: vec![] },
            }
        }
        let parsed = match Grammar::parse(&self.buffer) {
            Ok(o) => o,
            Err(e) => return Validation::Failure { fatal: YggdrasilError::from(e), diagnostics: vec![] },
        };
        let config = CodegenSettings {
            skip_whitespace: false,
            peginator_crate_name: "peginator".into(),
            derives: vec!["Debug".into(), "Clone".into()],
        };
        let success = parsed.generate_code(&config).unwrap();
        Validation::Success { value: success, diagnostics: errors }
    }
}

trait WritePeg {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result;
}

impl WritePeg for GrammarRule {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result {
        match self.r#type.to_ascii_lowercase().as_str() {
            "str" | "string" => w.write_str("@position @string\n")?,
            "char" | "character" => w.write_str("@char\n")?,
            _ => w.write_str("@position\n")?,
        }
        write!(w, "{}{}{} = ", info.rule_prefix, self.name, info.rule_suffix)?;
        self.body.write_peg(w, info)?;
        w.semicolon();
        Ok(())
    }
}

impl WritePeg for ExpressionNode {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result {
        w.tag(&self.tag);
        match &self.kind {
            ExpressionKind::Unary(e) => e.write_peg(w, info),
            ExpressionKind::Choice(e) => e.write_peg(w, info),
            ExpressionKind::Concat(e) => e.write_peg(w, info),
            ExpressionKind::Data(e) => e.write_peg(w, info),
            ExpressionKind::Function(e) => e.write_peg(w, info),
            ExpressionKind::Rule(e) => e.write_peg(w, info),
        }
    }
}

impl WritePeg for FunctionExpression {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result {
        todo!()
    }
}

impl WritePeg for UnaryExpression {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result {
        let mut pre = vec![];
        let mut post = vec![];
        for op in &self.ops {
            match op {
                Operator::Negative => {
                    pre.push("!(");
                    post.push(")")
                }
                Operator::Optional => {
                    pre.push("[");
                    post.push("]")
                }
                Operator::Repeats => {
                    pre.push("{");
                    post.push("}")
                }
                Operator::Repeat1 => {
                    pre.push("{");
                    post.push("}+")
                }
                Operator::RepeatsBetween(_, _) => {
                    // pre.push("(");
                    // post.push(")")
                }
                Operator::Remark => {
                    // pre.push("(");
                    // post.push(")")
                }
                Operator::Recursive => {
                    // pre.push("(");
                    // post.push(")")
                }
                Operator::Boxing => {
                    // pre.push("(");
                    // post.push(")")
                }
            }
        }
        for s in pre {
            w.write_str(s)?
        }
        self.base.write_peg(w, info)?;
        for s in post {
            w.write_str(s)?
        }
        Ok(())
    }
}

impl WritePeg for ChoiceExpression {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result {
        for (id, expr) in self.branches.iter().enumerate() {
            if id != 0 {
                w.write_str(" | ")?;
            }
            // w.write_start();
            expr.write_peg(w, info)?;
            // w.write_end();
        }

        Ok(())
    }
}

impl WritePeg for ConcatExpression {
    fn write_peg(&self, w: &mut PegCodegen, info: &GrammarInfo) -> std::fmt::Result {
        w.write_start();
        for (index, expr) in self.sequence.iter().enumerate() {
            if index != 0 {
                w.write_char(' ')?
            }
            expr.write_peg(w, info)?;
        }
        w.write_end();
        Ok(())
    }
}
