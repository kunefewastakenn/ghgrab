use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub github_token: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        let config: Config =
            serde_json::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = serde_json::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(config_path, content).context("Failed to write config file")?;

        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf> {
    let mut path = dirs::config_dir().context("Could not find config directory")?;
    path.push("ghgrab");
    path.push("config.json");
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.github_token.is_none());
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();
        config.github_token = Some("test_token".to_string());
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("test_token"));
        
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.github_token, Some("test_token".to_string()));
    }
}
