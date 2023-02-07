// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate pest;

use pest::{
    error::YggdrasilError,
    iterators::{Pair, TokenTree},
    pratt_parser::{Assoc, Op, PrattParser},
    state, Either, State, YggdrasilParser, YggdrasilRule,
};

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum JsonRule {
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

type ParseIn<'i> = Box<State<'i, JsonRule>>;
type ParseOut<'i> = Either<Box<State<'i, JsonRule>>>;

struct CalculatorParser {}

impl YggdrasilRule for JsonRule {}

impl YggdrasilParser for CalculatorParser {
    type Rule = JsonRule;
    // false positive: pest uses `..` as a complete range (historically)
    #[allow(clippy::almost_complete_range)]
    fn parse(rule: JsonRule, input: &str) -> Result<TokenTree<JsonRule>, YggdrasilError<JsonRule>> {
        fn expression(state: ParseIn) -> ParseOut {
            state.rule(JsonRule::expression, |s| {
                s.sequence(|s| {
                    primary(s).and_then(|s| {
                        s.repeat(|s| {
                            s.sequence(|s| {
                                plus(s)
                                    .or_else(minus)
                                    .or_else(times)
                                    .or_else(divide)
                                    .or_else(modulus)
                                    .or_else(power)
                                    .and_then(primary)
                            })
                        })
                    })
                })
            })
        }

        fn primary(state: ParseIn) -> ParseOut {
            state.sequence(|s| s.match_string("(").and_then(expression).and_then(|s| s.match_string(")"))).or_else(number)
        }

        fn number(state: ParseIn) -> ParseOut {
            state.rule(JsonRule::number, |s| {
                s.sequence(|s| {
                    s.optional(|s| s.match_string("-")).and_then(|s| {
                        s.match_string("0").or_else(|s| {
                            s.sequence(|s| s.match_range('1'..'9').and_then(|s| s.repeat(|s| s.match_range('0'..'9'))))
                        })
                    })
                })
            })
        }

        fn plus(state: ParseIn) -> ParseOut {
            state.rule(JsonRule::plus, |s| s.match_string("+"))
        }

        fn minus(state: ParseIn) -> ParseOut {
            state.rule(JsonRule::minus, |s| s.match_string("-"))
        }

        fn times(state: ParseIn) -> ParseOut {
            state.rule(JsonRule::times, |s| s.match_string("*"))
        }

        fn divide(state: ParseIn) -> ParseOut {
            state.rule(JsonRule::divide, |s| s.match_string("/"))
        }

        fn modulus(state: ParseIn) -> ParseOut {
            state.rule(JsonRule::modulus, |s| s.match_string("%"))
        }

        fn power(state: ParseIn) -> ParseOut {
            state.rule(JsonRule::power, |s| s.match_string("^"))
        }

        state(input, |state| match rule {
            JsonRule::expression => expression(state),
            _ => unreachable!(),
        })
    }
}

#[allow(deprecated)]
enum PrattOrPrecClimber<'a> {
    Pratt(&'a PrattParser<JsonRule>),
    PrecClimber(&'a pest::prec_climber::PrecClimber<JsonRule>),
}

fn consume(pair: Pair<JsonRule>, pratt_or_climber: &PrattOrPrecClimber) -> i32 {
    let primary = |pair| consume(pair, pratt_or_climber);
    let infix = |lhs: i32, op: Pair<JsonRule>, rhs: i32| match op.as_rule() {
        JsonRule::plus => lhs + rhs,
        JsonRule::minus => lhs - rhs,
        JsonRule::times => lhs * rhs,
        JsonRule::divide => lhs / rhs,
        JsonRule::modulus => lhs % rhs,
        JsonRule::power => lhs.pow(rhs as u32),
        _ => unreachable!(),
    };

    #[allow(deprecated)]
    match (pair.as_rule(), pratt_or_climber) {
        (JsonRule::expression, PrattOrPrecClimber::Pratt(pratt)) => {
            pratt.map_primary(primary).map_infix(infix).parse(pair.into_inner())
        }
        (JsonRule::expression, PrattOrPrecClimber::PrecClimber(climber)) => climber.climb(pair.into_inner(), primary, infix),
        (JsonRule::number, _) => pair.as_str().parse().unwrap(),
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
        Operator::new(JsonRule::plus, Assoc::Left) | Operator::new(JsonRule::minus, Assoc::Left),
        Operator::new(JsonRule::times, Assoc::Left)
            | Operator::new(JsonRule::divide, Assoc::Left)
            | Operator::new(JsonRule::modulus, Assoc::Left),
        Operator::new(JsonRule::power, Assoc::Right),
    ]);

    let pairs = CalculatorParser::parse(JsonRule::expression, "-12+3*(4-9)^3^2/9%7381");
    assert_eq!(-1_525, consume(pairs.unwrap().next().unwrap(), &PrattOrPrecClimber::PrecClimber(&climber)));
}

#[test]
fn pratt_parse() {
    let pratt = PrattParser::new()
        .op(Op::infix(JsonRule::plus, Assoc::Left) | Op::infix(JsonRule::minus, Assoc::Left))
        .op(Op::infix(JsonRule::times, Assoc::Left)
            | Op::infix(JsonRule::divide, Assoc::Left)
            | Op::infix(JsonRule::modulus, Assoc::Left))
        .op(Op::infix(JsonRule::power, Assoc::Right));

    let pairs = CalculatorParser::parse(JsonRule::expression, "-12+3*(4-9)^3^2/9%7381");
    assert_eq!(-1_525, consume(pairs.unwrap().next().unwrap(), &PrattOrPrecClimber::Pratt(&pratt)));
}
