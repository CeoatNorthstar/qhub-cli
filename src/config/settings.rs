use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: u32,
    pub user: Option<UserConfig>,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub quantum: QuantumConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

fn default_version() -> u32 {
    CONFIG_VERSION
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            user: None,
            ai: AiConfig::default(),
            quantum: QuantumConfig::default(),
            ui: UiConfig::default(),
        }
    }
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
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantumConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub default_backend: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_scroll_speed")]
    pub scroll_speed: u16,
    #[serde(default = "default_true")]
    pub show_timestamps: bool,
    #[serde(default = "default_true")]
    pub syntax_highlighting: bool,
}

fn default_model() -> String {
    "deepseek/deepseek-chat".to_string()
}

fn default_max_tokens() -> u32 {
    4096
}

fn default_scroll_speed() -> u16 {
    3
}

fn default_true() -> bool {
    true
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "deepseek".to_string(),
            api_key: None,
            model: default_model(),
            max_tokens: default_max_tokens(),
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

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            scroll_speed: default_scroll_speed(),
            show_timestamps: default_true(),
            syntax_highlighting: default_true(),
        }
    }
}

impl Config {
    /// Get the configuration directory (~/.qhub or platform-specific)
    pub fn config_dir() -> Result<PathBuf> {
        dirs::home_dir()
            .map(|home| home.join(".qhub"))
            .context("Could not find home directory")
    }

    /// Get the configuration file path
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Get the files directory for storing quantum programs
    pub fn files_dir() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("files"))
    }
    
    /// Get the cache directory for temporary data
    pub fn cache_dir() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("cache"))
    }

    /// Load configuration from file, with environment variable overrides
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        
        let mut config = if path.exists() {
            let content = fs::read_to_string(&path)
                .context("Failed to read config file")?;
            toml::from_str::<Config>(&content)
                .context("Failed to parse config file")?
        } else {
            Config::default()
        };

        // Apply environment variable overrides (higher precedence)
        config.apply_env_overrides();
        
        // Validate configuration
        config.validate()?;
        
        Ok(config)
    }
    
    /// Apply environment variable overrides to configuration
    fn apply_env_overrides(&mut self) {
        // AI Configuration
        if let Ok(key) = std::env::var("CLOUDFLARE_AI_TOKEN") {
            self.ai.api_key = Some(key);
        }
        if let Ok(provider) = std::env::var("QHUB_AI_PROVIDER") {
            self.ai.provider = provider;
        }
        if let Ok(model) = std::env::var("QHUB_AI_MODEL") {
            self.ai.model = model;
        }
        
        // Quantum Configuration
        if let Ok(key) = std::env::var("IBM_QUANTUM_TOKEN") {
            self.quantum.api_key = Some(key);
        }
        if let Ok(provider) = std::env::var("QHUB_QUANTUM_PROVIDER") {
            self.quantum.provider = provider;
        }
        if let Ok(backend) = std::env::var("QHUB_QUANTUM_BACKEND") {
            self.quantum.default_backend = Some(backend);
        }
    }
    
    /// Validate configuration values
    fn validate(&self) -> Result<()> {
        // Version check for future migrations
        if self.version > CONFIG_VERSION {
            anyhow::bail!(
                "Config file version {} is newer than supported version {}. Please update qhub.",
                self.version,
                CONFIG_VERSION
            );
        }
        
        // Validate AI provider
        let valid_ai_providers = ["deepseek", "openai", "anthropic"];
        if !valid_ai_providers.contains(&self.ai.provider.as_str()) {
            anyhow::bail!(
                "Invalid AI provider '{}'. Valid options: {}",
                self.ai.provider,
                valid_ai_providers.join(", ")
            );
        }
        
        // Validate quantum provider
        let valid_quantum_providers = ["ibm", "simulator"];
        if !valid_quantum_providers.contains(&self.quantum.provider.as_str()) {
            anyhow::bail!(
                "Invalid quantum provider '{}'. Valid options: {}",
                self.quantum.provider,
                valid_quantum_providers.join(", ")
            );
        }
        
        Ok(())
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let dir = Self::config_dir()?;
        
        // Ensure directory exists
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .context("Failed to create config directory")?;
        }
        
        // Serialize and write
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&path, content)
            .context("Failed to write config file")?;
        
        Ok(())
    }
    
    /// Create initial configuration with prompts
    pub fn create_initial() -> Result<Self> {
        let config = Config::default();
        config.save()?;
        Ok(config)
    }

    /// Ensure all required directories exist
    pub fn ensure_dirs() -> Result<()> {
        let dirs = [
            Self::config_dir()?,
            Self::files_dir()?,
            Self::cache_dir()?,
        ];
        
        for dir in &dirs {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
            }
        }
        
        Ok(())
    }
    
    /// Check if configuration file exists
    pub fn exists() -> bool {
        Self::config_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }
    
    /// Get AI API key with fallback to default
    pub fn get_ai_api_key(&self) -> Option<String> {
        self.ai.api_key.clone()
            .or_else(|| std::env::var("CLOUDFLARE_AI_TOKEN").ok())
    }
    
    /// Get quantum API key
    pub fn get_quantum_api_key(&self) -> Option<String> {
        self.quantum.api_key.clone()
            .or_else(|| std::env::var("IBM_QUANTUM_TOKEN").ok())
    }
}
