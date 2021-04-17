use super::*;

/// A subcommand for controlling testing
#[derive(Clap)]
pub struct CommandTest {
    /// Print debug info
    #[clap(short)]
    debug: bool,

    input: String,
}


impl CommandTest {
    pub fn run(&self) {
        if self.debug {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
}