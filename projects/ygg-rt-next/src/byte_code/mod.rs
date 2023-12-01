use daachorse::CharwiseDoubleArrayAhoCorasick;

pub struct Language {
    pub indentation: bool,
    pub patterns: Vec<Bytecode>,
}

pub enum Bytecode {
    Any,
    Character(char),
    Range(CharacterRange),
    MatchString(StringPattern),
    MatchRule { rule: u32, jump: u32, length: u32 },
    MatchTag { tag: u32, jump: u32, length: u32 },
    LookAhead {},
    StartOfStream,
    EndOfStream,
}

pub struct CharacterRange {
    pub negative: bool,
    pub min: char,
    pub max: char,
}

pub struct StringPattern {
    pub insensitive: bool,
    pub string: CharwiseDoubleArrayAhoCorasick<String>,
}
