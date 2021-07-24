use crate::records::position_system::LineBreaks;
use lsp_types::{Position, Range};

#[cfg(test)]
mod test;

impl<'i> LineBreaks<'i> {
    #[inline]
    pub fn get_lsp_range(&self, start: usize, end: usize) -> Range {
        Range { start: self.get_lsp_start(start), end: self.get_lsp_start(end) }
    }
    #[inline]
    pub fn get_lsp_start(&self, start: usize) -> Position {
        self.get_lsp_position(start)
    }
    #[inline]
    pub fn get_lsp_end(&self, end: usize) -> Position {
        self.get_lsp_position(end)
    }
    #[inline]
    fn get_lsp_position(&self, offset: usize) -> Position {
        let (line, column) = self.get_line_column(offset);
        let character = match self.input.lines().nth(line) {
            Some(s) => unsafe { s.get_unchecked(0..column).encode_utf16().count() as u32 },
            None => column as u32,
        };

        Position { line: line as u32, character }
    }
}
