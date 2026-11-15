//! Pacman implementation
//!
//! Main package manager

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::package::Package;
use crate::repository::Repository;
use crate::database::PackageDatabase;
use crate::transaction::Transaction;
use crate::config::PacmanConfig;
use crate::hook::Hook;
use crate::aur::AurHelper;
use crate::group::PackageGroup;

/// Pacman package manager
pub struct Pacman {
    pub database: PackageDatabase,
    pub repositories: HashMap<String, Repository>,
    pub config: PacmanConfig,
    pub hooks: Vec<Hook>,
    pub aur: AurHelper,
    pub groups: HashMap<String, PackageGroup>,
    pub initialized: bool,
}

impl Pacman {
    /// Create new pacman
    pub fn new() -> Result<Self> {
        Ok(Self {
            database: PackageDatabase::new()?,
            repositories: HashMap::new(),
            config: PacmanConfig::default(),
            hooks: Vec::new(),
            aur: AurHelper::new()?,
            groups: HashMap::new(),
            initialized: false,
        })
    }
    
    /// Initialize pacman
    pub fn init(&mut self) -> Result<()> {
        // Load configuration
        self.config.load_default()?;
        
        // Initialize database
        self.database.init()?;
        
        self.initialized = true;
        Ok(())
    }
    
    /// Add repository
    pub fn add_repo(&mut self, name: &str, url: &str) -> Result<()> {
        let repo = Repository::new(name, url)?;
        self.repositories.insert(name.to_string(), repo);
        Ok(())
    }
    
    /// Remove repository
    pub fn remove_repo(&mut self, name: &str) -> Result<()> {
        self.repositories.remove(name);
        Ok(())
    }
    
    /// Sync repositories
    pub fn sync(&mut self) -> Result<()> {
        for repo in self.repositories.values_mut() {
            repo.sync()?;
        }
        Ok(())
    }
    
    /// Install package
    pub fn install(&mut self, package_name: &str) -> Result<()> {
        // Find package in repositories
        let package = self.find_package(package_name)?;
        
        // Create transaction
        let transaction = Transaction::new_install(package.clone())?;
        
        // Execute transaction
        self.execute_transaction(transaction)?;
        
        // Execute hooks
        self.execute_hooks("post-install", package_name)?;
        
        Ok(())
    }
    
    /// Install package with dependencies
    pub fn install_with_deps(&mut self, package_name: &str) -> Result<()> {
        // Find package and dependencies
        let package = self.find_package(package_name)?;
        let deps = self.resolve_dependencies(&package)?;
        
        // Create transaction
        let mut transaction = Transaction::new_install(package)?;
        for dep in deps {
            transaction.add_operation(crate::transaction::Operation::Install {
                package: dep.name.clone(),
                repo: dep.repo.clone(),
            });
        }
        
        // Execute transaction
        self.execute_transaction(transaction)?;
        
        Ok(())
    }
    
    /// Install package from AUR
    pub fn install_aur(&mut self, package_name: &str) -> Result<()> {
        // Build package from AUR
        let package = self.aur.build_package(package_name)?;
        
        // Install package
        self.install_package(&package)?;
        
        Ok(())
    }
    
