use std::{env, env::current_dir, fs, fs::read_to_string, process::Command};

use clap::Parser;
use wax::{Glob, LinkBehavior, WalkEntry, WalkError};

use yggdrasil_error::Result;
use yggdrasil_shared::parse_grammar_raw;

use crate::{config::YccConfig, GaiaSystem};

use self::{ast::CommandAST, cst::CommandBuild, init::CommandInit, new::CommandNew, publish::CommandPub, test::CommandTest};

mod ast;
mod cst;
mod init;
mod new;
mod publish;
mod test;

#[derive(Parser)]
pub enum YccCommand {
    Init(CommandInit),
    New(CommandNew),
    Build(CommandBuild),
    AST(CommandAST),
    Test(CommandTest),
    Pub(CommandPub),
}

impl YccCommand {
    pub fn run(&self, config: &YccConfig) -> Result<()> {
        Self::calibrate_current_dir()?;
        match self {
            YccCommand::Init(cmd) => cmd.run(),
            YccCommand::New(cmd) => cmd.run(),
            YccCommand::Build(cmd) => cmd.run(config),
            YccCommand::AST(cmd) => cmd.run(),
            YccCommand::Test(cmd) => cmd.run(),
            YccCommand::Pub(cmd) => cmd.run(),
        }
    }
    pub fn calibrate_current_dir() -> Result<()> {
        let curr = env::current_dir()?;
        env::set_current_dir(curr)?;
        Ok(())
    }
}
