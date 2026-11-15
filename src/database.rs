//! Database implementation
//!
//! Package database management

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::package::Package;

/// Package database
pub struct PackageDatabase {
    pub packages: HashMap<String, Package>,
    pub files: HashMap<PathBuf, Vec<String>>,
    pub groups: HashMap<String, Vec<String>>,
    pub installed: bool,
}

impl PackageDatabase {
    /// Create new package database
    pub fn new() -> Result<Self> {
        Ok(Self {
            packages: HashMap::new(),
            files: HashMap::new(),
            groups: HashMap::new(),
            installed: false,
        })
    }
    
    /// Initialize database
    pub fn init(&mut self) -> Result<()> {
        // Load database from disk
        self.installed = true;
        Ok(())
    }
    
    /// Add package
    pub fn add(&mut self, package: Package) -> Result<()> {
        self.packages.insert(package.name.clone(), package.clone());
        
        // Update file database
        for file in &package.files {
            self.files.entry(file.path.clone())
                .or_insert_with(Vec::new)
                .push(package.name.clone());
        }
        
        Ok(())
    }
    
    /// Remove package
    pub fn remove(&mut self, name: &str) -> Result<()> {
        if let Some(package) = self.packages.remove(name) {
            // Update file database
            for file in &package.files {
                if let Some(packages) = self.files.get_mut(&file.path) {
                    packages.retain(|p| p != name);
                }
            }
        }
        Ok(())
    }
    
    /// Get package
    pub fn get(&self, name: &str) -> Result<Package> {
        self.packages.get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Package not found: {}", name))
    }
    
    /// Get installed package
    pub fn get_installed(&self, name: &str) -> Result<Package> {
        self.get(name)
    }
    
    /// Check if package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.packages.contains_key(name)
    }
    
    /// List all packages
    pub fn list(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }
    
    /// List installed packages
    pub fn list_installed(&self) -> Result<Vec<Package>> {
        Ok(self.packages.values().cloned().collect())
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
    
    /// Get packages by file
    pub fn get_packages_by_file(&self, path: &PathBuf) -> Vec<String> {
        self.files.get(path)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Add group
    pub fn add_group(&mut self, name: &str, packages: Vec<String>) {
        self.groups.insert(name.to_string(), packages);
    }
    
    /// Get group
    pub fn get_group(&self, name: &str) -> Option<&Vec<String>> {
        self.groups.get(name)
    }
    
    /// Remove group
    pub fn remove_group(&mut self, name: &str) -> Option<Vec<String>> {
        self.groups.remove(name)
    }
    
    /// List groups
    pub fn list_groups(&self) -> Vec<&String> {
        self.groups.keys().collect()
    }
}
