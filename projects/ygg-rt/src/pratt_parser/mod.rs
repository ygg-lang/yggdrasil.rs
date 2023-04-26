use core::{iter::Peekable, marker::PhantomData, ops::BitOr};

use crate::{TokenPair, YggdrasilRule};
use alloc::{boxed::Box, collections::BTreeMap};

/// Associativity of an infix binary operator, used by [`Op::infix(Assoc)`].
///
/// [`Op::infix(Assoc)`]: struct.Op.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Associativity {
    /// Left operator associativity. Evaluate expressions from left-to-right.
    Left,
    /// Right operator associativity. Evaluate expressions from right-to-left.
    Right,
}

/// An operator that corresponds to a rule.
pub struct YggdrasilOperator<R: YggdrasilRule> {
    rule: R,
    affix: Affix,
    next: Option<Box<YggdrasilOperator<R>>>,
}

type Priority = u32;

const STEP: Priority = 10;

enum Affix {
    Prefix,
    Postfix,
    Infix(Associativity),
}

impl<R: YggdrasilRule> YggdrasilOperator<R> {
    /// Defines `rule` as a prefix unary operator.
    pub fn prefix(rule: R) -> Self {
        Self { rule, affix: Affix::Prefix, next: None }
    }

    /// Defines `rule` as a postfix unary operator.
    pub fn postfix(rule: R) -> Self {
        Self { rule, affix: Affix::Postfix, next: None }
    }

    /// Defines `rule` as an infix binary operator with associativity `assoc`.
    pub fn infix(rule: R, assoc: Associativity) -> Self {
        Self { rule, affix: Affix::Infix(assoc), next: None }
    }
}

impl<R: YggdrasilRule> BitOr for YggdrasilOperator<R> {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self {
        fn assign_next<R: YggdrasilRule>(op: &mut YggdrasilOperator<R>, next: YggdrasilOperator<R>) {
            if let Some(ref mut child) = op.next {
                assign_next(child, next);
            }
            else {
                op.next = Some(Box::new(next));
            }
        }

        assign_next(&mut self, rhs);
        self
    }
}

/// Struct containing operators and precedences, which can perform [Pratt parsing][1] on
/// primary, prefix, postfix and infix expressions over [`Pairs`]. The tokens in [`Pairs`]
/// should alternate in the order:
/// `prefix* ~ primary ~ postfix* ~ (infix ~ prefix* ~ primary ~ postfix*)*`
///
/// # Panics
///
/// Panics will occur when:
/// * `pairs` is empty
/// * The tokens in `pairs` does not alternate in the expected order.
/// * No `map_*` function is specified for a certain kind of operator encountered in `pairs`.
///
/// # Example
///
/// The following pest grammar defines a calculator which can be used for Pratt parsing.
///
/// ```pest
/// WHITESPACE   =  _{ " " | "\t" | NEWLINE }
///  
/// program      =   { SOI ~ expr ~ EOI }
///   expr       =   { prefix* ~ primary ~ postfix* ~ (infix ~ prefix* ~ primary ~ postfix* )* }
///     infix    =  _{ add | sub | mul | div | pow }
///       add    =   { "+" } // Addition
///       sub    =   { "-" } // Subtraction
///       mul    =   { "*" } // Multiplication
///       div    =   { "/" } // Division
///       pow    =   { "^" } // Exponentiation
///     prefix   =  _{ neg }
///       neg    =   { "-" } // Negation
///     postfix  =  _{ fac }
///       fac    =   { "!" } // Factorial
///     primary  =  _{ int | "(" ~ expr ~ ")" }
///       int    =  @{ (ASCII_NONZERO_DIGIT ~ ASCII_DIGIT+ | ASCII_DIGIT) }
/// ```
///
/// Below is a [`PrattParser`] that is able to parse an `expr` in the above grammar. The order
/// of precedence corresponds to the order in which [`op`] is called. Thus, `mul` will
/// have higher precedence than `add`. Operators can also be chained with `|` to give them equal
/// precedence.
///
/// ```
/// # use yggdrasil_rt::{PrattParser, YggdrasilOperator};
/// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # enum Rule { program, expr, int, add, mul, sub, div, pow, fac, neg }
/// let pratt = PrattParser::new()
///     .op(YggdrasilOperator::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
///     .op(YggdrasilOperator::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
///     .op(YggdrasilOperator::infix(Rule::pow, Assoc::Right))
///     .op(YggdrasilOperator::prefix(Rule::neg))
///     .op(YggdrasilOperator::postfix(Rule::fac));
/// ```
///
/// To parse an expression, call the [`map_primary`], [`map_prefix`], [`map_postfix`],
/// [`map_infix`] and [`parse`] methods as follows:
///
/// ```
/// # use yggdrasil_rt::{PrattParser, TokenPair, YggdrasilRule};
/// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # enum Rule { program, expr, int, add, mul, sub, div, pow, fac, neg }
/// # impl YggdrasilRule for Rule {}
/// fn parse_expr(pairs: TokenPair<Rule>, pratt: &PrattParser<Rule>) -> i32 {
///     pratt
///         .map_primary(|primary| match primary.as_rule() {
///             Rule::int => primary.as_str().parse().unwrap(),
///             Rule::expr => parse_expr(primary.into_inner(), pratt), // from "(" ~ expr ~ ")"
///             _ => unreachable!(),
///         })
///         .map_prefix(|op, rhs| match op.as_rule() {
///             Rule::neg => -rhs,
///             _ => unreachable!(),
///         })
///         .map_postfix(|lhs, op| match op.as_rule() {
///             Rule::fac => (1..lhs + 1).product(),
///             _ => unreachable!(),
///         })
///         .map_infix(|lhs, op, rhs| match op.as_rule() {
///             Rule::add => lhs + rhs,
///             Rule::sub => lhs - rhs,
///             Rule::mul => lhs * rhs,
///             Rule::div => lhs / rhs,
///             Rule::pow => (1..rhs + 1).map(|_| lhs).product(),
///             _ => unreachable!(),
///         })
///         .parse(pairs)
/// }
/// ```
///
/// Note that [`map_prefix`], [`map_postfix`] and [`map_infix`] only need to be specified if the
/// grammar contains the corresponding operators.
///
/// [1]: https://en.wikipedia.org/wiki/Pratt_parser
/// [`Pairs`]: ../iterators/struct.Pairs.html
/// [`PrattParser`]: struct.PrattParser.html
/// [`map_primary`]: struct.PrattParser.html#method.map_primary
/// [`map_prefix`]: struct.PrattParserMap.html#method.map_prefix
/// [`map_postfix`]: struct.PrattParserMap.html#method.map_postfix
/// [`map_infix`]: struct.PrattParserMap.html#method.map_infix
/// [`parse`]: struct.PrattParserMap.html#method.parse
/// [`op`]: struct.PrattParserMap.html#method.op
pub struct PrattParser<R: YggdrasilRule> {
    precedence: Priority,
    operators: BTreeMap<R, (Affix, Priority)>,
    has_prefix: bool,
    has_postfix: bool,
    has_infix: bool,
}

