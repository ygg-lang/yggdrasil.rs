module.exports = grammar({
    name: 'yg',

    extras: $ => [
        $.NEWLINE,
        $.WHITESPACE,
    ],

    supertypes: $ => [

    ],
    conflict: $ => [

    ],
    inline: $ => [
        $._grammar_exts
    ],
    word: $ => $.id,

    rules: {
        program: $ => repeat(field("statement", $.statement)),

        statement: $ => choice(
            $.grammar_statement,
            $.fragment_statement,
            $.assign_statement
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
            optional(interleave(field("ext", $.string), ",", 1)),
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
        ignore: $ => "ignore!",


        assign_statement: $ => seq(
            field("id", $.id),
            field("eq", $.eq),
            optional("|"),
            field("rhs", $.expression),
            optional($.eos)
        ),

        eq: $ => choice(
            "=",
            "_=",
            "@="
        ),
        // Unhide top level expression
        expression: $ => choice(
            seq("(", $._expression, ")"),
            $.data,
            $.unary_suffix,
            $.unary_prefix,
            alias($._concat_expr, $.concat_expr),
            alias($._or_expr, $.or_expr),
            $.field_expr,
        ),
        // Hide recursive expression
        _expression: $ => choice(
            seq("(", $._expression, ")"),
            $.data,
            $.unary_suffix,
            $.unary_prefix,
            $._concat_expr,
            $._or_expr,
            $.field_expr,
        ),

        unary_prefix: $ => prec.left(200, choice(
            seq(field("prefix", $._prefix_op), field("base", $._expression)),
            // seq(field("prefix", "!"), field("expr", $._expression)),
        )),
        unary_suffix: $ => prec.right(210,
            seq(field("base", $._expression), field("suffix", $._suffix_op))
        ),

        _prefix_op: $ => choice(
            "^"
        ),
        _suffix_op: $ => choice(
            "?", "*", "+"
        ),

        _concat_expr: $ => prec.left(
            30,
            seq(
                field("base", $.expression),
                repeat1(seq(
                    field("op", "~"),
                    field("expr", $._expression),
                ))),
        ),
        _or_expr: $ => prec.left(
            20,
            seq(
                field("base", $.variant_tag),
                repeat1(seq(
                    field("op", "|"),
                    field("expr", $.variant_tag),
                )),
            )
        ),
        field_expr: $ => binary_left(10, $.id, "<-", $._expression),

        data: $ => choice(
            $.id,
            $.string,
            $.unsigned,
            $.macro_call,
            $.regex_long,
            $.regex_range,
            $.regex_set,
        ),

        variant_tag: $ => prec.left(100, seq(
            field("expression", $._expression),
            optional(seq(
                field("op", /[!_]?\#/),
                field("name", $.id),
            ))
        )),

        macro_call: $ => seq(
            "@",
            field("name", $.id),
            optional(seq(".", field("dot", $.id))),
            "(",
            interleave($._expression, ",", 1),
            ")"
        ),

        // Atomic
        id: $ => /[_\p{XID_Start}][\p{XID_Continue}]*/,

        integer: $ => seq(optional($._sign), $.unsigned),
        unsigned: $ => token(/0|[1-9][0-9]*/),
        _sign: $ => /[+-]/,

        string: $ => choice(
            seq(
                "'",
                /[^'\\]*(\\.[^'\\]*)*/,
                "'",
            ),
            seq(
                '"',
                /[^"\\]*(\\.[^"\\]*)*/,
                '"',
            )
        ),

        regex_long: $ => seq(
            "/",
            "/",
            optional(/i|g/)
        ),

        regex_range: $ => seq(
            field("is_neg", choice("[^", "[")),
            repeat($.regex_range_item),
            "]"
        ),
        regex_range_item: $ => choice(
            $.regex_set,
            $.regex_range_item_group,
            /[^\]]/
        ),
        regex_range_item_group: $ => binary_left(10, $.regex_range_item, "-", $.regex_range_item),

        regex_set: $ => seq(
            "\\p",
            "{",
            field("set", /[_0-9a-zA-Z]+/),
            "}"
        ),

        eos: $ => ";",

        NEWLINE: $ => /\r|\r|\n\r/,
        WHITESPACE: $ => /\s/,
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


function variadic_left(p, op, expr) {
    return prec.left(
        p,
        seq(
            field("expr", expr),
            repeat1(seq(
                field("op", op),
                field("expr", expr),
            )),
        )
    )
}

function ternary_left(p, lhs, op1, mid, op2, rhs) {
    return prec.left(
        p,
        seq(
            field("lhs", lhs),
            field("op1", op1),
            field("mid", mid),
            field("op2", op2),
            field("rhs", rhs),
        )
    )
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

function unary_prefix(p, op, base) {
    return prec.right(p, seq(
        field("prefix", op),
        field("expr", base),
    ))
}

function unary_suffix(p, expr, op) {
    return prec.right(p, seq(
        field("expr", base),
        field("suffix", op)
    ))
}

