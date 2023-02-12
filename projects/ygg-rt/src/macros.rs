// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[doc(hidden)]
#[macro_export]
macro_rules! consumes_to {
    ( $_rules:ident, $tokens:expr, [] ) => ();
    ( $rules:ident, $tokens:expr, [ $name:ident ( $start:expr, $end:expr ) ] ) => {
        let expected = format!("expected Start {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $start);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::Start { rule, pos } => {
                assert!(
                    rule == $rules::$name && pos.offset() == $start,
                    "{} but found Start {{ rule: {:?}, pos: Position {{ {} }} }}",
                    expected, rule, pos.offset(),
                )
            },
            token => panic!("{} but found {:?}", expected, token)
        };

        let expected = format!("expected End {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $end);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::End { rule, pos } => {
                assert!(rule == $rules::$name && pos.offset() == $end,
                    "{} but found End {{ rule: {:?}, pos: Position {{ {} }} }}",
                    expected, rule, pos.offset(),
                );
            },
            token => panic!("{} but found {:?}", expected, token)
        };
    };
    ( $rules:ident, $tokens:expr, [ $name:ident ( $start:expr, $end:expr ),
                                    $( $names:ident $calls:tt ),* $(,)* ] ) => {

        let expected = format!("expected Start {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $start);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::Start { rule, pos } => {
                assert!(rule == $rules::$name && pos.offset() == $start,
                    "{} but found Start {{ rule: {:?}, pos: Position {{ {} }} }}",
                    expected, rule, pos.offset(),
                );
            },
            token => panic!("{} but found {:?}", expected, token)
        };

        let expected = format!("expected End {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $end);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::End { rule, pos } => {
                assert!(rule == $rules::$name && pos.offset() == $end,
                    "{} but found End {{ rule: {:?}, pos: Position {{ {} }} }}",
                    expected, rule, pos.offset(),
                );
            },
            token => panic!("{} but found {:?}", expected, token)
        };

        consumes_to!($rules, $tokens, [ $( $names $calls ),* ]);
    };
    ( $rules:ident, $tokens:expr, [ $name:ident ( $start:expr, $end:expr,
                                                  [ $( $names:ident $calls:tt ),* $(,)* ] ) ] ) => {
        let expected = format!("expected Start {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $start);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::Start { rule, pos } => {
                assert!(rule == $rules::$name && pos.offset() == $start,
                    "{} but found Start {{ rule: {:?}, pos: Position {{ {} }} }}",
                    expected, rule, pos.offset(),
                );
            },
            token => panic!("{} but found {:?}", expected, token)
        };

        consumes_to!($rules, $tokens, [ $( $names $calls ),* ]);

        let expected = format!("expected End {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $end);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::End { rule, pos } => {
                assert!(rule == $rules::$name && pos.offset() == $end,
                    "{} but found End {{ rule: {:?}, pos: Position {{ {} }} }}",
                    expected, rule, pos.offset(),
                );
            },
            token => panic!("{} but found {:?}", expected, token)
        };
    };
    ( $rules:ident, $tokens:expr, [ $name:ident ( $start:expr, $end:expr,
                                                  [ $( $nested_names:ident $nested_calls:tt ),*
                                                  $(,)* ] ),
                                    $( $names:ident $calls:tt ),* ] ) => {

        let expected = format!("expected Start {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $start);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::Start { rule, pos } => {
                assert!(rule == $rules::$name && pos.offset() == $start,
                    "{} but found Start {{ rule: {:?}, pos: Position {{ {} }} }}",
                    expected, rule, pos.offset(),
                );
            },
            token => panic!("{} but found {:?}", expected, token)
        };

        consumes_to!($rules, $tokens, [ $( $nested_names $nested_calls ),* ]);

        let expected = format!("expected End {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $end);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::End { rule, pos } => {
                assert!(rule == $rules::$name && pos.offset() == $end,
                    "{} but found End {{ rule: {:?}, pos: Position {{ {} }} }}",
                    expected, rule, pos.offset(),
                );
            },
            token => panic!("{} but found {:?}", expected, token)
        };

        consumes_to!($rules, $tokens, [ $( $names $calls ),* ]);
    };
}

