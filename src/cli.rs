use clap::{Args, Parser, Subcommand};

/// An environment manager you'll envy
#[derive(Parser)]
#[command(
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    subcommand_required = false,
    arg_required_else_help = false,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Set a value for variable. Can overwrite existing one.
    Set(SetArgs),

    /// Remove a variable.
    Remove(RemoveArgs),

    /// List all available variables.
    List,
}

#[derive(Args)]
pub struct SetArgs {
    /// The key name to insert/update.
    pub key: Option<String>,

    /// The value to set.
    pub values: Option<String>,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// The key name to delete.
    pub name: Option<String>,
}
