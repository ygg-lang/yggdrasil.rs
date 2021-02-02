module.exports = grammar({
    name: 'yg',

    extras: $ => [
        /\s/
    ],

    supertypes: $ => [

    ],
    inline: $ => [
        $._grammar_exts
    ],
    word: $ => $.Id,

    rules: {
        program: $ => "program",


        // GrammarStatement
        GrammarStatement: $ => seq(
            $.Grammar,
            field("id", $.Id),
            optional($._grammar_exts),
            optional($.Eos)
        ),
        _grammar_exts: $ => seq(
            "{",
            optional(interleave($.String, ",", true)),
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


        // Atomic
        Id: $ => /[_\p{XID_Start}][_\p{XID_Continue}]*/,
        Integer: $ => choice(
            optional($._sign),
            $.Unsigned
        ),
        Unsigned: $=> /0|[1-9][0-9]*/,
        _sign: $ => /[+-]/,

        String: $ => "String",


        Regex: $ => "/",





        Eos: $ => ";"
    }
});

function interleave(rule, sep, trailing) {
    if (trailing) {
        return seq(rule, repeat(seq(sep, rule)), optional(sep))
    }
    else {
        return seq(rule, repeat(seq(sep, rule)))
    }
}
