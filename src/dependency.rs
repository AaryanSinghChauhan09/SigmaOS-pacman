//! Dependency implementation
//!
//! Dependency resolution

use anyhow::Result;
use std::collections::{HashMap, HashSet};
use crate::package::Package;

/// Dependency resolver
pub struct DependencyResolver {
    pub resolved: HashMap<String, Package>,
    pub visiting: HashSet<String>,
    pub visited: HashSet<String>,
}

impl DependencyResolver {
    /// Create new dependency resolver
    pub fn new() -> Self {
        Self {
            resolved: HashMap::new(),
            visiting: HashSet::new(),
            visited: HashSet::new(),
        }
    }
    
    /// Resolve dependencies
    pub fn resolve(&mut self, package: &Package, find_package: &dyn Fn(&str) -> Result<Package>) -> Result<Vec<Package>> {
        self.visiting.clear();
        self.visited.clear();
        self.resolved.clear();
        
        self.resolve_recursive(package, find_package)?;
        
        Ok(self.resolved.values().cloned().collect())
    }
    
    /// Resolve dependencies recursively
    fn resolve_recursive(&mut self, package: &Package, find_package: &dyn Fn(&str) -> Result<Package>) -> Result<()> {
        // Check for circular dependency
        if self.visiting.contains(&package.name) {
            return Err(anyhow::anyhow!("Circular dependency detected: {}", package.name));
        }
        
        // Already visited
        if self.visited.contains(&package.name) {
            return Ok(());
        }
        
        self.visiting.insert(package.name.clone());
        
        // Resolve dependencies
        for dep_name in &package.depends {
            if let Ok(dep_package) = find_package(dep_name) {
                self.resolve_recursive(&dep_package, find_package)?;
            }
        }
        
        self.visiting.remove(&package.name);
        self.visited.insert(package.name.clone());
        self.resolved.insert(package.name.clone(), package.clone());
        
        Ok(())
    }
    
    /// Check for conflicts
    pub fn check_conflicts(&self, package: &Package, installed: &[Package]) -> Result<()> {
        for installed_package in installed {
            // Check if installed package conflicts with new package
            if installed_package.conflicts.contains(&package.name) {
                return Err(anyhow::anyhow!(
                    "Package {} conflicts with {}",
                    package.name, installed_package.name
                ));
            }
            
            // Check if new package conflicts with installed package
            if package.conflicts.contains(&installed_package.name) {
                return Err(anyhow::anyhow!(
                    "Package {} conflicts with {}",
                    package.name, installed_package.name
                ));
            }
        }
        
        Ok(())
    }
    
    /// Check if package provides required dependency
    pub fn provides(&self, package: &Package, dependency: &str) -> bool {
        package.provides.iter().any(|p| p == dependency) ||
        package.name == dependency
    }
}
