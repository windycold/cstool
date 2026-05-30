mod app;
mod cli;
mod core;
mod recipes;

use clap::Parser;
use cli::Cli;

/// Application entry point.
/// Parses command-line arguments using clap and runs the application.
/// Displays help message and exits if argument parsing fails.
fn main() {
    let cli = Cli::parse();
    app::App::new().run(cli); //run the App
}
