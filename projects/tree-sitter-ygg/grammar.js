module.exports = grammar({
    name: 'ygg',

    extras: $ => [
        $.NEWLINE,
        $.WHITESPACE,
        $.COMMENT,
    ],

    supertypes: $ => [],
    conflicts: $ => [],
    inline: $ => [],
    word: $ => $.id,

    rules: {
        program: $ => repeat(field("statement", $.statement)),

        statement: $ => choice(
            $.grammar_statement,
            $.fragment_statement,
            $.ignore_statement,
            $.assign_statement,
            $.comment_doc,
        ),

        // GrammarStatement
        grammar_statement: $ => seq(
            $.grammar,
            field("id", $.id),
            optional(choice(
                field("ext", $.string),
                seq("{", optional(join(field("ext", $.string), ",", 0)), "}"),
                seq("[", optional(join(field("ext", $.string), ",", 0)), "]")
            )),
            optional($.eos)
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
        ignore_statement: $ => seq(
            $.ignore,
            choice(
                field("item", $.id),
                seq("{", optional(join(field("item", $.id), ",", 0)), "}"),
                seq("[", optional(join(field("item", $.id), ",", 0)), "]"),
            ),
            optional($.eos)
        ),
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
            "@=",
            "^="
        ),
        // Unhide top level expression
        expression: $ => choice(
            seq("(", optional("|"), $.expression, ")"),
            $.data,
            $.unary_suffix,
            $.unary_prefix,
            $.concat_expression,
            $.choice_expression,
            $.field_expr,
        ),

        unary_prefix: $ => prec.left(200, choice(
            seq(field("prefix", $._prefix_op), field("base", $.expression)),
            // seq(field("prefix", "!"), field("expr", $.expression)),
        )),
        unary_suffix: $ => prec.right(210,
            seq(field("base", $.expression), field("suffix", $._suffix_op))
        ),

        _prefix_op: $ => choice(
            "^"
        ),
        _suffix_op: $ => choice(
            "?", "*", "+"
        ),

        concat_expression: $ => binary_left(30, $.expression, "~", $.expression),
        choice_expression: $ => binary_left(20, $.choice_tag, "|", $.choice_tag),
        field_expr: $ => binary_left(10, $.expression, "<-", $.expression),

        data: $ => choice(
            $.id,
            $.string,
            $.unsigned,
            $.macro_call,
            $.regex_long,
            $.regex_range,
            $.regex_set,
        ),

        choice_tag: $ => prec.left(100, seq(
            field("expression", $.expression),
            optional(seq(
                "#",
                field("tag", $.id),
                optional(field("mode", /[!^]/)),
                optional(seq(
                    ":",
                    field("ty", $.id)
                )),
            ))
        )),

        macro_call: $ => seq(
            "@",
            field("name", $.id),
            optional(seq(".", field("dot", $.id))),
            "(",
            join($.expression, ",", 0),
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
                field("text", /[^'\\]*(\\.[^'\\]*)*/),
                "'",
            ),
            seq(
                '"',
                field("text", /[^"\\]*(\\.[^"\\]*)*/),
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

        comment_doc: $ => token(seq(
            choice("//!", "//?", "//*"),
            /[^\n\r]*/
        )),

        COMMENT: $ => token(choice(
            seq('//', /[^\n\r]*/),
            seq(
                '/*',
                /[^*]*\*+([^/*][^*]*\*+)*/,
                '/'
            )
        )),

        NEWLINE: $ => /\r|\r|\n\r/,
        WHITESPACE: $ => /\s/,
    }
});

function join(rule, sep, trailing) {
    if (trailing > 0) {
        // must add trailing separator
        return seq(rule, repeat(seq(sep, rule)), sep)
    } else if (trailing < 0) {
        // disallow add trailing separator
        return seq(rule, repeat(seq(sep, rule)))
    } else {
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

