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

        FRAGMENT: $ => "fragment",

        GRAMMAR: $ => "grammar",

        Id: $ => /(r#)?[_\p{XID_Start}][_\p{XID_Continue}]*/,
    }
});
