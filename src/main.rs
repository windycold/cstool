mod app;
mod cli;
mod core;
mod recipes;

use clap::{CommandFactory, Parser};
use cli::Cli;

fn main() {
    let cli = Cli::try_parse().unwrap_or_else(|_| {
        Cli::command().print_help().unwrap();
        std::process::exit(0);
    });
    app::App::new().run(cli); //run the App
}
