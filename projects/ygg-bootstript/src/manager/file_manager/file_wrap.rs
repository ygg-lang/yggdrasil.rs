use super::*;
use crate::{manager::HintItems, HINT_MANAGER};

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
    pub async fn parse_ygg(&mut self, url: Url, parser: &mut YGGBuilder) -> Result<&GrammarState> {
        match self {
            FileType::Grammar(g) => Ok(g),
            FileType::GrammarString(s) => {
                parser.update_by_text(s)?;
                let mut hints = HintItems::default();
                let (mut grammar, err) = parser.traverse()?.build_grammar(url.to_owned())?;
                hints += err;
                hints += grammar.optimize()?;
                *self = Self::Grammar(grammar);
                let grammar = match self {
                    FileType::Grammar(g) => Ok(g),
                    _ => Err(YGGError::Unreachable),
                }?;
                HINT_MANAGER.write().await.set(url, hints);

                Ok(grammar)
            }
            _ => Err(YGGError::language_error("Not a grammar file")),
        }
    }
}
