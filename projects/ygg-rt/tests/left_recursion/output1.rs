use std::str::FromStr;

use yggdrasil_rt::{YResult, YState};

// P  →  Pα | β
// P  →  βP'
// P' →  αP' | ε
//
// expr: expr '+' expr #AddExpr
//     | atom          #Atom
#[derive(Debug)]
pub enum Expr {
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Atom(usize),
}

impl Expr {
    pub fn parse(state: YState) -> YResult<Expr> {
        let (state, expr_lifted) = ExprLifted::parse(state)?;
        // Flip a binary tree
        // a (b c) -> (a b) c
        // a (b (c d)) -> ((a b) c) d)
        // 1.collect atoms
        let mut atoms = vec![expr_lifted.atom];
        let mut add_expr = &expr_lifted.add_expr;
        loop {
            match &add_expr {
                ExprAdd::Expr { expr } => {
                    atoms.push(expr.atom);
                    add_expr = &expr.add_expr;
                }
                ExprAdd::EOF => break,
            }
        }
        // 2.flip tree
        let mut expr = Expr::Atom(atoms[0]);
        for atom in atoms[1..].iter() {
            expr = Expr::Add { lhs: Box::new(expr), rhs: Box::new(Expr::Atom(*atom)) };
        }
        state.finish(expr)
    }
}

// expr_lift: (lhs:atom +) rhs:atom
#[derive(Debug)]
struct ExprLifted {
    atom: usize,
    add_expr: ExprAdd,
}

impl ExprLifted {
    pub fn parse(state: YState) -> YResult<ExprLifted> {
        let (state, atom) = parse_atom(state)?;
        let (state, add_expr) = ExprAdd::parse(state)?;
        state.finish(ExprLifted { atom, add_expr })
    }
}

// add_expr: '+' expr_lift | eof
#[derive(Debug)]
enum ExprAdd {
    Expr { expr: Box<ExprLifted> },
    EOF,
}

impl ExprAdd {
    fn parse(state: YState) -> YResult<ExprAdd> {
        let (state, add_expr) = state //
            .begin_choice()
            .maybe(ExprAdd::parse_axu1)
            .maybe(ExprAdd::parse_aux2)
            .end_choice()?;
        state.finish(add_expr)
    }
    /// '+' expr_lift
    fn parse_axu1(state: YState) -> YResult<ExprAdd> {
        let (state, _) = state.match_char('+')?;
        let (state, expr) = ExprLifted::parse(state)?;
        state.finish(ExprAdd::Expr { expr: Box::new(expr) })
    }
    /// EOF
    fn parse_aux2(state: YState) -> YResult<ExprAdd> {
        let (state, _) = state.match_eof()?;
        state.finish(ExprAdd::EOF)
    }
}

fn parse_atom(state: YState) -> YResult<usize> {
    let (state, atom) = state.match_repeat_m_n(1, usize::MAX, |state| state.match_char_range('0', '9'))?;
    let num = String::from_iter(atom);
    state.finish(usize::from_str(&num).unwrap())
}

#[test]
fn test_output_1() {
    println!("{:#?}", Expr::parse(YState::new("1")));
    println!("{:#?}", Expr::parse(YState::new("1+2")));
    println!("{:#?}", Expr::parse(YState::new("1+2+3")));
    println!("{:#?}", Expr::parse(YState::new("1+2+3+4")));
}
