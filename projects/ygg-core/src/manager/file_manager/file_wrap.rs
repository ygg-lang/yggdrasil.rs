
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
    pub async fn parse_ygg(&mut self, url: Url) -> Result<GrammarState> {
        let mut parser = PARSER_MANAGER.write().await;
        match self {
            FileType::Grammar(g) => {
                // TODO: no clone
                Ok(g.clone())
            }
            FileType::GrammarString(s) => {
                let mut hints = HintItems::default();
                let (program, err) = parser.parse_program(s)?;
                let file = FilePosition::new(s, &url);
                parse_error_to_hints(&file, err, &mut hints);
                let (mut grammar, err) = program.translate(&file)?;
                hints += err;
                hints += grammar.optimize().await?;
                hints += grammar.report_meta();
                *self = Self::Grammar(grammar.to_owned());
                HINT_MANAGER.set(url, hints);
                Ok(grammar)
            }
            _ => Err(Error::language_error("Not a grammar file")),
        }
    }
}

fn parse_error_to_hints(file: &FilePosition, es: Vec<Error>, hint: &mut HintItems) {


    for e in es {
        let diag = match e {
            Error::ParsingError { error, range } => {
                Diagnostic {
                    range: file.get_lsp_range(range),
                    severity: None,
                    code: None,
                    code_description: None,
                    source: None,
                    message: error,
                    related_information: None,
                    tags: None,
                    data: None
                }
            }
            _ => unreachable!()
        };
        hint.diagnostic.push(diag)
    }
}