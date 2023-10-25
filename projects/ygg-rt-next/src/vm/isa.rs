use std::fmt::{Display, Formatter};

use crate::vm::charset::Set;

pub struct Program {
    pub instructions: Vec<Inst>,
}

impl Program {
    pub fn size(&self) -> usize {
        let mut count = 0;
        for i in &self.instructions {
            match i {
                Inst::Label(_) | Inst::Nop => continue,
                _ => count += 1,
            }
        }
        count
    }
}

pub enum Inst {
    /// Label is used for marking a location in the instruction code with
    /// a unique ID
    Label { id: usize },
    /// Char consumes the next byte of the subject if it matches Byte and
    /// fails otherwis   Char { byte: u8 },
    /// Jump jumps to Lbl.
    Jump { lbl: usize },
    /// Choice pushes Lbl to the stack and if there is a failure the label will
    /// be popped from the stack and jumped to.
    Choice { lbl: usize },
    /// Call pushes the next instruction to the stack as a return address and jumps
    /// to Lbl.
    Call { lbl: usize },
    /// Commit jumps to Lbl and removes the top entry from the stack
    Commit { lbl: usize },
    /// Return pops a return address off the stack and jumps to it.
    Return,
    /// Fail causes the instruction pointer to go to the fail state.
    Fail,
    /// Set consumes the next byte of input if it is in the set of chars defined
    /// by Chars.
    Set { chars: Set },
    /// Any consumes the next N bytes and fails if that is not possible.
    Any { n: usize },
    /// PartialCommit modifies the backtrack entry on the top of the stack to
    /// point to the current subject offset, and jumps to Lbl.
    PartialCommit { lbl: usize },
    /// Span consumes zero or more bytes in the set Chars. This instruction
    /// never fails.
    Span { chars: Set },
    /// BackCommit pops a backtrack entry off the stack, goes to the subject
    /// position in the entry, and jumps to Lbl.
    BackCommit { lbl: usize },
    /// FailTwice pops an entry off the stack and sets the instruction pointer to
    /// the fail state.
    FailTwice,
    /// Empty makes a zero-width assertion according to the Op option. We use the
    /// same zero-width assertions that are supported by Go's regexp package.
    Empty { op: syntax::EmptyOp },
    /// TestChar consumes the next byte if it matches Byte and jumps to Lbl
    /// otherwise. If the consumption is possible, a backtrack entry referring
    /// to Lbl and the subject position from before consumption is pushed to the
    /// stack.
    TestChar { byte: u8, lbl: usize },
    /// TestCharNoChoice consumes the next byte if it matches Byte and jumps to Lbl
    /// otherwise. No backtrack entry is pushed to the stack.
    TestCharNoChoice { byte: u8, lbl: usize },
    /// TestSet consumes the next byte if it is in the set Chars and jumps to
    /// Lbl otherwise. If the consumption is possible, a backtrack entry referring
    /// to Lbl and the subject position from before consumption is pushed to the
    /// stack.
    TestSet { chars: Set, lbl: usize },
    /// TestSetNoChoice is the same as TestSet but no backtrack entry is pushed to
    /// the stack.
    TestSetNoChoice { chars: Set, lbl: usize },
    /// TestAny consumes the next N bytes and jumps to Lbl if that is not possible.
    /// If the consumption is possible, a backtrack entry referring to Lbl and
    /// the subject position from before consumption is pushed to the stack.
    TestAny { n: usize, lbl: usize },
    /// End immediately completes the pattern as a match.
    End { fail: bool },
    /// Nop does nothing.
    Nop,
    /// MemoOpen begins a memo entry at this position. It marks the pattern that is
    /// being memoized with a unique ID for that pattern, and stores a label to
    /// jump to if the pattern is found in the memoization table.
    MemoOpen { lbl: usize, id: usize },
    /// MemoClose completes a memoization entry and adds the entry into the memo
    /// table if it meets certain conditions (size, or other heuristics).
    MemoClose,
    /// MemoTreeOpen starts a memoization tree repetition routine.
    MemoTreeOpen { lbl: usize, id: usize },
    /// MemoTreeInsert performs insertion into the memoization table for the tree
    /// memoization strategy.
    MemoTreeInsert,
    /// MemoTree "tree-ifies" the current memoization entries on the stack.
    MemoTree,
    /// MemoTreeClose completes the tree memoization routine.
    MemoTreeClose { id: usize },
    /// CaptureBegin begins capturing the given ID.
    CaptureBegin { id: usize },
    /// CaptureLate begins capturing the given ID at the current subject position
    /// minus Back.
    CaptureLate { back: u8, id: usize },
    /// CaptureEnd completes an active capture.
    CaptureEnd { id: usize },
    /// CaptureFull begins a capture for the given ID at the current subject
    /// position minus Back, and immediately completes the capture. This is
    /// equivalent to CaptureLate Back ID; CaptureEnd.
    CaptureFull { back: u8, id: usize },
    /// CheckBegin marks the beginning position for a checker.
    CheckBegin { id: usize, flag: usize },
    /// CheckEnd records the end position of a checker and applies the checker to
    /// determine if the match should fail.
    CheckEnd { checker: Checker },
    /// Error logs an error message at the current position.
    Error { message: String },
}

