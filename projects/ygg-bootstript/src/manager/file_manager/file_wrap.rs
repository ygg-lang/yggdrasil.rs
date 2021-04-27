use crate::{HINT_MANAGER};
use crate::manager::HintItems;
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
    pub fn parse_ygg(&mut self, url: Url, parser: &mut YGGBuilder) -> Result<&GrammarState> {
        match self {
            FileType::Grammar(g) => Ok(g),
            FileType::GrammarString(s) => {
                parser.update_by_text(s)?;
                let mut hints = HintItems::default();
                let (mut grammar, err) = parser.traverse()?.build_grammar(url.to_owned())?;
                hints += err;


                *self = Self::Grammar(grammar);
                let grammar = match self {
                    FileType::Grammar(g) => g,
                    _ => unreachable!(),
                };
                HINT_MANAGER.write().await.set(url, hints);

                Ok(grammar)
            }
            _ => Err(YGGError::language_error("Not a grammar file")),
        }
    }
}
