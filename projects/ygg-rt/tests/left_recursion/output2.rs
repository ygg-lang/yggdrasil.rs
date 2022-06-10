use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use yggdrasil_rt::ast_mode::{YResult, YState};

/// ```bnf
/// expr: expr '+' expr #Add
///     | expr '^' expr #Pow
///     | atom          #Atom
/// ```
///
/// ```bnf
/// expr: add
/// add:
///     | pow ('+' pow)*
/// pow:
///     | atom ('^' atom)*
/// atom
/// ```
pub enum Expr {
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Pow { lhs: Box<Expr>, rhs: Box<Expr> },
    Atom(usize),
}

impl Expr {
    pub fn parse(state: YState) -> YResult<Expr> {
        let (state, expr_lifted) = ExprAdd::parse(state)?;
        /// build left bind add
        state.finish(expr_lifted.ascent())
    }
}

/// `pow ('+' pow)*`
struct ExprAdd {
    pow: Vec<ExprPow>,
}

impl ExprAdd {
    // fold tree by left
    fn ascent(mut self) -> Expr {
        let mut expr = self.pow.remove(0).ascent();
        for pow in self.pow.into_iter() {
            expr = Expr::Add { lhs: Box::new(expr), rhs: Box::new(pow.ascent()) };
        }
        expr
    }
    fn parse(state: YState) -> YResult<ExprAdd> {
        let mut pow = vec![];
        let (state, pow1) = ExprPow::parse(state)?;
        pow.push(pow1);
        let (state, pow2) = state.match_repeat_m_n(0, usize::MAX, ExprAdd::parse_aux1)?;
        pow.extend(pow2);
        state.finish(ExprAdd { pow })
    }
    fn parse_aux1(state: YState) -> YResult<ExprPow> {
        let (state, _) = state.match_char('+')?;
        let (state, pow) = ExprPow::parse(state)?;
        state.finish(pow)
    }
}

/// `atom ('^' atom)*`
struct ExprPow {
    atom: Vec<ExprAtom>,
}

impl ExprPow {
    pub fn parse(state: YState) -> YResult<ExprPow> {
        let mut atom = vec![];
        let (state, atom1) = ExprAtom::parse(state)?;
        atom.push(atom1);
        let (state, atom2) = state.match_repeat_m_n(0, usize::MAX, ExprPow::parse_aux1)?;
        atom.extend(atom2);
        state.finish(ExprPow { atom })
    }
    // fold tree by right
    pub fn ascent(mut self) -> Expr {
        let mut expr = self.atom.remove(self.atom.len() - 1).ascent();
        for atom in self.atom.into_iter().rev() {
            expr = Expr::Pow { lhs: Box::new(atom.ascent()), rhs: Box::new(expr) };
        }
        expr
    }
    pub fn parse_aux1(state: YState) -> YResult<ExprAtom> {
        let (state, _) = state.match_char('^')?;
        let (state, atom) = ExprAtom::parse(state)?;
        state.finish(atom)
    }
}

enum ExprAtom {
    Atom(usize),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Add { lhs, rhs } => write!(f, "({:?} + {:?})", lhs, rhs),
            Expr::Pow { lhs, rhs } => write!(f, "({:?} ^ {:?})", lhs, rhs),
            Expr::Atom(atom) => write!(f, "{}", atom),
        }
    }
}

impl ExprAtom {
    pub fn parse(state: YState) -> YResult<ExprAtom> {
        let (state, atom) = state.match_repeat_m_n(1, usize::MAX, |state| state.match_char_range('0', '9'))?;
        let num = String::from_iter(atom);
        state.finish(ExprAtom::Atom(usize::from_str(&num).unwrap()))
    }
    pub fn ascent(&self) -> Expr {
        match self {
            ExprAtom::Atom(atom) => Expr::Atom(*atom),
        }
    }
}

#[test]
fn test_output_1() {
    println!("{:#?}", Expr::parse(YState::new("1+2+3")));
    println!("{:#?}", Expr::parse(YState::new("4^5^6")));
    println!("{:#?}", Expr::parse(YState::new("1+2+3+4^5^6")));
}
