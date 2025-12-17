use anyhow::Result;
use std::{fs, io::ErrorKind};
use thiserror::Error;

use crate::envfile::Envfile;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("file '{0}' is not found")]
    FileNotFound(String),

    #[error("failed to read the file: {0}")]
    ReadFailed(String),

    #[error("failed to write to file: {0}")]
    WriteFailed(String),
}

pub fn load_env(path: &str) -> Result<Envfile> {
    let content = fs::read_to_string(path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => LoaderError::FileNotFound(path.to_string()),
        _ => LoaderError::ReadFailed(e.to_string()),
    })?;
    let envfile = Envfile::from_string(content)?;
    Ok(envfile)
}

pub fn save_env(path: &str, envfile: &Envfile) -> Result<()> {
    let dump = envfile.dump();
    fs::write(path, dump).map_err(|e| LoaderError::WriteFailed(e.to_string()))?;
    Ok(())
}
