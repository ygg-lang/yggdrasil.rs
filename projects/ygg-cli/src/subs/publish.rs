use super::*;

/// Create a new grammar project
#[derive(Parser)]
pub struct CommandPub {
    /// Set the folder name of the project
    grammar_name: String,
    /// Overwrite the existing grammar project
    #[clap(short, long)]
    force: bool,
    /// Whether to generate boilerplate grammar
    #[clap(short, long)]
    placeholder: bool,
}

impl CommandPub {
    pub fn run(&self) -> Result<()> {
        let curr = env::current_dir()?;
        let dir_name = curr.join("projects").join(&self.grammar_name);
        if self.force {
            // Command::new("rm").args(["-rf", &dir_name]).spawn()?.wait()?;
            if let Ok(true) = fs::try_exists(&dir_name) {
                fs::remove_dir_all(&dir_name)?
            };
        }
        println!("ygg ast_mode does not been unimplemented!");
        if self.placeholder {}
        Command::new("cd").args([&dir_name]).spawn()?.wait()?;
        Ok(())
    }
}
