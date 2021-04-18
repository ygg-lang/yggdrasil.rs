use super::*;
use std::{env, fs, path::Path, process::Command};

/// Create a new grammar project
#[derive(Clap)]
pub struct CommandAST {
    /// Set the folder name of the project
    #[clap(default_value = "*")]
    grammar_name: Vec<String>,
}

impl CommandAST {
    pub fn run(&self) -> Result<()> {
        let curr = env::current_dir()?;
        let dir_name = curr.join("projects").join(&grammar_name);
        println!("{:?}", dir_name);
        Ok(())
    }
}
