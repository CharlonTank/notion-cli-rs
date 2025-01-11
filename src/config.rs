use anyhow::{anyhow, Result};
use std::env;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub notion_token: String,
    pub database_id: String,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Result<Self> {
        let notion_token = env::var("NOTION_TOKEN")
            .map_err(|_| anyhow!("NOTION_TOKEN environment variable not set"))?;
        let database_id = env::var("NOTION_DATABASE_ID")
            .map_err(|_| anyhow!("NOTION_DATABASE_ID environment variable not set"))?;

        Ok(Config {
            notion_token,
            database_id,
        })
    }
} 