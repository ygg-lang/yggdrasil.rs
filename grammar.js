module.exports = grammar({
    name: 'yg',

    extras: $ => [
        /\s/
    ],

    supertypes: $ => [

    ],
    word: $ => $.Id,

    rules: {
        program: $ => "program",

        // GrammarStatement
        Grammar: $ => "grammar!",

        // FragmentStatement
        FragmentStatement : $ => seq(
            $.Fragment,
            field("id", $.Id)
        ),
        Fragment: $ => "fragment!",



        Id: $ => /[_\p{XID_Start}][_\p{XID_Continue}]*/,
    }
});
