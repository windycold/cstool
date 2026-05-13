mod cli;
mod core;
mod recipes;

use crate::cli::ScopeArg;
use crate::core::{MirrorError, Scope};
use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();
    let scope: Option<core::Scope> = cli.scope.map(Into::into);

    match cli.command {
        Command::Set { target, mirror } => match set(&target, mirror, scope) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        },
        Command::Reset { target } => match reset(&target, scope) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        },
        Command::List { target } => match list(target) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        },
    }
}

fn set(
    target: &str,
    mirror: Option<String>,
    scope: Option<Scope>,
) -> Result<(), crate::core::MirrorError> {
    todo!()
}
fn reset(target: &str, scope: Option<Scope>) -> Result<(), crate::core::MirrorError> {
    todo!()
}
fn list(target: Option<String>) -> Result<(), crate::core::MirrorError> {
    match target {
        Some(t) => match recipes::get_manger(&t) {
            Some(manger) => {
                print!(
                    "名称：{}\n维护者：{}\n可用的源：",
                    manger.name(),
                    manger.author()
                );
                for i in manger.available_mirrors() {
                    print!("{}", i.name)
                }
                Ok(())
            }
            None => Err(MirrorError::MangerNotFound(t)),
        },
        None => {
            println!("支持的目标有：");
            for manger in recipes::MANGER_REGISTRY {
                print!("{} ", manger.name());
            }
            Ok(())
        }
    }
}