/// Testing tool that compares produced tokens.
///
/// This macro takes several arguments:
///
/// * `parser` - name of the data structure implementing `Parser`
/// * `input` - input to be tested against
/// * `rule` - `Rule` which will be run
/// * `tokens` - token pairs of the form `name(start_pos, end_pos, [nested_child_tokens])`
///
/// *Note:* `start_pos` and `end_pos` are byte positions.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate pest;
/// # use pest::YggdrasilParser;
/// # use pest::error::YggdrasilError;
/// # use pest::iterators::TokenTree;
/// # fn main() {
/// # #[allow(non_camel_case_types)]
/// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # enum Rule {
/// #     a,
/// #     b,
/// #     c
/// # }
/// #
/// # struct AbcParser;
/// #
/// # impl YggdrasilParser<Rule> for AbcParser {
/// #     fn parse<'i>(_: Rule, input: &'i str) -> Result<TokenTree<'i, Rule>, YggdrasilError<Rule>> {
/// #         pest::state(input, |state| {
/// #             state.rule(Rule::a, |state| {
/// #                 state.skip(1).unwrap().rule(Rule::b, |state| {
/// #                     state.skip(1)
/// #                 }).unwrap().skip(1)
/// #             }).and_then(|state| {
/// #                 state.skip(1).unwrap().rule(Rule::c, |state| {
/// #                     state.skip(1)
/// #                 })
/// #             })
/// #         })
/// #     }
/// # }
/// parses_to! {
///     parser: AbcParser,
///     input:  "abcde",
///     rule:   Rule::a,
///     tokens: [
///         a(0, 3, [
///             b(1, 2)
///         ]),
///         c(4, 5)
///     ]
/// };
/// # }
/// ```
#[macro_export]
macro_rules! parses_to {
    ( parser: $parser:ident, input: $string:expr, rule: $rules:tt :: $rule:tt,
      tokens: [ $( $names:ident $calls:tt ),* $(,)* ] ) => {

        #[allow(unused_mut)]
        {
            use $crate::YggdrasilParser;

            let mut tokens = $parser::parse($rules::$rule, $string).unwrap().tokens();

            consumes_to!($rules, &mut tokens, [ $( $names $calls ),* ]);

            let rest: Vec<_> = tokens.collect();

            match rest.len() {
                0 => (),
                2 => {
                    let (first, second) = (&rest[0], &rest[1]);

                    match (first, second) {
                        (
                            &$crate::Token::Start { rule: ref first_rule, .. },
                            &$crate::Token::End { rule: ref second_rule, .. }
                        ) => {
                            assert!(
                                format!("{:?}", first_rule) == "EOI",
                                "expected end of input, but found {:?}", rest
                            );
                            assert!(
                                format!("{:?}", second_rule) == "EOI",
                                "expected end of input, but found {:?}", rest
                            );
                        }
                        _ => panic!("expected end of input, but found {:?}", rest)
                    }
                }
                _ => panic!("expected end of input, but found {:?}", rest)
            };
        }
    };
}

