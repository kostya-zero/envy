use anyhow::Result;
use indexmap::IndexMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvfileError {
    #[error("Key '{0}' not found.")]
    KeyNotFound(String),
}

#[derive(Debug, Default)]
pub struct Envfile {
    variables: IndexMap<String, String>,
}

impl Envfile {
    pub fn new() -> Self {
        Self {
            variables: IndexMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        if self.variables.contains_key(key) {
            *self.variables.get_mut(key).unwrap() = value.to_string();
        } else {
            self.variables
                .insert(key.to_string(), value.to_string())
                .unwrap();
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<&String> {
        if let Some(v) = self.variables.get(&key.to_string()) {
            Ok(v)
        } else {
            Err(EnvfileError::KeyNotFound(key.to_string()).into())
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        if self.variables.shift_remove(key).is_some() {
            Ok(())
        } else {
            Err(EnvfileError::KeyNotFound(key.to_string()).into())
        }
    }
}
