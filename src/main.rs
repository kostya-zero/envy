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
        if let Err(e) = root::handle_list() {
            eprintln!("{}: {e}", "error".bright_red().bold());
            exit(1);
        }
        exit(0);
    }

    let result = match cli.command.unwrap() {
        cli::Command::New => root::handle_new(),
        cli::Command::Set(args) => root::handle_set(args),
        cli::Command::Get(args) => root::handle_get(args),
        cli::Command::List => root::handle_list(),
        cli::Command::Remove(args) => root::handle_remove(args),
    };

    if let Err(e) = result {
        eprintln!("{}: {e}", "error".bright_red().bold());
        exit(1)
    }
}
