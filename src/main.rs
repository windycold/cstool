mod cli;
mod core;
mod recipes;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Set { target, mirror } => match set(&target, mirror, cli.scope) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        },
        Command::Reset { target } => match reset(&target, cli.scope) {
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
    scope: Option<cli::ScopeArg>,
) -> Result<(), crate::core::MirrorError> {
    todo!()
}
fn reset(target: &str, scope: Option<cli::ScopeArg>) -> Result<(), crate::core::MirrorError> {
    todo!()
}
fn list(target: Option<String>) -> Result<(), crate::core::MirrorError> {
    todo!()
}
