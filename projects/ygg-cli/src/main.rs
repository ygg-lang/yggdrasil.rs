#![feature(path_try_exists)]

use anyhow::Result;
use clap::{AppSettings, Clap, crate_authors, crate_version};
use self::subs::*;
use self::cache::*;

mod subs;
mod cache;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = crate_version ! (), author = crate_authors ! ("\n"))]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "ygg.toml")]
    config: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    /// Subcommands
    #[clap(subcommand)]
    subcmd: SubCommand,
}



fn main() -> Result<()> {
    let opts: Options = Options::parse();
    match opts.subcmd {
        SubCommand::Init(cmd) => cmd.run(),
        SubCommand::New(cmd) => cmd.run(),
        SubCommand::CST(cmd) => cmd.run(),
        SubCommand::AST(cmd) => cmd.run(),
        SubCommand::Test(cmd) => cmd.run(),
        SubCommand::Pub(cmd) => cmd.run(),
    }
}
