//! Demo for SigmaOS Pacman
//!
//! This binary demonstrates pacman functionality

use sigmaos_pacman::*;
use anyhow::Result;

fn main() -> Result<()> {
    println!("SigmaOS Pacman Demo");
    println!("===================");
    
    // Create pacman
    let mut pacman = Pacman::new()?;
    pacman.init()?;
    
    println!("Pacman initialized");
    
    // Add repositories
    pacman.add_repo("core", "https://repo.archlinux.org/core")?;
    pacman.add_repo("extra", "https://repo.archlinux.org/extra")?;
    pacman.add_repo("community", "https://repo.archlinux.org/community")?;
    
    println!("Added repositories: core, extra, community");
    
    // Sync repositories
    pacman.sync()?;
    println!("Repositories synced");
    
    // Install package
    println!("\nInstalling package: firefox");
    pacman.install("firefox")?;
    println!("Package firefox installed");
    
    // Install package with dependencies
    println!("\nInstalling package: vlc");
    pacman.install_with_deps("vlc")?;
    println!("Package vlc installed with dependencies");
    
    // Search packages
    println!("\nSearching for 'browser':");
    let results = pacman.search("browser")?;
    for package in results.iter().take(5) {
        println!("  {}: {}", package.name, package.description);
    }
    
    // Get package info
    println!("\nPackage info for firefox:");
    if let Ok(info) = pacman.info("firefox") {
        println!("  Name: {}", info.name);
        println!("  Version: {}", info.version);
        println!("  Description: {}", info.description);
        println!("  Depends: {:?}", info.depends);
    }
    
    // List installed packages
    println!("\nInstalled packages:");
    let installed = pacman.list_installed()?;
    for package in installed.iter().take(10) {
        println!("  {}: {}", package.name, package.version);
    }
    
    // List package files
    println!("\nFiles in firefox package:");
    let files = pacman.list_files("firefox")?;
    for file in files.iter().take(10) {
        println!("  {}", file.path.display());
    }
    
    // Update packages
    println!("\nUpdating packages...");
    pacman.update()?;
    println!("Packages updated");
    
    // Remove package
    println!("\nRemoving package: vlc");
    pacman.remove("vlc")?;
    println!("Package vlc removed");
    
    // Configuration
    println!("\nConfiguration:");
    pacman.set_config("ParallelDownloads", "10")?;
    pacman.set_config("Color", "always")?;
    let color = pacman.get_config("Color")?;
    println!("  Color: {}", color);
    
    // Hooks
    println!("\nHooks:");
    pacman.add_hook("post-install", "firefox", "/usr/bin/firefox --version")?;
    pacman.execute_hooks("post-install", "firefox")?;
    println!("  Hook executed for firefox");
    
    println!("\nPacman demo completed successfully!");
    
    Ok(())
}
