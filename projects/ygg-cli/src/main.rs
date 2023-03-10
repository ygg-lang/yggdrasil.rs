#![feature(path_try_exists)]
#![feature(once_cell)]
#![feature(fs_try_exists)]
#![feature(lazy_cell)]

use self::{cache::*, subs::*};
use clap::Parser;
use yggdrasil_error::Result;

mod cache;
mod subs;

mod config;

#[derive(Parser)]
// #[clap(version = crate_version!(), author = crate_authors!("\n"))]
// #[clap(setting = AppSettings::ColoredHelp)]
pub struct Ycc {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "yggdrasil.toml")]
    config: String,
    #[clap(subcommand)]
    subs: YccCommand,
}

fn main() -> Result<()> {
    let opts: Ycc = Ycc::parse();
    opts.subs.run(&opts)
}
