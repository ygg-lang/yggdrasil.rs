#![feature(fs_try_exists)]
#![feature(lazy_cell)]

use self::subs::*;
use crate::config::YccConfig;
use clap::Parser;
use std::path::PathBuf;
use yggdrasil_error::Result;

mod cache;
mod subs;

pub mod config;

pub use cache::GaiaSystem;

#[derive(Parser)]
// #[clap(version = crate_version!(), author = crate_authors!("\n"))]
// #[clap(setting = AppSettings::ColoredHelp)]
pub struct Ycc {
    /// Sets a custom config file. Could have been an `Option<T>` with no default too
    #[clap(short, long)]
    config: Option<PathBuf>,
    #[clap(subcommand)]
    subs: YccCommand,
}

fn main() -> Result<()> {
    let Ycc { config, subs, .. } = Ycc::parse();
    let config = YccConfig::load_path(config)?;
    subs.run(&config)
}
