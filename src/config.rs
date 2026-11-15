//! Config implementation
//!
//! Pacman configuration management

use anyhow::Result;
use std::collections::HashMap;

/// Pacman configuration
#[derive(Debug, Clone)]
pub struct PacmanConfig {
    pub settings: HashMap<String, String>,
}

impl PacmanConfig {
    /// Create new pacman config
    pub fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }
    
    /// Load default configuration
    pub fn load_default(&mut self) -> Result<()> {
        // Set default values
        self.set("ParallelDownloads", "5")?;
        self.set("Color", "auto")?;
        self.set("VerbosePkgLists", "false")?;
        self.set("CheckSpace", "true")?;
        self.set("BackupOpt", "0")?;
        self.set("LogLevel", "info")?;
        Ok(())
    }
    
    /// Load configuration from file
    pub fn load(&mut self, path: &str) -> Result<()> {
        // Simulate loading config
        Ok(())
    }
    
    /// Save configuration to file
    pub fn save(&self, path: &str) -> Result<()> {
        // Simulate saving config
        Ok(())
    }
    
    /// Set configuration value
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        self.settings.insert(key.to_string(), value.to_string());
        Ok(())
    }
    
    /// Get configuration value
    pub fn get(&self, key: &str) -> Result<String> {
        self.settings.get(key)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Config key not found: {}", key))
    }
    
    /// Get configuration value with default
    pub fn get_or_default(&self, key: &str, default: &str) -> String {
        self.settings.get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }
    
    /// Get boolean configuration value
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        let value = self.get(key)?;
        match value.to_lowercase().as_str() {
            "true" | "yes" | "1" | "on" => Ok(true),
            "false" | "no" | "0" | "off" => Ok(false),
            _ => Err(anyhow::anyhow!("Invalid boolean value for {}: {}", key, value)),
        }
    }
    
    /// Get integer configuration value
    pub fn get_int(&self, key: &str) -> Result<i64> {
        let value = self.get(key)?;
        value.parse::<i64>()
            .map_err(|_| anyhow::anyhow!("Invalid integer value for {}: {}", key, value))
    }
    
    /// List all settings
    pub fn list_settings(&self) -> Vec<(&String, &String)> {
        self.settings.iter().collect()
    }
}

impl Default for PacmanConfig {
    fn default() -> Self {
        Self::new()
    }
}
