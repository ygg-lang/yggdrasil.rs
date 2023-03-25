use super::*;

impl<'i, R> Debug for TokenTree<'i, R>
where
    R: YggdrasilRule,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'i, R> Display for TokenTree<'i, R>
where
    R: YggdrasilRule,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl<'i, R> TokenTree<'i, R>
where
    R: YggdrasilRule,
{
    fn pretty_print(&self, f: &mut Formatter<'_>, depth: usize) -> core::fmt::Result {
        for term in self.clone().into_iter() {
            if !f.alternate() && term.as_rule().is_ignore() {
                continue;
            }
            for _ in 0..depth {
                f.write_str("  ")?
            }
            if term.has_child() {
                f.write_str("\x1b[34m+\x1b[0m ")?
            }
            else {
                f.write_str("\x1b[34m*\x1b[0m ")?
            }
            let span = term.as_span();
            write!(f, "{:?}({}..{}", term.as_rule(), span.start, span.end)?;
            match term.as_node_tag() {
                Some(s) => write!(f, ", {s})")?,
                None => write!(f, ")")?,
            }
            writeln!(f, ": \x1b[32m{:?}\x1b[0m", span.input)?;

            term.into_inner().pretty_print(f, depth + 1)?
        }
        Ok(())
    }
}
