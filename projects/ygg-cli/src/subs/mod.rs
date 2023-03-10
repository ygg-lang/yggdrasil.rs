mod ast;
mod cst;
mod init;
mod new;
mod publish;
mod test;

use self::{ast::CommandAST, cst::CommandCST, init::CommandInit, new::CommandNew, publish::CommandPub, test::CommandTest};
use crate::Ycc;
use clap::Parser;
use std::{env, fs, path::Path, process::Command};
use yggdrasil_error::Result;

#[derive(Parser)]
pub enum YccCommand {
    Init(CommandInit),
    New(CommandNew),
    CST(CommandCST),
    AST(CommandAST),
    Test(CommandTest),
    Pub(CommandPub),
}

impl YccCommand {
    pub fn run(&self, _: &Ycc) -> Result<()> {
        Self::calibrate_current_dir()?;
        match self {
            YccCommand::Init(cmd) => cmd.run(),
            YccCommand::New(cmd) => cmd.run(),
            YccCommand::CST(cmd) => cmd.run(),
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
