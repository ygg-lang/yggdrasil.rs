use super::*;
use crate::{cache::GrammarCache, GaiaSystem};
use std::{collections::BTreeMap, env::current_dir, fs::read_to_string};
use wax::{CandidatePath, Glob, LinkBehavior, Pattern, WalkEntry, WalkError};
use yggdrasil_error::{Failure, Success, Validation};
use yggdrasil_shared::{parse_grammar, parse_grammar_raw, GrammarInfo};

/// Create a new grammar project
#[derive(Parser)]
pub struct CommandBuild {
    /// Set the folder name of the project
    #[clap(default_value = "*")]
    include: Vec<String>,
}

impl CommandBuild {
    pub fn run(&self, config: &YccConfig) -> Result<()> {
        let glob = Glob::new("grammars/*.ygg")?;
        for entry in glob.walk_with_behavior(current_dir()?, LinkBehavior::ReadTarget).not([".test"])? {
            match self.insert_item(entry) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Ok(())
    }
    fn insert_item(&self, item: std::result::Result<WalkEntry, WalkError>) -> Result<()> {
        let item = item?;
        let cache = GaiaSystem::default();
        if item.path().is_file() {
            let text = read_to_string(item.path())?;
            let grammar = parse_grammar_raw(&text)?;
        }
        Ok(())
    }

    pub fn build_one(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }
    pub fn build_all(&self) -> Result<()> {
        unimplemented!()
    }
}
