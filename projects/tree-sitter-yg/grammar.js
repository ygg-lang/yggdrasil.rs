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
    word: $ => $.id,

    rules: {
        program: $ => repeat(choice(
            $.grammar_statement,
            $.fragment_statement,
            $.assign_statement
        )),

        // TODO: Appears at the top, each at most once, can be disordered
        _top_level: $ => choice(
            $.grammar_statement,
            $.fragment_statement
        ),

        // GrammarStatement
        grammar_statement: $ => seq(
            $.grammar,
            field("id", $.id),
            optional($._grammar_exts),
            optional($.eos)
        ),
        _grammar_exts: $ => seq(
            "{",
            optional(interleave($.string, ",", 1)),
            "}"
        ),
        grammar: $ => "grammar!",


        // FragmentStatement
        fragment_statement: $ => seq(
            $.fragment,
            field("id", $.id),
            optional($.eos)
        ),
        fragment: $ => "fragment!",


        // IgnoresStatement
        ignore: $ => "ignore",


        assign_statement: $ => seq(
            field("id", $.id),
            field("eq", $.id),
            $._expression,
            optional($.eos)
        ),

        eq: $ => choice(
            "=",
            "_="
        ),

        _expression: $ => choice(
            $.id,
            $.string,
            $.unary_expression,
            $.binary_expression,
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
            binary_left(40, $._expression, token.immediate(/ +/), $._expression),
            binary_left(30, $._expression, "~", $._expression),
            binary_left(20, $._expression, "|", $._expression),
            binary_left(10, $._expression, "<-", $._expression),
        ),



        // Atomic
        id: $ => /[_\p{XID_Start}][_\p{XID_Continue}]*/,

        integer: $ => seq(
            optional($._sign),
            $.unsigned,
        ),
        unsigned: $ => token(/0|[1-9][0-9]*/),
        _sign: $ => /[+-]/,

        string: $ => choice(
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

function binary_left(p, lhs, op, rhs) {
    return prec.left(
        p, 
        seq(
            field("lhs", lhs), 
            field("op", op), 
            field("rhs", rhs)
        )
    )
}

function unary_prefix(p, lhs, op, rhs) {
    return seq(
        $._expression,
        field("op", '^'), 
    )
}

function unary_suffix(op, expr) {
    return seq(
        field("op", op), 
        field("expr", expr)
    )
}

