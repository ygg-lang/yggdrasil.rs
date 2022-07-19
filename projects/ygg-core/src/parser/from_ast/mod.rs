use yggdrasil_ir::{QErrorKind, SyntaxError};

use super::*;

impl ParseContext {
    #[inline]
    pub fn syntax_error<S>(&mut self, message: S, position: &Range<usize>)
    where
        S: Into<String>,
    {
        let kind = SyntaxError { message: message.into(), file: Default::default(), span: position.clone() };
        let error = QError { error: Box::new(QErrorKind::Syntax(kind)), level: Default::default() };
        self.errors.push(error);
    }
}
