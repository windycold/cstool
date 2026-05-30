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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_set() {
        let args = vec!["chsrc_rs", "--scope", "user", "set", "pip", "tuna"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(cli.scope, Some(ScopeArg::User)));
        assert!(!cli.ipv6);
        match cli.command {
            Command::Set { target, mirror } => {
                assert_eq!(target, "pip");
                assert_eq!(mirror, Some("tuna".to_string()));
            }
            _ => panic!("Expected Set command"),
        }
    }

    #[test]
    fn test_cli_parsing_reset() {
        let args = vec!["chsrc_rs", "--ipv6", "reset", "pip"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.scope.is_none());
        assert!(cli.ipv6);
        match cli.command {
            Command::Reset { target } => {
                assert_eq!(target, "pip");
            }
            _ => panic!("Expected Reset command"),
        }
    }

    #[test]
    fn test_cli_parsing_list() {
        let args = vec!["chsrc_rs", "list"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(!cli.ipv6);
        match cli.command {
            Command::List { target } => {
                assert!(target.is_none());
            }
            _ => panic!("Expected List command"),
        }
    }
}

