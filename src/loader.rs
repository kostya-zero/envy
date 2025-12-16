use anyhow::Result;
use std::{fs, path::Path};
use thiserror::Error;

use crate::envfile::Envfile;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("no file found")]
    FileNotFound,

    #[error("failed to read the file")]
    ReadFailed,

    #[error("failed to write to file")]
    WriteFailed,

    #[error("unexpected error occurred: {0}")]
    UnexpectedError(String),
}

pub fn load_env(path: &str) -> Result<Envfile> {
    // FIXME: Do 'file not found' in read_to_string function instead
    let path = Path::new(&path);
    if !path.exists() {
        return Err(LoaderError::FileNotFound.into());
    }

    let content = fs::read_to_string(path).map_err(|_| LoaderError::ReadFailed)?;
    let envfile = Envfile::from_string(content)?;
    Ok(envfile)
}

pub fn save_env(path: &str, envfile: &Envfile) -> Result<()> {
    let path = Path::new(&path);
    let dump = envfile.dump();
    fs::write(path, dump).map_err(|_| LoaderError::WriteFailed)?;
    Ok(())
}
