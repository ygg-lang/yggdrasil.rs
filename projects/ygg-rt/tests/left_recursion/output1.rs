use std::ops::Try;
use yggdrasil_rt::ast_mode::{Parsed, YResult, YState};

// E  → bE'
// E' → aE' | ε
//
// expr: expr '+' expr #AddExpr
//     | atom          #Atom
//
//
// expr: atom add_expr
// add_expr: '+' add_expr | EOF
#[derive(Debug)]
enum Expr {
    Add(Box<AddExpr>),
    Atom(usize),
}

#[derive(Debug)]
struct AddExpr {
    rhs: Expr,
}

/// `atom add_expr`
fn parse_expr(state: YState) -> YResult<Expr> {
    let start = state.start_offset;
    let Parsed(state, atom) = parse_atom(state)?;
    let Parsed(state, add_expr) = parse_add_expr(state)?;
    let Parsed(state, c) = state.parse_char('c')?;
    let range = start..state.start_offset;
    Parsed::ok(state, Output1 { a, b, c, range })
}

/// '+' add_expr | EOF
fn parse_add_expr(state: YState) -> YResult<AddExpr> {
    let start = state.start_offset;
    let Parsed(state, atom) = state.parse_char();
    let Parsed(state, add_expr) = state.parse_repeats(1, 3, |state| state.parse_char('b'))?;
    let Parsed(state, c) = state.parse_char('c')?;
    let range = start..state.start_offset;
    Parsed::ok(state, Output1 { a, b, c, range })
}

fn parse_atom(state: YState) -> YResult<usize> {
    let Parsed(state, atom) = state.parse_repeats(0, usize::MAX, |state| state.parse_char_range('0', '9'));

    Parsed::ok(state, atom.value)
}

#[test]
fn test_output_1() {
    println!("{:#?}", parse_add_expr(YState::new("ac")));
    println!("{:#?}", parse_add_expr(YState::new("abc")));
    println!("{:#?}", parse_add_expr(YState::new("abbc")));
    println!("{:#?}", parse_add_expr(YState::new("abbbc")));
    println!("{:#?}", parse_add_expr(YState::new("abbbbc")));
}
