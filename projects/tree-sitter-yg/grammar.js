module.exports = grammar({
    name: 'yg',

    extras: $ => [
        $.whitespace
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
            $.FragmentStatement,
            $.AssignStatement
        )),

        // TODO: Appears at the top, each at most once, can be disordered
        _top_level: $ => choice(
            $.GrammarStatement,
            $.FragmentStatement
        ),

        // GrammarStatement
        grammar_statement: $ => seq(
            $.Grammar,
            field("id", $.Id),
            optional($._grammar_exts),
            field("eos", optional($.eos))
        ),
        _grammar_exts: $ => seq(
            "{",
            optional(interleave($.String, ",", 1)),
            "}"
        ),
        grammar: $ => "grammar!",




        // FragmentStatement
        fragment_statement: $ => seq(
            $.Fragment,
            field("id", $.Id),
            optional($.Eos)
        ),
        fragment: $ => "fragment!",


        // IgnoresStatement
        ignore: $ => "ignore",


        assign_statement: $ => seq(
            field("id", $.Id),
            "=",
            $._expression,
            optional($.Eos)
        ),

        _expression: $ => choice(
            $.Id,
            $.String,
            $.UnaryExpression,
            $.BinaryExpression,
            // ...
        ),

        unary_expression: $ => prec(2, choice(
            seq('^', $._expression),
            seq('!', $._expression),
            // ...
        )),

        binary_expression: $ => choice(
            // 空格连接禁止换行, 否则有可能会把下面几行的函数给吃进去
            // prec.left(90, token.immediate(seq($._expression, repeat1("\w"), $._expression))),
            // name <- a ~ b | name ~ c
            // <- 是长程符号
            // ~ 等于空格, 是短程符号
            // 因此上式等价于:
            // name <- ((a ~ b) | (name ~ c))
            prec.left(30, seq(field("lhs", $._expression), '~', field("rhs", $._expression))),
            prec.left(20, seq(field("lhs", $._expression), '|', field("rhs", $._expression))),
            prec.left(10, seq(field("lhs", $._expression), '<-', field("rhs", $._expression))),
        ),




        // Atomic
        id: $ => /[_\p{XID_Start}][_\p{XID_Continue}]*/,

        integer: $ => seq(
            optional($._sign),
            $.Unsigned,
        ),
        unsigned: $ => token(/0|[1-9][0-9]*/),
        _sign: $ => /[+-]/,

        String: $ => choice(
            seq(
                "'",
                /[a-zA-Z]*/,
                "'",
            ),
            seq(
                '"',
                /[a-zA-Z]*/,
                '"',
            )
        ),

        Regex: $ => "/",

        eos: $ => ";",

        whitespace: $ => /\s/,
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
