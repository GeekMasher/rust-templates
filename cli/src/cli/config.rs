use std::path::PathBuf;
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// Configuration settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    // config settings
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ..Default::default()
        }
    }
}

impl Config {
    /// Load YAML the configuration from a file path 
    pub fn load(path: PathBuf) -> Result<Self> {
        let config = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&config)?;
        Ok(config)
    }
}
