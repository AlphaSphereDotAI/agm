use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs::{read_to_string, write};
use std::path::Path;

pub const CONFIG_FILE: &str = "agm.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    pub version: String,
    pub skills: Vec<String>,
    pub mcps: Vec<String>,
}

impl Base {
    /// Creates a new default AGM config.
    pub fn new(version: String) -> Self {
        Self {
            version,
            skills: Vec::new(),
            mcps: Vec::new(),
        }
    }
}

/// Initializes the config file at the given path.
pub fn init_config(path: impl AsRef<Path>, config: &Base) -> Result<()> {
    let path = path.as_ref();

    if path.exists() {
        bail!(
            "Config file '{}' already exists. Remove it first or use the existing config.",
            path.display()
        );
    }

    let content = to_string_pretty(config).context("failed to serialize config")?;
    write(path, format!("{content}\n"))
        .with_context(|| format!("failed to write '{}'", path.display()))?;
    Ok(())
}

/// Loads the config file from the given path.
pub fn load_config(path: impl AsRef<Path>) -> Result<Base> {
    let path = path.as_ref();

    if !path.exists() {
        bail!(
            "Config file '{}' was not found. Run `agm init` first.",
            path.display()
        );
    }

    let content =
        read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("failed to parse '{}'", path.display()))
}
