use crate::{YError, YResult};
use std::{iter::Peekable, vec::IntoIter};

pub type PrecedenceNumber = u16;

#[derive(Copy, Clone)]
pub enum Associativity {
    Left,
    Right,
    Neither,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Precedence(PrecedenceNumber);

impl Precedence {
    pub const fn new(i: PrecedenceNumber) -> Self {
        Self(i)
    }

    const fn raise(mut self) -> Precedence {
        self.0 += 1;
        self
    }
    const fn lower(mut self) -> Precedence {
        self.0 -= 1;
        self
    }
    const fn normalize(mut self) -> Precedence {
        self.0 *= 10;
        self
    }
    const fn min() -> Precedence {
        Precedence(PrecedenceNumber::MIN)
    }
    const fn max() -> Precedence {
        Precedence(PrecedenceNumber::MAX)
    }
}

#[derive(Copy, Clone)]
pub enum Affix {
    Infix(Precedence, Associativity),
    Prefix(Precedence),
    Suffix(Precedence),
    None,
}

impl Affix {
    pub fn infix_left(p: PrecedenceNumber) -> Self {
        Self::Infix(Precedence(p), Associativity::Left)
    }
    pub fn infix_right(p: PrecedenceNumber) -> Self {
        Self::Infix(Precedence(p), Associativity::Right)
    }
    pub fn infix(p: PrecedenceNumber, a: Associativity) -> Self {
        Self::Infix(Precedence(p), a)
    }
    pub fn suffix(p: PrecedenceNumber) -> Self {
        Self::Prefix(Precedence(p))
    }
    pub fn prefix(p: PrecedenceNumber) -> Self {
        Self::Prefix(Precedence(p))
    }
}

pub trait PrattParser {
    type Input: Clone;
    type Output: Sized;

    fn query(&mut self, input: &Self::Input) -> YResult<Affix>;

    fn primary(&mut self, input: Self::Input) -> YResult<Self::Output>;

    fn infix(&mut self, lhs: Self::Output, op: Self::Input, rhs: Self::Output) -> YResult<Self::Output>;

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> YResult<Self::Output>;

    fn suffix(&mut self, lhs: Self::Output, op: Self::Input) -> YResult<Self::Output>;

    fn parse(&mut self, inputs: &[Self::Input]) -> YResult<Self::Output> {
        let mut stream = inputs.to_vec().into_iter().peekable();
        self.parse_input(&mut stream, Precedence(0))
    }

    fn parse_input(&mut self, tail: &mut Peekable<IntoIter<Self::Input>>, rbp: Precedence) -> YResult<Self::Output> {
        if let Some(head) = tail.next() {
            let info = self.query(&head)?;
            let mut nbp = self.nbp(info);
            let mut node = self.nud(head, tail, info);
            while let Some(head) = tail.peek() {
                let info = self.query(head)?;
                let lbp = self.lbp(info);
                if rbp < lbp && lbp < nbp {
                    let head = tail.next().unwrap();
                    nbp = self.nbp(info);
                    node = self.led(head, tail, info, node?);
                }
                else {
                    break;
                }
            }
            node
        }
        else {
            Err(YError::syntax_error("EmptyInput"))
        }
    }

    /// Null-Denotation
    fn nud(&mut self, head: Self::Input, tail: &mut Peekable<IntoIter<Self::Input>>, info: Affix) -> YResult<Self::Output> {
        match info {
            Affix::Prefix(precedence) => {
                let rhs = self.parse_input(tail, precedence.normalize().lower());
                self.prefix(head, rhs?)
            }
            Affix::None => self.primary(head),
            Affix::Suffix(_) => Err(YError::syntax_error("Unexpected Postfix")),
            Affix::Infix(_, _) => Err(YError::syntax_error("Unexpected Infix")),
        }
    }

    /// Left-Denotation
    fn led(
        &mut self,
        head: Self::Input,
        tail: &mut Peekable<IntoIter<Self::Input>>,
        info: Affix,
        lhs: Self::Output,
    ) -> YResult<Self::Output> {
        match info {
            Affix::Infix(precedence, associativity) => {
                let precedence = precedence.normalize();
                let rhs = match associativity {
                    Associativity::Left => self.parse_input(tail, precedence),
                    Associativity::Right => self.parse_input(tail, precedence.lower()),
                    Associativity::Neither => self.parse_input(tail, precedence.raise()),
                };
                self.infix(lhs, head, rhs?)
            }
            Affix::Suffix(_) => self.suffix(lhs, head),
            Affix::None => Err(YError::syntax_error("Unexpected NilFix")),
            Affix::Prefix(_) => Err(YError::syntax_error("Unexpected Prefix")),
        }
    }

    //         <lbp>  <rbp>  <nbp> <kind>
    // Nilfix:  MIN |  MIN |  MAX | nud
    // Prefix:  MIN |   bp |  MAX | nud
    // Postfix:  bp |  MIN |  MAX | led
    // InfixL:   bp |   bp | bp+1 | led
    // InfixR:   bp | bp-1 | bp+1 | led
    // InfixN:   bp |   bp |   bp | led

    /// Left-Binding-Power
    fn lbp(&mut self, info: Affix) -> Precedence {
        match info {
            Affix::None => Precedence::min(),
            Affix::Prefix(_) => Precedence::min(),
            Affix::Suffix(precedence) => precedence.normalize(),
            Affix::Infix(precedence, _) => precedence.normalize(),
        }
    }

    /// Next-Binding-Power
    fn nbp(&mut self, info: Affix) -> Precedence {
        match info {
            Affix::None => Precedence::max(),
            Affix::Prefix(_) => Precedence::max(),
            Affix::Suffix(_) => Precedence::max(),
            Affix::Infix(precedence, Associativity::Left) => precedence.normalize().raise(),
            Affix::Infix(precedence, Associativity::Right) => precedence.normalize().raise(),
            Affix::Infix(precedence, Associativity::Neither) => precedence.normalize(),
        }
    }
}
