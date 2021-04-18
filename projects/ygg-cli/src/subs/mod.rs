mod ast;
mod cst;
mod init;
mod new;
mod test;
mod publish;

use std::{env, fs, path::Path, process::Command};
use anyhow::Result;
use clap::Clap;
use self::ast::CommandAST;
use self::cst::CommandCST;
use self::init::CommandInit;
use self::new::CommandNew;
use self::test::CommandTest;
use self::publish::CommandPub;

#[derive(Clap)]
enum SubCommand {
    Init(CommandInit),
    New(CommandNew),
    CST(CommandCST),
    AST(CommandAST),
    Test(CommandTest),
    Pub(CommandPub)
}