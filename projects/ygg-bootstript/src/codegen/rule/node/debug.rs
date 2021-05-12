use super::*;

impl Debug for YGGRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("name", &self.name.data)
            .field("type", &self.ty.data)
            .field("force_inline", &self.force_inline)
            .field("already_inline", &self.already_inline)
            .field("eliminate_unmarked", &self.eliminate_unmarked)
            .field("eliminate_unnamed", &self.eliminate_unnamed)
            .field("expression", &self.expression)
            .field("range", &self.range)
            .finish()
    }
}

impl Debug for ExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut w = &mut f.debug_struct("ExpressionNode");
        if let Some(s) = &self.tag {
            w.field("tag", &s.tag.data);
            w.field("tag_mode", &s.mode);
        }
        if let Some(s) = &self.field {
            w.field("field", &s.data);
        }
        if let Some(s) = &self.ty {
            w.field("type", &s.data);
        }
        w.field("base", &self.node);
        w.finish()
    }
}

impl Debug for RefinedExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RefinedExpression::Data(e) => {e.fmt(f)}
            RefinedExpression::Unary(e) => {e.fmt(f)}
            RefinedExpression::Choice(e) => {e.fmt(f)}
            RefinedExpression::Concat(e) => {e.fmt(f)}
        }
    }
}

impl Debug for RefinedChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ChoiceExpression")?;
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl Debug for RefinedConcat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ConcatExpression")?;
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl Debug for RefinedData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RefinedData::Identifier(e) => {e.fmt(f)}
            RefinedData::String(e) => {e.fmt(f)}
            RefinedData::Regex(e) => {e.fmt(f)}
            RefinedData::Integer(e) => {e.fmt(f)}
        }
    }
}