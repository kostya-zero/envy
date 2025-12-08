use anyhow::Result;
use indexmap::IndexMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvfileError {
    #[error("key '{0}' not found")]
    KeyNotFound(String),

    #[error("bad format on line {0}: {1}")]
    BadFormat(usize, EnvfileParseError),
}

#[derive(Debug, Error)]
pub enum EnvfileParseError {
    #[error("key with name '{0}' is already defined")]
    AlreadyDefined(String),

    #[error("syntax error (not a key)")]
    NotKey,
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

    pub fn from_string(content: String) -> Result<Envfile, EnvfileError> {
        let lines = content.lines();
        let mut vars: IndexMap<String, String> = IndexMap::new();
        for (id, raw_line) in lines.enumerate() {
            let line = raw_line.trim();

            // Skip if empty or a comment
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            // Line should have at least one '=' symbol
            if !line.contains("=") {
                return Err(EnvfileError::BadFormat(id + 1, EnvfileParseError::NotKey));
            }

            let splitted = line.split_once("=").unwrap();
            let key = splitted.0.to_string();
            let value = splitted.1.to_string();
            if vars.contains_key(&key) {
                return Err(EnvfileError::BadFormat(
                    id + 1,
                    EnvfileParseError::AlreadyDefined(key),
                ));
            }
            vars.insert(key, value);
        }

        Ok(Self { variables: vars })
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        if self.variables.contains_key(key) {
            *self.variables.get_mut(key).unwrap() = value.to_string();
        } else {
            self.variables.insert(key.to_string(), value.to_string());
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn get(&self, key: &str) -> Result<&String> {
        // First try exact match for performance
        if let Some(v) = self.variables.get(key) {
            return Ok(v);
        }

        // Fall back to case-insensitive search
        let key_lower = key.to_lowercase();
        for (k, v) in self.variables.iter() {
            if k.to_lowercase() == key_lower {
                return Ok(v);
            }
        }

        Err(EnvfileError::KeyNotFound(key.to_string()).into())
    }

    pub fn get_all(&self) -> &IndexMap<String, String> {
        &self.variables
    }

    pub fn dump(&self) -> String {
        let mut result = String::new();

        if self.variables.is_empty() {
            return result;
        }

        for (k, v) in self.variables.iter() {
            result.push_str(format!("{k}={v}\n").as_str());
        }

        result
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        if self.variables.shift_remove(key).is_some() {
            Ok(())
        } else {
            Err(EnvfileError::KeyNotFound(key.to_string()).into())
        }
    }
}
