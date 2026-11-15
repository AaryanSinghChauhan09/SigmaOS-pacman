//! Package implementation
//!
//! Package data structure

use std::path::PathBuf;
use crate::pacman::FileEntry;

/// Package
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub license: String,
    pub arch: String,
    pub size: u64,
    pub installed_size: u64,
    pub depends: Vec<String>,
    pub opt_depends: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
    pub files: Vec<FileEntry>,
    pub scripts: PackageScripts,
    pub repo: String,
    pub build_date: u64,
}

/// Package scripts
#[derive(Debug, Clone, Default)]
pub struct PackageScripts {
    pub pre_install: Option<String>,
    pub post_install: Option<String>,
    pub pre_remove: Option<String>,
    pub post_remove: Option<String>,
    pub pre_upgrade: Option<String>,
    pub post_upgrade: Option<String>,
}

impl Package {
    /// Create new package
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: String::new(),
            url: String::new(),
            license: String::new(),
            arch: String::new(),
            size: 0,
            installed_size: 0,
            depends: Vec::new(),
            opt_depends: Vec::new(),
            conflicts: Vec::new(),
            provides: Vec::new(),
            replaces: Vec::new(),
            files: Vec::new(),
            scripts: PackageScripts::default(),
            repo: String::new(),
            build_date: 0,
        }
    }
    
    /// Check if package is installed
    pub fn is_installed(&self) -> bool {
        // Would check database
        false
    }
    
    /// Get package dependencies
    pub fn dependencies(&self) -> &[String] {
        &self.depends
    }
    
    /// Get package conflicts
    pub fn conflicts(&self) -> &[String] {
        &self.conflicts
    }
    
    /// Get package provides
    pub fn provides(&self) -> &[String] {
        &self.provides
    }
}
