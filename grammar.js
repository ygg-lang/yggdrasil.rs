module.exports = grammar({
    name: 'yg',

    extras: $ => [
        /\s/
    ],

    supertypes: $ => [

    ],
    inline: $ => [
        $._grammar_exts, //   $._sign
    ],
    word: $ => $.Id,

    rules: {
        program: $ => repeat(choice(
            $.GrammarStatement,
            $.FragmentStatement
        )),

        // TODO: Appears at the top, each at most once, can be disordered
        _top_level: $ => choice(
            $.GrammarStatement,
            $.FragmentStatement
        ),

        // GrammarStatement
        GrammarStatement: $ => seq(
            $.Grammar,
            field("id", $.Id),
            optional($._grammar_exts),
            optional($.Eos)
        ),
        _grammar_exts: $ => seq(
            "{",
            optional(interleave($.String, ",", 1)),
            "}"
        ),
        Grammar: $ => "grammar!",


        // FragmentStatement
        FragmentStatement: $ => seq(
            $.Fragment,
            field("id", $.Id),
            optional($.Eos)
        ),
        Fragment: $ => "fragment!",


        // IgnoresStatement
        Ignore: $ => "ignore",

        _expression: $ => choice(
            $.Id,
            $.unary_expression,
            $.binary_expression,
            // ...
        ),

        unary_expression: $ => prec(2, choice(
            seq('-', $._expression),
            seq('!', $._expression),
            // ...
        )),

        binary_expression: $ => choice(
            // name <- a ~ b | name ~ c
            // ((name <- a) ~ b) | (name ~ c)
            prec.left(30, seq($._expression, '<-', $._expression)),
            prec.left(20, seq($._expression, '~', $._expression)),
            prec.left(10, seq($._expression, '|', $._expression)),
        ),




        // Atomic
        Id: $ => /[_\p{XID_Start}][_\p{XID_Continue}]*/,

        Integer: $ => seq(
            optional($._sign),
            $.Unsigned,
        ),
        Unsigned: $ => /0|[1-9][0-9]*/,
        _sign: $ => /[+-]/,

        String: $ => "String",


        Regex: $ => "/",

        Eos: $ => ";"
    }
});

function interleave(rule, sep, trailing) {
    if (trailing > 0) {
        // must add trailing separator
        return seq(rule, repeat(seq(sep, rule)), sep)
    }
    else if (trailing < 0) {
        // disallow add trailing separator
        return seq(rule, repeat(seq(sep, rule)))
    }
    else {
        // trailing separator is optional
        return seq(rule, repeat(seq(sep, rule)), optional(sep))
    }
}
