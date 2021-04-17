// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use clap::{AppSettings, Clap, crate_version, crate_authors};
use crate::init::CommandInit;
use crate::test::CommandTest;

mod test;
mod init;
mod new;
mod cst;
mod ast;

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
    Test(CommandTest),
    Init(CommandInit),
}


fn main() {
    let opts: Options = Options::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Test(cmd) => {
            cmd.run()
        }
        SubCommand::Init(cmd) => {
            cmd.run()
        }
    }

    // more program logic goes here...
}