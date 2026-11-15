//! Repository implementation
//!
//! Package repository management

use anyhow::Result;
use std::collections::HashMap;
use crate::package::Package;

/// Repository
pub struct Repository {
    pub name: String,
    pub url: String,
    pub packages: HashMap<String, Package>,
    pub synced: bool,
    pub last_sync: u64,
}

impl Repository {
    /// Create new repository
    pub fn new(name: &str, url: &str) -> Result<Self> {
        Ok(Self {
            name: name.to_string(),
            url: url.to_string(),
            packages: HashMap::new(),
            synced: false,
            last_sync: 0,
        })
    }
    
    /// Sync repository
    pub fn sync(&mut self) -> Result<()> {
        // Simulate repository sync
        // In real implementation, would download package database
        self.synced = true;
        self.last_sync = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        Ok(())
    }
    
    /// Get package
    pub fn get_package(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }
    
    /// Add package
    pub fn add_package(&mut self, package: Package) {
        self.packages.insert(package.name.clone(), package);
    }
    
    /// Remove package
    pub fn remove_package(&mut self, name: &str) -> Option<Package> {
        self.packages.remove(name)
    }
    
    /// Search packages
    pub fn search(&self, query: &str) -> Result<Vec<Package>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        for package in self.packages.values() {
            if package.name.to_lowercase().contains(&query_lower) ||
               package.description.to_lowercase().contains(&query_lower) {
                results.push(package.clone());
            }
        }
        
        Ok(results)
    }
    
    /// List all packages
    pub fn list_packages(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }
    
    /// Check if repository is synced
    pub fn is_synced(&self) -> bool {
        self.synced
    }
    
    /// Get last sync time
    pub fn get_last_sync(&self) -> u64 {
        self.last_sync
    }
}
