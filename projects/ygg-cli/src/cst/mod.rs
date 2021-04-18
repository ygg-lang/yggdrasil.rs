use super::*;
use std::{env, fs, path::Path, process::Command};

/// Create a new grammar project
#[derive(Clap)]
pub struct CommandCST {
    /// Set the folder name of the project
    #[clap(default_value = "*")]
    grammar_name: Vec<String>,
    /// [unimplemented] Custom generated path
    #[clap(short, long)]
    path: Option<String>,
}

impl CommandCST {
    pub fn run(&self) -> Result<()> {
        println!("{:?}", dir_name);
        Ok(())
    }

    pub fn target_dir(&self, name: &str) {
        let curr = env::current_dir()?;
        let dir_name = curr.join("projects").join(&grammar_name);
    }

    pub fn build_one(&self, s: &str) {

    }
}
