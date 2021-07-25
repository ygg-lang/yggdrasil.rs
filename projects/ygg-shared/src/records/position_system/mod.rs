mod offset;

pub use self::offset::OffsetRange;

use std::fmt::{Debug, Formatter};

pub struct LineBreaks<'input> {
    input: &'input str,
    lines: Vec<usize>,
}

impl<'i> LineBreaks<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { input, lines: Self::count_lines(input) }
    }
    pub fn update(&mut self, input: &'i str) {
        self.input = input;
        self.lines = Self::count_lines(input);
    }
    pub fn count_lines(input: &str) -> Vec<usize> {
        let mut counter = 0;
        let mut out = vec![];
        for line in input.lines() {
            counter += line.len();
            out.push(counter)
        }
        return out;
    }
    pub fn get_text(&self) -> &'i str {
        self.input
    }
    pub fn get_lines(&self) -> &[usize] {
        self.lines.as_slice()
    }
    pub fn get_nth_line(&self, line: usize) -> Option<&'_ str> {
        self.input.lines().nth(line)
    }
}

impl<'i> LineBreaks<'i> {
    pub fn get_line(&self, offset: usize) -> usize {
        let mut lower = 0;
        let mut upper = self.lines.len() - 1;
        while lower < upper {
            let mid = (lower + upper) / 2;
            unsafe {
                let line = *self.lines.get_unchecked(mid);
                match line > offset {
                    true => upper = mid,
                    false => lower = mid + 1,
                }
            };
        }
        return lower - 1;
    }
    pub fn get_line_column(&self, offset: usize) -> (usize, usize) {
        let line = self.get_line(offset);
        let line_break = unsafe { *self.lines.get_unchecked(line) };
        return (line, offset.saturating_sub(line_break));
    }
}