    /// Install package group
    pub fn install_group(&mut self, group_name: &str) -> Result<()> {
        if let Some(group) = self.groups.get(group_name) {
            for package_name in &group.packages {
                self.install_with_deps(package_name)?;
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Group not found: {}", group_name))
        }
    }
    
    /// Remove package
    pub fn remove(&mut self, package_name: &str) -> Result<()> {
        // Find installed package
        let package = self.database.get_installed(package_name)?;
        
        // Create transaction
        let transaction = Transaction::new_remove(package)?;
        
        // Execute transaction
        self.execute_transaction(transaction)?;
        
        // Execute hooks
        self.execute_hooks("post-remove", package_name)?;
        
        Ok(())
    }
    
    /// Remove package with dependencies
    pub fn remove_with_deps(&mut self, package_name: &str) -> Result<()> {
        // Find installed package and dependencies
        let package = self.database.get_installed(package_name)?;
        let deps = self.find_reverse_dependencies(package_name)?;
        
        // Create transaction
        let mut transaction = Transaction::new_remove(package)?;
        for dep in deps {
            transaction.add_operation(crate::transaction::Operation::Remove {
                package: dep,
            });
        }
        
        // Execute transaction
        self.execute_transaction(transaction)?;
        
        Ok(())
    }
    
    /// Remove package group
    pub fn remove_group(&mut self, group_name: &str) -> Result<()> {
        if let Some(group) = self.groups.get(group_name) {
            for package_name in &group.packages {
                self.remove_with_deps(package_name)?;
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Group not found: {}", group_name))
        }
    }
    
    /// Update all packages
    pub fn update(&mut self) -> Result<()> {
        // Sync repositories first
        self.sync()?;
        
        // Check for updates
        let updates = self.check_updates()?;
        
        // Upgrade packages
        for update in updates {
            self.upgrade_package(&update.package, &update.from_version, &update.to_version)?;
        }
        
        Ok(())
    }
    
    /// Update specific package
    pub fn update_package(&mut self, package_name: &str) -> Result<()> {
        // Sync repositories
        self.sync()?;
        
        // Check for update
        let update = self.check_package_update(package_name)?;
        
        if let Some(update) = update {
            self.upgrade_package(&update.package, &update.from_version, &update.to_version)?;
        }
        
        Ok(())
    }
    
    /// Update package from AUR
    pub fn update_aur(&mut self, package_name: &str) -> Result<()> {
        // Update AUR package
        self.aur.update_package(package_name)?;
        
        // Reinstall package
        self.install_aur(package_name)?;
        
        Ok(())
    }
    
    /// Upgrade all packages
    pub fn upgrade(&mut self) -> Result<()> {
        self.update()?;
        Ok(())
    }
    
    /// Search packages
    pub fn search(&self, query: &str) -> Result<Vec<Package>> {
        let mut results = Vec::new();
        
        // Search in repositories
        for repo in self.repositories.values() {
            let matches = repo.search(query)?;
            results.extend(matches);
        }
        
        // Search in installed packages
        let installed_matches = self.database.search(query)?;
        results.extend(installed_matches);
        
        // Search in AUR
        let aur_matches = self.aur.search(query)?;
        results.extend(aur_matches);
        
        Ok(results)
    }
    
    /// Search in repositories
    pub fn search_repo(&self, query: &str) -> Result<Vec<Package>> {
        let mut results = Vec::new();
        
        for repo in self.repositories.values() {
            let matches = repo.search(query)?;
            results.extend(matches);
        }
        
        Ok(results)
    }
    
    /// Search in AUR
    pub fn search_aur(&self, query: &str) -> Result<Vec<Package>> {
        self.aur.search(query)
    }
    
    /// Get package info
    pub fn info(&self, package_name: &str) -> Result<Package> {
        self.database.get(package_name)
    }
    
    /// List installed packages
    pub fn list_installed(&self) -> Result<Vec<Package>> {
        self.database.list_installed()
    }
    
    /// List package files
    pub fn list_files(&self, package_name: &str) -> Result<Vec<FileEntry>> {
        let package = self.database.get(package_name)?;
        Ok(package.files.clone())
    }
    
    /// Set configuration option
    pub fn set_config(&mut self, key: &str, value: &str) -> Result<()> {
        self.config.set(key, value)?;
        Ok(())
    }
    
    /// Get configuration option
    pub fn get_config(&self, key: &str) -> Result<String> {
        self.config.get(key)
    }
    
    /// Load configuration file
    pub fn load_config(&mut self, path: &str) -> Result<()> {
        self.config.load(path)?;
        Ok(())
    }
    
    /// Save configuration
    pub fn save_config(&self, path: &str) -> Result<()> {
        self.config.save(path)
    }
    
    /// Add hook
    pub fn add_hook(&mut self, event: &str, package: &str, command: &str) -> Result<()> {
        let hook = Hook::new(event, package, command)?;
        self.hooks.push(hook);
        Ok(())
    }
    
    /// Remove hook
    pub fn remove_hook(&mut self, event: &str, package: &str) -> Result<()> {
        self.hooks.retain(|h| !(h.event == event && h.package == package));
        Ok(())
    }
    
    /// Execute hooks
    pub fn execute_hooks(&self, event: &str, package: &str) -> Result<()> {
        for hook in &self.hooks {
            if hook.event == event && hook.package == package {
                hook.execute()?;
            }
        }
        Ok(())
    }
    
    /// Find package in repositories
    fn find_package(&self, package_name: &str) -> Result<Package> {
        // Search in repositories
        for repo in self.repositories.values() {
            if let Some(package) = repo.get_package(package_name)? {
                return Ok(package.clone());
            }
        }
        
        // Search in AUR
        if let Some(package) = self.aur.get_package(package_name)? {
            return Ok(package.clone());
        }
        
        Err(anyhow::anyhow!("Package not found: {}", package_name))
    }
    
    /// Resolve dependencies
    fn resolve_dependencies(&self, package: &Package) -> Result<Vec<Package>> {
        let mut deps = Vec::new();
        
        for dep_name in &package.depends {
            if let Ok(dep) = self.find_package(dep_name) {
                deps.push(dep);
            }
        }
        
        Ok(deps)
    }
    
    /// Find reverse dependencies
    fn find_reverse_dependencies(&self, package_name: &str) -> Result<Vec<String>> {
        let mut deps = Vec::new();
        
        for installed in self.database.list_installed()? {
            if installed.depends.contains(&package_name.to_string()) {
                deps.push(installed.name);
            }
        }
        
        Ok(deps)
    }
    
    /// Check for updates
    fn check_updates(&self) -> Result<Vec<PackageUpdate>> {
        let mut updates = Vec::new();
        
        for installed in self.database.list_installed()? {
            if let Ok(update) = self.check_package_update(&installed.name) {
                if let Some(update) = update {
                    updates.push(update);
                }
            }
        }
        
        Ok(updates)
    }
    
    /// Check package update
    fn check_package_update(&self, package_name: &str) -> Result<Option<PackageUpdate>> {
        // Find installed package
        let installed = self.database.get_installed(package_name)?;
        
        // Find latest version in repositories
        if let Ok(latest) = self.find_package(package_name) {
            if latest.version != installed.version {
                return Ok(Some(PackageUpdate {
                    package: package_name.to_string(),
                    from_version: installed.version,
                    to_version: latest.version,
                }));
            }
        }
        
        Ok(None)
    }
    
    /// Execute transaction
    fn execute_transaction(&mut self, transaction: Transaction) -> Result<()> {
        for operation in transaction.operations {
            match operation {
                crate::transaction::Operation::Install { package, repo } => {
                    if let Ok(pkg) = self.find_package(&package) {
                        self.install_package(&pkg)?;
                    }
                }
                crate::transaction::Operation::Remove { package } => {
                    self.database.remove(&package)?;
                }
                crate::transaction::Operation::Upgrade { package, from, to } => {
                    self.upgrade_package(&package, &from, &to)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Install package
    fn install_package(&mut self, package: &Package) -> Result<()> {
        self.database.add(package.clone())?;
        Ok(())
    }
    
    /// Upgrade package
    fn upgrade_package(&mut self, package_name: &str, from_version: &str, to_version: &str) -> Result<()> {
        // Remove old version
        self.database.remove(package_name)?;
        
        // Install new version
        let package = self.find_package(package_name)?;
        self.install_package(&package)?;
        
        Ok(())
    }
}

/// Package update information
#[derive(Debug, Clone)]
pub struct PackageUpdate {
    pub package: String,
    pub from_version: String,
    pub to_version: String,
}

/// File entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub mode: u32,
}
