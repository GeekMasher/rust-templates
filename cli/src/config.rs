use anyhow::Result;
use figment::{
    providers::{Env, Format, Json, Toml, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");

/// Configuration settings
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    // config settings
}

impl Config {
    /// Load configuration from both environment variables and a configuration file
    pub fn load(path: PathBuf) -> Result<Self> {
        let path = path.as_path();
        log::debug!("Loading configuration from {}", path.display());

        let project_name = PROJECT_NAME.to_uppercase();
        log::debug!("Loading environment prefix: {}", project_name);
        let mut fig = Figment::new().merge(Env::prefixed(project_name.as_str()));

        if path.exists() {
            if path
                .extension()
                .is_some_and(|ext| ext == "yaml" || ext == "yml")
            {
                log::debug!("Loading configuration from YAML file");
                fig = fig.merge(Yaml::file(path));
            } else if path.extension().is_some_and(|ext| ext == "toml") {
                log::debug!("Loading configuration from TOML file");
                fig = fig.merge(Toml::file(path));
            } else if path.extension().is_some_and(|ext| ext == "json") {
                log::debug!("Loading configuration from JSON file");
                fig = fig.merge(Json::file(path));
            } else {
                log::warn!("Unsupported configuration file format");
                return Err(anyhow::anyhow!("Unsupported configuration file format"));
            }
        } else {
            log::warn!("Configuration file not found");
        }

        Ok(fig.extract()?)
    }

    /// Update configuration with command line arguments
    #[allow(dead_code, unused)]
    pub fn arguments(&mut self, arguments: &crate::cli::Arguments) {
        todo!("Lets write some code...");
    }

    /// Save configuration to a file
    #[allow(dead_code)]
    pub fn save(&self, path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
        log::debug!("Saving configuration to {}", path.display());

        let data = if path
            .extension()
            .is_some_and(|ext| ext == "yaml" || ext == "yml")
        {
            serde_yaml::to_string(self)?
        } else if path.extension().is_some_and(|ext| ext == "toml") {
            toml::to_string(self)?
        } else if path.extension().is_some_and(|ext| ext == "json") {
            serde_json::to_string(self)?
        } else {
            log::warn!("Unsupported configuration file format");
            return Err(anyhow::anyhow!("Unsupported configuration file format"));
        };

        std::fs::write(path, data)?;

        Ok(())
    }
}