impl<R: YggdrasilRule> Default for PrattParser<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: YggdrasilRule> PrattParser<R> {
    /// Instantiate a new `PrattParser`.
    pub fn new() -> Self {
        Self { precedence: STEP, operators: BTreeMap::new(), has_prefix: false, has_postfix: false, has_infix: false }
    }

    /// Add `op` to `PrattParser`.
    pub fn op(mut self, op: YggdrasilOperator<R>) -> Self {
        self.precedence += STEP;
        let mut iter = Some(op);
        while let Some(YggdrasilOperator { rule, affix, next }) = iter.take() {
            match affix {
                Affix::Prefix => self.has_prefix = true,
                Affix::Postfix => self.has_postfix = true,
                Affix::Infix(_) => self.has_infix = true,
            }
            self.operators.insert(rule, (affix, self.precedence));
            iter = next.map(|op| *op);
        }
        self
    }

    /// Maps primary expressions with a closure `primary`.
    pub fn map_primary<'pratt, 'a, 'i, X, T>(&'pratt self, primary: X) -> PrattParserMap<'pratt, 'a, 'i, R, X, T>
    where
        X: FnMut(TokenPair<'i, R>) -> T,
        R: 'pratt,
    {
        PrattParserMap { pratt: self, primary, prefix: None, postfix: None, infix: None, phantom: PhantomData }
    }
}

