use super::*;

#[derive(Clone, Debug)]
pub enum FileType {
    GrammarString(String),
    Grammar(GrammarState),
    TypeString(String),
    Type(GrammarType),
}

impl FileType {
    pub fn parse_toml(&mut self) -> Result<FileType> {
        unimplemented!()
    }
    pub fn parse_ygg(&mut self, url: Url, parser: &mut YGGBuilder) -> Result<(&GrammarState, Vec<Diagnostic>)> {
        match self {
            FileType::Grammar(g) => Ok((g, vec![])),
            FileType::GrammarString(s) => {
                parser.update_by_text(s);
                let mut diag = vec![];
                let (mut grammar, err) = parser.traverse()?.build_grammar(Some(url))?;
                diag.extend(err);
                *self = Self::Grammar(grammar);
                let grammar = match self {
                    FileType::Grammar(g) => g,
                    _ => unreachable!(),
                };
                Ok((grammar, diag))
            }
            _ => Err(YGGError::language_error("Not a grammar file")),
        }
    }
}
