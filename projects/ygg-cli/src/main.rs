#![feature(path_try_exists)]
#![feature(once_cell)]
#![feature(fs_try_exists)]

use self::{cache::*, subs::*};
use anyhow::Result;
use clap::Parser;

mod cache;
mod subs;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser)]
// #[clap(version = crate_version!(), author = crate_authors!("\n"))]
// #[clap(setting = AppSettings::ColoredHelp)]
pub struct App {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "yggdrasil.toml")]
    config: String,
    // A level of verbosity, and can be used multiple times
    // #[clap(short, long, parse(from_occurrences))]
    // verbose: i32,
    /// Subcommands
    #[clap(subcommand)]
    subs: SubCommand,
}

fn main() -> Result<()> {
    let opts: App = App::parse();
    opts.subs.run(&opts)
}
