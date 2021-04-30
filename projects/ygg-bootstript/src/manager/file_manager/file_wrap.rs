use crate::HINT_MANAGER;
use super::*;
use crate::manager::global_parser::PARSER_MANAGER;

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
    pub async fn parse_ygg(&mut self, url: Url) -> Result<GrammarState> {
        let mut parser = PARSER_MANAGER.write().await;
        match self {
            FileType::Grammar(g) => {
                // TODO: no clone
                Ok(g.clone())
            }
            FileType::GrammarString(s) => {
                parser.update_by_text(s)?;
                let mut hints = HintItems::default();
                let (mut grammar, err) = parser.traverse()?.build_grammar(url.to_owned())?;
                hints += err;
                hints += grammar.optimize().await?;
                hints += grammar.report_meta();
                // FIXME: Use ref
                // *self = Self::Grammar(grammar);
                // let grammar = match self {
                //     FileType::Grammar(g) => Ok(g),
                //     _ => Err(YGGError::Unreachable),
                // }?;
                *self = Self::Grammar(grammar.to_owned());
                // FIXME: dead lock
                HINT_MANAGER.set(url, hints);
                Ok(grammar)
            }
            _ => Err(YGGError::language_error("Not a grammar file")),
        }
    }
}
