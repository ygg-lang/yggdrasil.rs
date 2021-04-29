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
    pub fn parse_ygg(&mut self, url: Url, parser: &mut YGGBuilder) -> ParseResult<&GrammarState> {
        match self {
            FileType::Grammar(g) => Ok((g, None)),
            FileType::GrammarString(s) => {
                parser.update_by_text(s)?;
                let mut hints = HintItems::default();
                let (mut grammar, err) = parser.traverse()?.build_grammar(url.to_owned())?;
                hints += err;
                hints += grammar.optimize()?;
                hints += grammar.report_meta();
                *self = Self::Grammar(grammar);
                let grammar = match self {
                    FileType::Grammar(g) => Ok(g),
                    _ => Err(YGGError::Unreachable),
                }?;
                // FIXME: dead lock
                // HINT_MANAGER.write().await.set(url, hints);
                Ok((grammar, Some(hints)))
            }
            _ => Err(YGGError::language_error("Not a grammar file")),
        }
    }
}
