use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use yggdrasil_rt::ast_mode::{YResult, YState};

/// ```ygg
/// climb Expr {
///     | Atom                    #Atom
///     | lhs:Self '^' rhs:Self   #Pow
///     | lhs:Self '*' rhs:Self   #Mul
///     | lhs:Self '+' rhs:Self   #Add
/// }
///
/// class ExprAtom {
///     Atom
/// }
/// class ExprPow {
///     (lhs:ExprAtom '^')? rhs:ExprPow
/// }
/// class ExprMul {
///     lhs:ExprPow ('*' rhs:ExprPow)?
/// }
/// class ExprAdd {
///     lhs:ExprMul ('+' rhs:ExprMul)?
/// }
/// ```
pub enum Expr {
    Atom { atom: usize },
    Pow { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
}

impl Expr {
    pub fn parse(state: YState) -> YResult<Expr> {
        let (state, expr_lifted) = ExprAdd::parse(state)?;
        /// build left bind add
        state.finish(expr_lifted.ascent())
    }
}

enum ExprAtom {
    Atom(usize),
}

impl ExprAtom {
    pub fn parse(state: YState) -> YResult<ExprAtom> {
        let (state, atom) = state.match_repeats(|state| state.match_char_range('0', '9'))?;
        let num = String::from_iter(atom);
        state.finish(ExprAtom::Atom(usize::from_str(&num).unwrap()))
    }
    pub fn ascent(self) -> Expr {
        match self {
            ExprAtom::Atom(atom) => Expr::Atom { atom },
        }
    }
}

/// `(lhs:ExprAtom '^')? rhs:ExprPow`
struct ExprPow {
    lhs: Option<ExprAtom>,
    rhs: ExprPow,
    has_lhs: bool,
}
/// `lhs:ExprAtom '^'`
struct ExprPowLhs {
    lhs: ExprAtom,
}

impl ExprPow {
    fn ascent(mut self) -> Expr {
        if !self.has_lhs {
            return self.rhs.ascent();
        }
        #[rustfmt::skip]
        unsafe {
            Expr::Pow {
                lhs: Box::new(self.lhs.unwrap_unchecked().ascent()),
                rhs: Box::new(self.rhs.ascent()),
            }
        }
    }
    fn consume(state: YState) -> YResult<Self> {
        let mut lhs = None;

        let (state, expr_pow_lhs) = state.match_optional(ExprPow::parse_lhs)?;
        let (state, expr_pow) = ExprPow::consume(state)?;
        let expr_pow = match expr_pow_lhs {
            Some(s) => ExprPow { lhs: Some(s.lhs), rhs: expr_pow, has_lhs: true },
            None => ExprPow { lhs: None, rhs: expr_pow, has_lhs: false },
        };
        state.finish(expr_pow)
    }
    fn parse_lhs(state: YState) -> YResult<ExprPowLhs> {
        let (state, lhs) = ExprAtom::parse(state)?;
        state.finish(ExprPowLhs { lhs })
    }
}

/// `lhs:ExprPow ('*' rhs:ExprPow)?`
struct ExprMul {
    lhs: ExprPow,
    rhs: Option<ExprPow>,
    has_rhs: bool,
}

struct ExprMulRhs {
    rhs: ExprPow,
}

impl ExprMul {
    pub fn ascent(mut self) -> Expr {
        let mut expr = self.lhs.remove(self.lhs.len() - 1).ascent();
        for atom in self.lhs.into_iter().rev() {
            expr = Expr::Pow { lhs: Box::new(atom.ascent()), rhs: Box::new(expr) };
        }
        expr
    }
    pub fn consume(state: YState) -> YResult<Self> {
        let mut atom = vec![];
        let (state, atom1) = state.match_parse(ExprPow::parse_lhs)?;
        atom.extend(atom1);
        let (state, atom2) = ExprAtom::parse(state)?;
        atom.push(atom2);
        state.finish(Self {})
    }
    // '*' rhs:ExprPow
    pub fn parse_aux1(state: YState) -> YResult<ExprMulRhs> {
        let (state, _) = state.match_char('*')?;
        let (state, expr_pow) = ExprPow::consume(state)?;
        state.finish(ExprMulRhs { rhs: expr_pow })
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
        let (state, pow1) = ExprPow::consume(state)?;
        pow.push(pow1);
        let (state, pow2) = state.match_repeat_m_n(0, usize::MAX, ExprAdd::parse_aux1)?;
        pow.extend(pow2);
        state.finish(ExprAdd { pow })
    }
    fn parse_aux1(state: YState) -> YResult<ExprPow> {
        let (state, _) = state.match_char('+')?;
        let (state, pow) = ExprPow::consume(state)?;
        state.finish(pow)
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Add { lhs, rhs } => write!(f, "({:?} + {:?})", lhs, rhs),
            Expr::Mul { lhs, rhs } => write!(f, "({:?} * {:?})", lhs, rhs),
            Expr::Pow { lhs, rhs } => write!(f, "({:?} ^ {:?})", lhs, rhs),
            Expr::Atom(atom) => write!(f, "{}", atom),
        }
    }
}

#[test]
fn test_output_1() {
    println!("{:#?}", Expr::parse(YState::new("1+2+3")));
    println!("{:#?}", Expr::parse(YState::new("4^5^6")));
    println!("{:#?}", Expr::parse(YState::new("1+2+3+4^5^6")));
}
