use super::*;

/// Line iterator for Spans, created by [`Span::lines()`].
///
/// Iterates all lines that are at least partially covered by the span.
///
/// [`Span::lines()`]: struct.Span.html#method.lines
pub struct Lines<'i> {
    span: &'i Span<'i>,
    pos: usize,
}

impl<'i> Iterator for Lines<'i> {
    type Item = &'i str;
    fn next(&mut self) -> Option<&'i str> {
        if self.pos > self.span.end {
            return None;
        }
        let pos = Position::new(self.span.input, self.pos)?;
        if pos.at_end() {
            return None;
        }
        let line = pos.line_of();
        self.pos = pos.find_line_end();
        Some(line)
    }
}