/// Testing tool that compares produced errors.
///
/// This macro takes several arguments:
///
/// * `parser` - name of the data structure implementing `Parser`
/// * `input` - input to be tested against
/// * `rule` - `Rule` which will be run
/// * `positives` - positive `Rule` attempts that failed
/// * `negatives` - negative `Rule` attempts that failed
/// * `pos` - byte position of failure
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate pest;
/// # use pest::YggdrasilParser;
/// # use pest::error::YggdrasilError;
/// # use pest::iterators::TokenTree;
/// # fn main() {
/// # #[allow(non_camel_case_types)]
/// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # enum Rule {
/// #     a,
/// #     b,
/// #     c
/// # }
/// #
/// # struct AbcParser;
/// #
/// # impl YggdrasilParser<Rule> for AbcParser {
/// #     fn parse<'i>(_: Rule, input: &'i str) -> Result<TokenTree<'i, Rule>, YggdrasilError<Rule>> {
/// #         pest::state(input, |state| {
/// #             state.rule(Rule::a, |state| {
/// #                 state.skip(1).unwrap().rule(Rule::b, |s| {
/// #                     s.skip(1)
/// #                 }).unwrap().skip(1)
/// #             }).and_then(|state| {
/// #                 state.skip(1).unwrap().rule(Rule::c, |s| {
/// #                     s.match_string("e")
/// #                 })
/// #             })
/// #         })
/// #     }
/// # }
/// fails_with! {
///     parser: AbcParser,
///     input: "abcdf",
///     rule: Rule::a,
///     positives: vec![Rule::c],
///     negatives: vec![],
///     pos: 4
/// };
/// # }
/// ```
#[macro_export]
macro_rules! fails_with {
    ( parser: $parser:ident, input: $string:expr, rule: $rules:tt :: $rule:tt,
      positives: $positives:expr, negatives: $negatives:expr, pos: $pos:expr ) => {
        #[allow(unused_mut)]
        {
            use $crate::YggdrasilParser;

            let error = $parser::parse($rules::$rule, $string).unwrap_err();

            match error.variant {
                $crate::errors::ErrorKind::ParsingError { positives, negatives } => {
                    assert_eq!(positives, $positives, "positives");
                    assert_eq!(negatives, $negatives, "negatives");
                }
                _ => unreachable!(),
            };

            match error.location {
                $crate::errors::InputLocation::Pos(pos) => assert_eq!(pos, $pos, "pos"),
                _ => unreachable!(),
            }
        }
    };
}

#[cfg(test)]
pub mod tests {
    use super::super::{errors::YggdrasilError, iterators::TokenTree, state, YggdrasilParser};
    use crate::YggdrasilRule;
    use alloc::{format, vec, vec::Vec};

    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum TestRule {
        a,
        b,
        c,
        d,
    }

    impl YggdrasilRule for TestRule {}

    pub struct AbcParser;

    impl YggdrasilParser for AbcParser {
        type Rule = TestRule;
        fn parse(_: TestRule, input: &str) -> Result<TokenTree<'_, TestRule>, YggdrasilError<TestRule>> {
            state(input, |state| {
                state
                    .rule(TestRule::a, |s| s.skip(1).unwrap().rule(TestRule::b, |s| s.skip(1)).unwrap().skip(1))
                    .and_then(|s| s.skip(1).unwrap().rule(TestRule::c, |s| s.match_string("e")))
                    .and_then(|s| s.optional(|s| s.rule(TestRule::d, |s| s.match_string("fgh"))))
            })
        }
    }

    #[test]
    fn parses_to() {
        parses_to! {
            parser: AbcParser,
            input: "abcde",
            rule: TestRule::a,
            tokens: [
                a(0, 3, [
                    b(1, 2),
                ]),
                c(4, 5)
            ]
        };
    }

    #[test]
    #[should_panic]
    fn missing_end() {
        parses_to! {
            parser: AbcParser,
            input: "abcde",
            rule: TestRule::a,
            tokens: [
                a(0, 3, [
                    b(1, 2)
                ])
            ]
        };
    }

    #[test]
    #[should_panic]
    fn empty() {
        parses_to! {
            parser: AbcParser,
            input: "abcde",
            rule: TestRule::a,
            tokens: []
        };
    }

    #[test]
    fn fails_with() {
        fails_with! {
            parser: AbcParser,
            input: "abcdf",
            rule: TestRule::a,
            positives: vec![TestRule::c],
            negatives: vec![],
            pos: 4
        };
    }

    #[test]
    #[should_panic]
    fn wrong_positives() {
        fails_with! {
            parser: AbcParser,
            input: "abcdf",
            rule: TestRule::a,
            positives: vec![TestRule::a],
            negatives: vec![],
            pos: 4
        };
    }

    #[test]
    #[should_panic]
    fn wrong_negatives() {
        fails_with! {
            parser: AbcParser,
            input: "abcdf",
            rule: TestRule::a,
            positives: vec![TestRule::c],
            negatives: vec![TestRule::c],
            pos: 4
        };
    }

    #[test]
    #[should_panic]
    fn wrong_pos() {
        fails_with! {
            parser: AbcParser,
            input: "abcdf",
            rule: TestRule::a,
            positives: vec![TestRule::c],
            negatives: vec![],
            pos: 3
        };
    }
}
