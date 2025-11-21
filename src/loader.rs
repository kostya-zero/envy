use anyhow::Result;
use std::{env, fs};
use thiserror::Error;

use crate::envfile::Envfile;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("No env file found.")]
    FileNotFound,

    #[error("Failed to read filke contents.")]
    ReadFailed,

    #[error("Unexpected error occurred: {0}")]
    UnexpectedError(String),
}

pub fn load_env() -> Result<Envfile> {
    let mut cwd = env::current_dir()
        .map_err(|_| LoaderError::UnexpectedError("cannot get current directory".to_string()))?;
    cwd = cwd.join(".env");

    if !cwd.exists() {
        return Err(LoaderError::FileNotFound.into());
    }

    let content = fs::read_to_string(cwd).map_err(|_| LoaderError::ReadFailed)?;
    let envfile = Envfile::from_string(content)?;
    Ok(envfile)
}
