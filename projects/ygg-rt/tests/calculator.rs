// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use yggdrasil_rt::{
    parses_to, pratt_parser::PrattParser, prec_climber::PrecClimber, Either, State, TokenTree, YggdrasilError,
    YggdrasilLanguage, YggdrasilRule,
};

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CalculatorRule {
    expression,
    primary,
    number,
    plus,
    minus,
    times,
    divide,
    modulus,
    power,
}

type ParseIn<'i> = Box<State<'i, CalculatorRule>>;
type ParseOut<'i> = Either<Box<State<'i, CalculatorRule>>>;

struct CalculatorParser {}

impl YggdrasilRule for CalculatorRule {
    fn all_rules() -> &'static [Self]
    where
        Self: Sized,
    {
        todo!()
    }

    fn is_ignore(&self) -> bool {
        todo!()
    }
}

impl YggdrasilLanguage for CalculatorParser {
    type Rule = CalculatorRule;

    fn parse_cst(input: &str, rule: Self::Rule) -> Result<TokenTree<Self::Rule>, YggdrasilError<Self::Rule>> {
        todo!()
    }
}

#[allow(deprecated)]
enum PrattOrPrecClimber<'a> {
    Pratt(&'a PrattParser<CalculatorRule>),
    PrecClimber(&'a PrecClimber<CalculatorRule>),
}

fn consume(pair: Pair<CalculatorRule>, pratt_or_climber: &PrattOrPrecClimber) -> i32 {
    let primary = |pair| consume(pair, pratt_or_climber);
    let infix = |lhs: i32, op: Pair<CalculatorRule>, rhs: i32| match op.as_rule() {
        CalculatorRule::plus => lhs + rhs,
        CalculatorRule::minus => lhs - rhs,
        CalculatorRule::times => lhs * rhs,
        CalculatorRule::divide => lhs / rhs,
        CalculatorRule::modulus => lhs % rhs,
        CalculatorRule::power => lhs.pow(rhs as u32),
        _ => unreachable!(),
    };

    #[allow(deprecated)]
    match (pair.as_rule(), pratt_or_climber) {
        (CalculatorRule::expression, PrattOrPrecClimber::Pratt(pratt)) => {
            pratt.map_primary(primary).map_infix(infix).parse(pair.into_inner())
        }
        (CalculatorRule::expression, PrattOrPrecClimber::PrecClimber(climber)) => {
            climber.climb(pair.into_inner(), primary, infix)
        }
        (CalculatorRule::number, _) => pair.as_str().parse().unwrap(),
        _ => unreachable!(),
    }
}

#[test]
fn number() {
    parses_to! {
        parser: CalculatorParser,
        input: "-12",
        rule: JsonRule::expression,
        tokens: [
            expression(0, 3, [
                number(0, 3)
            ])
        ]
    };
}

#[test]
fn parens() {
    parses_to! {
        parser: CalculatorParser,
        input: "((-12))",
        rule: JsonRule::expression,
        tokens: [
            expression(0, 7, [
                expression(1, 6, [
                    expression(2, 5, [
                        number(2, 5)
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn expression() {
    parses_to! {
        parser: CalculatorParser,
        input: "-12+3*(4-9)^7^2",
        rule: JsonRule::expression,
        tokens: [
            expression(0, 15, [
                number(0, 3),
                plus(3, 4),
                number(4, 5),
                times(5, 6),
                expression(7, 10, [
                    number(7, 8),
                    minus(8, 9),
                    number(9, 10)
                ]),
                power(11, 12),
                number(12, 13),
                power(13, 14),
                number(14, 15)
            ])
        ]
    };
}

#[test]
#[allow(deprecated)]
fn prec_climb() {
    use pest::prec_climber::{Assoc, Operator, PrecClimber};
    let climber = PrecClimber::new(vec![
        Operator::new(CalculatorRule::plus, Assoc::Left) | Operator::new(CalculatorRule::minus, Assoc::Left),
        Operator::new(CalculatorRule::times, Assoc::Left)
            | Operator::new(CalculatorRule::divide, Assoc::Left)
            | Operator::new(CalculatorRule::modulus, Assoc::Left),
        Operator::new(CalculatorRule::power, Assoc::Right),
    ]);

    let pairs = CalculatorParser::parse(CalculatorRule::expression, "-12+3*(4-9)^3^2/9%7381");
    assert_eq!(-1_525, consume(pairs.unwrap().next().unwrap(), &PrattOrPrecClimber::PrecClimber(&climber)));
}

#[test]
fn pratt_parse() {
    let pratt = PrattParser::new()
        .op(Op::infix(CalculatorRule::plus, Assoc::Left) | Op::infix(CalculatorRule::minus, Assoc::Left))
        .op(Op::infix(CalculatorRule::times, Assoc::Left)
            | Op::infix(CalculatorRule::divide, Assoc::Left)
            | Op::infix(CalculatorRule::modulus, Assoc::Left))
        .op(Op::infix(CalculatorRule::power, Assoc::Right));

    let pairs = CalculatorParser::parse(CalculatorRule::expression, "-12+3*(4-9)^3^2/9%7381");
    assert_eq!(-1_525, consume(pairs.unwrap().next().unwrap(), &PrattOrPrecClimber::Pratt(&pratt)));
}
