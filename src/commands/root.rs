use std::fs;

use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

use crate::cli::RemoveArgs;
use crate::{cli::SetArgs, loader};

pub fn handle_set(args: SetArgs) -> Result<()> {
    if args.key.is_none() {
        return Err(anyhow!("No key provided."));
    }
    let key = args.key.unwrap();

    if args.values.is_none() {
        return Err(anyhow!("No value provided."));
    }
    let value = args.values.unwrap();
    let mut envfile = loader::load_env()?;

    if let Err(e) = envfile.set(&key, &value) {
        return Err(anyhow!(e));
    }

    let content = envfile.dump();
    fs::write(".env", content).map_err(|_| anyhow!("An error in file system occurred."))?;

    Ok(())
}

pub fn handle_list() -> Result<()> {
    let envfile = loader::load_env()?;

    if envfile.is_empty() {
        println!("No keys in env file.");
        return Ok(());
    }

    let keys = envfile.get_all();
    for (k, v) in keys.iter() {
        println!("{k}: {v}");
    }

    Ok(())
}

pub fn handle_remove(args: RemoveArgs) -> Result<()> {
    let mut envfile = loader::load_env()?;

    if envfile.is_empty() {
        println!("No keys in env file.");
        return Ok(());
    }

    if args.name.is_none() {
        bail!("No key name provided.");
    }

    envfile.remove(&args.name.unwrap())?;

    let content = envfile.dump();
    fs::write(".env", content).map_err(|_| anyhow!("An error in file system occurred."))?;

    Ok(())
}