impl Display for Inst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Inst::Label { id } => write!(f, "L{}", id),
            Inst::Jump { lbl } => write!(f, "Jump {}", lbl),
            Inst::Choice { lbl } => write!(f, "Choice {}", lbl),
            Inst::Call { lbl } => write!(f, "Call {}", lbl),
            Inst::Commit { lbl } => write!(f, "Commit {}", lbl),
            Inst::Return => write!(f, "Return"),
            Inst::Fail => write!(f, "Fail"),
            Inst::Set { chars } => write!(f, "Set {}", chars),
            Inst::Any { n } => write!(f, "Any {}", n),
            Inst::PartialCommit { lbl } => write!(f, "PartialCommit {}", lbl),
            Inst::Span { chars } => write!(f, "Span {}", chars),
            Inst::BackCommit { lbl } => write!(f, "BackCommit {}", lbl),
            Inst::FailTwice => write!(f, "FailTwice"),
            Inst::Empty { op } => write!(f, "Empty {}", empty_to_string(*op)),
            Inst::TestChar { byte, lbl } => {
                write!(f, "TestChar {} {}", byte as char, lbl)
            }
            Inst::TestCharNoChoice { byte, lbl } => {
                write!(f, "TestCharNoChoice {} {}", byte as char, lbl)
            }
            Inst::TestSet { chars, lbl } => {
                write!(f, "TestSet {} {}", chars, lbl)
            }
            Inst::TestSetNoChoice { chars, lbl } => {
                write!(f, "TestSetNoChoice {} {}", chars, lbl)
            }
            Inst::TestAny { n, lbl } => write!(f, "TestAny {} {}", n, lbl),
            Inst::End { fail } => {
                let result = if *fail { "Fail" } else { "Success" };
                write!(f, "End {}", result)
            }
            Inst::Nop => write!(f, "Nop"),
            Inst::MemoOpen { lbl, id } => write!(f, "MemoOpen {} {}", lbl, id),
            Inst::MemoClose => write!(f, "MemoClose"),
            Inst::MemoTreeOpen { lbl, id } => write!(f, "MemoTreeOpen {} {}", lbl, id),
            Inst::MemoTreeInsert => write!(f, "MemoTreeInsert"),
            Inst::MemoTree => write!(f, "MemoTree"),
            Inst::MemoTreeClose { id } => write!(f, "MemoTreeClose {}", id),
            Inst::CaptureBegin { id } => write!(f, "CaptureBegin {}", id),
            Inst::CaptureLate { back, id } => write!(f, "CaptureLate {} {}", back, id),
            Inst::CaptureEnd { id } => write!(f, "CaptureEnd {}", id),
            Inst::CaptureFull { back, id } => write!(f, "CaptureFull {} {}", back, id),
            Inst::CheckBegin { id, flag } => write!(f, "CheckBegin {} {}", id, flag),
            Inst::CheckEnd { checker } => write!(f, "CheckEnd {}", checker),
            Inst::Error { message } => write!(f, "Error {}", message),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut last = Inst::Nop;
        for i in &self.instructions {
            match i {
                Inst::Label { .. } | Inst::Nop => continue,
                _ => s += &format!("\t{}\n", i),
            }
            last = i.clone();
        }
        write!(f, "{}", s)
    }
}

fn empty_to_string(op: EmptyOp) -> &'static str {
    match op {
        syntax::EmptyBeginLine => "BeginLine",
        syntax::EmptyEndLine => "EndLine",
        syntax::EmptyBeginText => "BeginText",
        syntax::EmptyEndText => "EndText",
        syntax::EmptyWordBoundary => "WordBoundary",
        syntax::EmptyNoWordBoundary => "NoWordBoundary",
    }
}

// func emptyToString(op syntax.EmptyOp) string {
// 	switch op {
// 	case syntax.EmptyBeginLine:
// 		return "BeginLine"
// 	case syntax.EmptyEndLine:
// 		return "EndLine"
// 	case syntax.EmptyBeginText:
// 		return "BeginText"
// 	case syntax.EmptyEndText:
// 		return "EndText"
// 	case syntax.EmptyWordBoundary:
// 		return "WordBoundary"
// 	case syntax.EmptyNoWordBoundary:
// 		return "NoWordBoundary"
// 	}
// 	return "Unknown"
// }
