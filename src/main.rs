use std::process::exit;

use clap::Parser;

use colored::Colorize;
use envy::{
    cli::{self, Cli},
    commands::root,
};

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        let _ = root::handle_list();
    }

    let result = match cli.command.unwrap() {
        cli::Command::Set(args) => root::handle_set(args),
        cli::Command::List => root::handle_list(),
        cli::Command::Remove(args) => root::handle_remove(args),
    };

    if let Err(e) = result {
        eprintln!("{}: {e}", "Error".red());
        exit(1)
    }
}
