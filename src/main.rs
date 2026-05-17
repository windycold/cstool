mod app;
mod cli;
mod core;
mod recipes;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    app::App::new().run(cli);    //run the App
}
