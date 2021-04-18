#![feature(path_try_exists)]

// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use crate::{init::CommandInit, test::CommandTest};
use anyhow::Result;
use clap::{crate_authors, crate_version, AppSettings, Clap};
use crate::ast::CommandAST;
use crate::cst::CommandCST;
use crate::new::CommandNew;
use crate::publish::CommandPub;

mod subs;

mod ast;
mod cst;
mod init;
mod new;
mod test;
mod publish;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = crate_version ! (), author = crate_authors ! ("\n"))]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Init(CommandInit),
    New(CommandNew),
    CST(CommandCST),
    AST(CommandAST),
    Test(CommandTest),
    Pub(CommandPub)
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
