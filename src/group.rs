//! Group implementation
//!
//! Package groups management

use anyhow::Result;

/// Package group
#[derive(Debug, Clone)]
pub struct PackageGroup {
    pub name: String,
    pub description: String,
    pub packages: Vec<String>,
    pub required: bool,
}

impl PackageGroup {
    /// Create new package group
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            packages: Vec::new(),
            required: false,
        }
    }
    
    /// Add package to group
    pub fn add_package(&mut self, package: &str) {
        self.packages.push(package.to_string());
    }
    
    /// Remove package from group
    pub fn remove_package(&mut self, package: &str) -> bool {
        if let Some(pos) = self.packages.iter().position(|p| p == package) {
            self.packages.remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Check if package is in group
    pub fn has_package(&self, package: &str) -> bool {
        self.packages.contains(&package.to_string())
    }
    
    /// List packages in group
    pub fn list_packages(&self) -> &[String] {
        &self.packages
    }
    
    /// Set group as required
    pub fn set_required(&mut self, required: bool) {
        self.required = required;
    }
    
    /// Check if group is required
    pub fn is_required(&self) -> bool {
        self.required
    }
}
