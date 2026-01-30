use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub user: Option<UserConfig>,
    pub ai: AiConfig,
    pub quantum: QuantumConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub email: String,
    pub token: Option<String>,
    pub tier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantumConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub default_backend: Option<String>,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "deepseek".to_string(),
            api_key: None,
        }
    }
}

impl Default for QuantumConfig {
    fn default() -> Self {
        Self {
            provider: "ibm".to_string(),
            api_key: None,
            default_backend: None,
        }
    }
}

impl Config {
    pub fn config_dir() -> PathBuf {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".qhub")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn files_dir() -> PathBuf {
        Self::config_dir().join("files")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        let dir = Self::config_dir();
        
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn ensure_dirs() -> Result<()> {
        let config_dir = Self::config_dir();
        let files_dir = Self::files_dir();
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        if !files_dir.exists() {
            fs::create_dir_all(&files_dir)?;
        }
        
        Ok(())
    }
}