type FPrefix<'a, 'i, R, T> = Box<dyn FnMut(TokenPair<'i, R>, T) -> T + 'a>;
type FSuffix<'a, 'i, R, T> = Box<dyn FnMut(T, TokenPair<'i, R>) -> T + 'a>;
type FInfix<'a, 'i, R, T> = Box<dyn FnMut(T, TokenPair<'i, R>, T) -> T + 'a>;

/// Product of calling [`map_primary`] on [`PrattParser`], defines how expressions should
/// be mapped.
///
/// [`map_primary`]: struct.PrattParser.html#method.map_primary
/// [`PrattParser`]: struct.PrattParser.html
pub struct PrattParserMap<'pratt, 'a, 'i, R, F, T>
where
    R: YggdrasilRule,
    F: FnMut(TokenPair<'i, R>) -> T,
{
    pratt: &'pratt PrattParser<R>,
    primary: F,
    prefix: Option<FPrefix<'a, 'i, R, T>>,
    postfix: Option<FSuffix<'a, 'i, R, T>>,
    infix: Option<FInfix<'a, 'i, R, T>>,
    phantom: PhantomData<T>,
}

impl<'pratt, 'a, 'i, R, F, T> PrattParserMap<'pratt, 'a, 'i, R, F, T>
where
    R: YggdrasilRule + 'pratt,
    F: FnMut(TokenPair<'i, R>) -> T,
{
    /// Maps prefix operators with closure `prefix`.
    pub fn map_prefix<X>(mut self, prefix: X) -> Self
    where
        X: FnMut(TokenPair<'i, R>, T) -> T + 'a,
    {
        self.prefix = Some(Box::new(prefix));
        self
    }

    /// Maps postfix operators with closure `postfix`.
    pub fn map_postfix<X>(mut self, postfix: X) -> Self
    where
        X: FnMut(T, TokenPair<'i, R>) -> T + 'a,
    {
        self.postfix = Some(Box::new(postfix));
        self
    }

    /// Maps infix operators with a closure `infix`.
    pub fn map_infix<X>(mut self, infix: X) -> Self
    where
        X: FnMut(T, TokenPair<'i, R>, T) -> T + 'a,
    {
        self.infix = Some(Box::new(infix));
        self
    }

    /// The last method to call on the provided pairs to execute the Pratt
    /// parser (previously defined using [`map_primary`], [`map_prefix`], [`map_postfix`],
    /// and [`map_infix`] methods).
    ///
    /// [`map_primary`]: struct.PrattParser.html#method.map_primary
    /// [`map_prefix`]: struct.PrattParserMap.html#method.map_prefix
    /// [`map_postfix`]: struct.PrattParserMap.html#method.map_postfix
    /// [`map_infix`]: struct.PrattParserMap.html#method.map_infix
    pub fn parse<P: Iterator<Item = TokenPair<'i, R>>>(&mut self, pairs: P) -> T {
        self.expr(&mut pairs.peekable(), 0)
    }

    fn expr<P: Iterator<Item = TokenPair<'i, R>>>(&mut self, pairs: &mut Peekable<P>, rbp: Priority) -> T {
        let mut lhs = self.nud(pairs);
        while rbp < self.lbp(pairs) {
            lhs = self.led(pairs, lhs);
        }
        lhs
    }

    /// Null-Denotation
    ///
    /// "the action that should happen when the symbol is encountered
    ///  as start of an expression (most notably, prefix operators)
    fn nud<P: Iterator<Item = TokenPair<'i, R>>>(&mut self, pairs: &mut Peekable<P>) -> T {
        let pair = pairs.next().expect("Pratt parsing expects non-empty Pairs");
        match self.pratt.operators.get(&pair.get_rule()) {
            Some((Affix::Prefix, prec)) => {
                let rhs = self.expr(pairs, *prec - 1);
                match self.prefix.as_mut() {
                    Some(prefix) => prefix(pair, rhs),
                    None => panic!("Could not map {}, no `.map_prefix(...)` specified", pair),
                }
            }
            None => (self.primary)(pair),
            _ => panic!("Expected prefix or primary expression, found {}", pair),
        }
    }

    /// Left-Denotation
    ///
    /// "the action that should happen when the symbol is encountered
    /// after the start of an expression (most notably, infix and postfix operators)"
    fn led<P: Iterator<Item = TokenPair<'i, R>>>(&mut self, pairs: &mut Peekable<P>, lhs: T) -> T {
        let pair = pairs.next().unwrap();
        match self.pratt.operators.get(&pair.get_rule()) {
            Some((Affix::Infix(assoc), prec)) => {
                let rhs = match *assoc {
                    Associativity::Left => self.expr(pairs, *prec),
                    Associativity::Right => self.expr(pairs, *prec - 1),
                };
                match self.infix.as_mut() {
                    Some(infix) => infix(lhs, pair, rhs),
                    None => panic!("Could not map {}, no `.map_infix(...)` specified", pair),
                }
            }
            Some((Affix::Postfix, _)) => match self.postfix.as_mut() {
                Some(postfix) => postfix(lhs, pair),
                None => panic!("Could not map {}, no `.map_postfix(...)` specified", pair),
            },
            _ => panic!("Expected postfix or infix expression, found {}", pair),
        }
    }

    /// Left-Binding-Power
    ///
    /// "describes the symbol's precedence in infix form (most notably, operator precedence)"
    fn lbp<P: Iterator<Item = TokenPair<'i, R>>>(&mut self, pairs: &mut Peekable<P>) -> Priority {
        match pairs.peek() {
            Some(pair) => match self.pratt.operators.get(&pair.get_rule()) {
                Some((_, prec)) => *prec,
                None => panic!("Expected operator, found {}", pair),
            },
            None => 0,
        }
    }
}
