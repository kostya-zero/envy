use std::env;
use std::fs;
use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use colored::Colorize;

use crate::cli::GetArgs;
use crate::cli::RemoveArgs;
use crate::terminal::print_done;
use crate::{cli::SetArgs, loader};

pub fn handle_new() -> Result<()> {
    let cwd = env::current_dir().map_err(|_| anyhow!("failed to get current directory:"))?;
    let env_path = cwd.join(".env");
    let env_example_path = cwd.join(".env.example");

    if env_path.exists() {
        bail!("environment file already exists");
    }

    let content = load_template_content(&env_example_path)?;
    fs::write(env_path, content).map_err(|e| anyhow!("failed to write .env file: {e}"))?;

    if env_example_path.exists() {
        print_done("Generated .env file from .env.example");
    } else {
        print_done("Created empty .env file.");
    }

    Ok(())
}

fn load_template_content(example_path: &Path) -> Result<String> {
    if example_path.exists() {
        fs::read_to_string(example_path).map_err(|e| anyhow!("failed to read .env.example: {e}"))
    } else {
        Ok(String::new())
    }
}

pub fn handle_set(args: SetArgs) -> Result<()> {
    let mut envfile = loader::load_env(".env")?;
    envfile.set(&args.key, &args.value)?;
    loader::save_env(".env", &envfile)?;
    Ok(())
}

pub fn handle_get(args: GetArgs) -> Result<()> {
    let envfile = loader::load_env(".env")?;
    if let Ok(v) = envfile.get(&args.key) {
        println!("{v}");
    } else {
        bail!("key not found");
    }

    Ok(())
}

pub fn handle_list() -> Result<()> {
    let envfile = loader::load_env(".env")?;

    if envfile.is_empty() {
        println!("No keys in env file.");
        return Ok(());
    }

    let keys = envfile.get_all();
    for (k, v) in keys.iter() {
        println!("{}: {}", k.bold(), v.dimmed());
    }

    Ok(())
}

pub fn handle_remove(args: RemoveArgs) -> Result<()> {
    let mut envfile = loader::load_env(".env")?;
    envfile.remove(&args.name)?;

    loader::save_env(".env", &envfile)?;
    Ok(())
}
