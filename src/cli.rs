//! Command-line interface definitions using clap for argument parsing.

use clap::{Parser, Subcommand, ValueEnum};

/// Main CLI structure that defines the command-line interface.
#[derive(Parser)]
#[command(name = "chsrc_rs", version, about)]
pub struct Cli {
    /// The subcommand to execute (set, reset, or list).
    #[command(subcommand)]
    pub command: Command,
    /// Optional scope for mirror configuration (system, user, or project).
    #[arg(long = "scope")]
    pub scope: Option<ScopeArg>,
    /// Flag to enable IPv6 support.
    #[arg(long = "ipv6")]
    pub ipv6: bool,
}

/// Available subcommands for the CLI.
#[derive(Subcommand)]
pub enum Command {
    /// Set a new mirror for the specified target.
    Set {
        /// The target software/package manager name.
        target: String,
        /// Optional mirror name to use.
        mirror: Option<String>,
    },
    /// Reset the mirror to the official source.
    Reset {
        /// The target software/package manager name.
        target: String,
    },
    /// List available mirrors or supported targets.
    List {
        /// Optional target name; if omitted, lists all supported targets.
        target: Option<String>,
    },
}

/// Command-line argument for specifying the scope of mirror configuration.
#[derive(Clone, ValueEnum)]
pub enum ScopeArg {
    /// System-wide scope.
    System,
    /// User-level scope.
    User,
    /// Project-specific scope.
    Project,
}
