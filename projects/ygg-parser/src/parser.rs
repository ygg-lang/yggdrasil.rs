use std::mem::take;

use yggdrasil_ir::{ExpressionKind, ExpressionNode, FunctionRule, GrammarInfo, GrammarRule, Operator, Validation};
use yggdrasil_rt::traits::{Affix, PrattParser};

mod charset;

pub struct GrammarParser {
    info: GrammarInfo,
    atomic: bool,
    capture: bool,
    docs: String,
    file: FileID,
    errors: Vec<YggdrasilError>,
}

impl GrammarParser {
    pub fn parse(input: &str) -> Validation<GrammarInfo> {
        let mut ctx = GrammarParser {
            //
            info: Default::default(),
            atomic: false,
            capture: true,
            docs: "".to_string(),
            file: Default::default(),
            errors: vec![],
        };
        match ProgramParser::parse(input) {
            Ok(o) => match o.program.translate(&mut ctx) {
                Ok(_) => {}
                Err(e) => return Validation::Failure { fatal: e, diagnostics: vec![] },
            },
            Err(e) => {
                let error = SyntaxError { message: e.to_string(), file: Default::default(), span: Default::default() }
                    .as_error(DiagnosticLevel::Error);
                return Validation::Failure { fatal: error, diagnostics: vec![] };
            }
        };
        Validation::Success { value: ctx.info, diagnostics: ctx.errors }
    }
}
