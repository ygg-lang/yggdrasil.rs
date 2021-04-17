use std::path::Path;
use std::process::Command;
use super::*;

/// Initialize the project
#[derive(Clap)]
pub struct CommandInit {
    /// [unimplemented] Overwrite the existing project
    #[clap(short, long)]
    force: bool,
    /// [unimplemented] Overwrite the existing project
    dir_name: Option<String>,
}


impl CommandInit {
    pub fn run(&self) {
        if self.force {

        }

        let out = Command::new("git")
            .args(["clone", "https://github.com/ygg-lang/project-yggdrasil-template.git"])
            .spawn()
            .expect("failed to execute process");
        println!("{:?}", out)
    }
}