use anyhow::Result;
use std::fs;
use thiserror::Error;

use crate::envfile::Envfile;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("failed to read the file: {0}")]
    ReadFailed(String),

    #[error("failed to write to file")]
    WriteFailed,

    #[error("unexpected error occurred: {0}")]
    UnexpectedError(String),
}

pub fn load_env(path: &str) -> Result<Envfile> {
    let content = fs::read_to_string(path).map_err(|e| LoaderError::ReadFailed(e.to_string()))?;
    let envfile = Envfile::from_string(content)?;
    Ok(envfile)
}

pub fn save_env(path: &str, envfile: &Envfile) -> Result<()> {
    let dump = envfile.dump();
    fs::write(path, dump).map_err(|_| LoaderError::WriteFailed)?;
    Ok(())
}
