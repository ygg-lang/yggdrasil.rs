use std::str::FromStr;

use yggdrasil_rt::ast_mode::{YResult, YState};

// E  → bE'
// E' → aE' | ε
//
// expr: expr '+' expr #AddExpr
//     | atom          #Atom
#[derive(Debug)]
enum Expr {
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Atom(usize),
}

#[derive(Debug)]
enum AddExpr {
    /// '+' expr
    Continue { expr: Box<Expr> },
    /// EOF
    EOF,
}

impl Expr {
    pub fn parse(state: YState) -> YResult<Expr> {
        let (state, atom) = parse_atom(state)?;
        let (state, add_expr) = AddExpr::parse(state)?;
        match add_expr {
            AddExpr::Continue { expr } => state.finish(Expr::Add { lhs: Box::new(Expr::Atom(atom)), rhs: expr }),
            AddExpr::EOF => state.finish(Expr::Atom(atom)),
        }
    }
}

impl AddExpr {
    fn parse(state: YState) -> YResult<AddExpr> {
        let (state, out) = state
            .begin_choice() //
            .maybe(AddExpr::parse_axu1)
            .maybe(AddExpr::parse_aux2)
            .end_choice()?;
        state.finish(out)
    }
    /// '+' expr
    fn parse_axu1(state: YState) -> YResult<AddExpr> {
        let (state, _) = state.parse_char('+')?;
        let (state, expr) = Expr::parse(state)?;
        state.finish(AddExpr::Continue { expr: Box::new(expr) })
    }
    /// EOF
    fn parse_aux2(state: YState) -> YResult<AddExpr> {
        let (state, _) = state.parse_eof()?;
        state.finish(AddExpr::EOF)
    }
}

fn parse_atom(state: YState) -> YResult<usize> {
    let (state, atom) = state.parse_repeats(0, usize::MAX, |state| state.parse_char_range('0', '9'))?;
    state.finish(usize::from_str(&String::from_iter(atom)).unwrap())
}

#[test]
fn test_output_1() {
    println!("{:#?}", Expr::parse(YState::new("1")));
    println!("{:#?}", Expr::parse(YState::new("1+1")));
    println!("{:#?}", Expr::parse(YState::new("1+1+1")));
}
