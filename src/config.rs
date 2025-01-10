use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub notion_token: String,
    pub database_id: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let notion_token = std::env::var("NOTION_TOKEN")
            .map_err(|_| anyhow!("NOTION_TOKEN environment variable not set"))?;
        let database_id = std::env::var("NOTION_DATABASE_ID")
            .map_err(|_| anyhow!("NOTION_DATABASE_ID environment variable not set"))?;

        Ok(Config {
            notion_token,
            database_id,
        })
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let contents = fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            Self::new()
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(config_path, contents)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not find config directory"))?;
        path.push("notion-cli");
        path.push("config.json");
        Ok(path)
    }
} 