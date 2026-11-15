//! AUR implementation
//!
//! Arch User Repository integration

use anyhow::Result;
use std::collections::HashMap;
use crate::package::Package;

/// AUR helper
pub struct AurHelper {
    pub packages: HashMap<String, AurPackage>,
    pub cache_dir: String,
}

/// AUR package
#[derive(Debug, Clone)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub license: String,
    pub maintainer: String,
    pub votes: u32,
    pub popularity: f64,
    pub last_modified: u64,
    pub pkgbuild_url: String,
}

impl AurHelper {
    /// Create new AUR helper
    pub fn new() -> Result<Self> {
        Ok(Self {
            packages: HashMap::new(),
            cache_dir: "/var/cache/aur".to_string(),
        })
    }
    
    /// Search AUR
    pub fn search(&self, query: &str) -> Result<Vec<Package>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        for aur_package in self.packages.values() {
            if aur_package.name.to_lowercase().contains(&query_lower) ||
               aur_package.description.to_lowercase().contains(&query_lower) {
                results.push(Package::new(&aur_package.name, &aur_package.version));
            }
        }
        
        Ok(results)
    }
    
    /// Get AUR package
    pub fn get_package(&self, name: &str) -> Result<Option<Package>> {
        if let Some(aur_package) = self.packages.get(name) {
            Ok(Some(Package::new(&aur_package.name, &aur_package.version)))
        } else {
            Ok(None)
        }
    }
    
    /// Build package from AUR
    pub fn build_package(&self, name: &str) -> Result<Package> {
        if let Some(aur_package) = self.packages.get(name) {
            // Simulate building package
            let mut package = Package::new(&aur_package.name, &aur_package.version);
            package.description = aur_package.description.clone();
            package.url = aur_package.url.clone();
            package.license = aur_package.license.clone();
            Ok(package)
        } else {
            Err(anyhow::anyhow!("AUR package not found: {}", name))
        }
    }
    
    /// Update AUR package
    pub fn update_package(&self, name: &str) -> Result<()> {
        // Simulate updating AUR package
        Ok(())
    }
    
    /// Add AUR package
    pub fn add_package(&mut self, package: AurPackage) {
        self.packages.insert(package.name.clone(), package);
    }
    
    /// Remove AUR package
    pub fn remove_package(&mut self, name: &str) -> Option<AurPackage> {
        self.packages.remove(name)
    }
    
    /// List AUR packages
    pub fn list_packages(&self) -> Vec<&AurPackage> {
        self.packages.values().collect()
    }
    
    /// Get popular packages
    pub fn get_popular(&self, limit: usize) -> Vec<&AurPackage> {
        let mut packages: Vec<&AurPackage> = self.packages.values().collect();
        packages.sort_by(|a, b| b.popularity.partial_cmp(&a.popularity).unwrap());
        packages.truncate(limit);
        packages
    }
}
