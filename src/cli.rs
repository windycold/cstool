//!使用clap实现cli

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "chsrc_rs", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(long = "scope")]
    pub scope: Option<ScopeArg>,
    #[arg(long = "ipv6")]
    pub ipv6: bool,
}

#[derive(Subcommand)]
pub enum Command {
    Set {
        target: String,
        mirror: Option<String>,
    },
    Reset {
        target: String,
    },
    List {
        target: Option<String>,
    },
}

#[derive(Clone, ValueEnum)]
pub enum ScopeArg {
    System,
    User,
    Project,
}
