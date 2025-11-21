use anyhow::Result;

use crate::loader;

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
