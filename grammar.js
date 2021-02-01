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

        String: $ => "String",

        Id: $ => /[_\p{XID_Start}][_\p{XID_Continue}]*/,
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
