mod ast;
mod cst;
mod init;
mod new;
mod publish;
mod test;

use self::{ast::CommandAST, cst::CommandBuild, init::CommandInit, new::CommandNew, publish::CommandPub, test::CommandTest};
use crate::{config::YccConfig, Ycc};
use clap::Parser;
use std::{env, fs, process::Command};
use yggdrasil_error::Result;

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
