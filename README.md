# SigmaOS Pacman

> Package manager implementation for SigmaOS inspired by Arch Linux pacman

## Overview

This component implements a package manager for SigmaOS inspired by Arch Linux's pacman. It provides package installation, removal, updating, and dependency resolution with support for Arch-style packages and repositories.

## Features

- Package installation and removal
- Dependency resolution and management
- Repository synchronization
- Package database management
- Arch-style package format support
- AUR (Arch User Repository) integration
- Package verification and signatures
- Rollback support
- Transaction-based operations
- Package file database
- Configuration management
- Hooks and scripts
- Parallel downloads
- Package groups and metapackages

## Architecture

### Package Database

```rust
pub struct PackageDatabase {
    pub packages: HashMap<String, Package>,
    pub files: HashMap<PathBuf, Vec<String>>, // file -> packages
    pub groups: HashMap<String, Vec<String>>, // group -> packages
    pub repos: HashMap<String, Repository>,
}

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
```

### Repository

```rust
pub struct Repository {
    pub name: String,
    pub url: String,
    pub packages: Vec<String>,
    pub synced: bool,
    pub last_sync: u64,
}

pub trait RepositorySync {
    fn sync(&mut self) -> Result<()>;
    fn get_package(&self, name: &str) -> Option<&Package>;
    fn search(&self, query: &str) -> Vec<&Package>;
}
```

### Transaction

```rust
pub struct Transaction {
    pub id: u64,
    pub operations: Vec<Operation>,
    pub status: TransactionStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
}

pub enum Operation {
    Install { package: String, repo: String },
    Remove { package: String },
    Upgrade { package: String, from: String, to: String },
}
```

## Installation

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS-pacman.git

# Build
cd SigmaOS-pacman
cargo build --release
```

## Usage

### Initialize Pacman

```rust
use sigmaos_pacman::*;

fn main() -> Result<()> {
    let mut pacman = Pacman::new()?;
    pacman.init()?;
    
    // Add repositories
    pacman.add_repo("core", "https://repo.archlinux.org/core")?;
    pacman.add_repo("extra", "https://repo.archlinux.org/extra")?;
    pacman.add_repo("community", "https://repo.archlinux.org/community")?;
    
    // Sync repositories
    pacman.sync()?;
    
    Ok(())
}
```

### Install Package

```rust
// Install package
pacman.install("firefox")?;

// Install with dependencies
pacman.install_with_deps("vlc")?;

// Install from AUR
pacman.install_aur("yay")?;

// Install package group
pacman.install_group("base-devel")?;
```

### Remove Package

```rust
// Remove package
pacman.remove("firefox")?;

// Remove with dependencies
pacman.remove_with_deps("vlc")?;

// Remove package group
pacman.remove_group("base-devel")?;
```

### Update Packages

```rust
// Update all packages
pacman.update()?;

// Update specific package
pacman.update_package("firefox")?;

// Update from AUR
pacman.update_aur("yay")?;

// Upgrade all packages
pacman.upgrade()?;
```

### Search Packages

```rust
// Search packages
let results = pacman.search("browser")?;
for package in results {
    println!("{}: {}", package.name, package.description);
}

// Search in repositories
let results = pacman.search_repo("firefox")?;

// Search in AUR
let results = pacman.search_aur("firefox")?;
```

### Package Information

```rust
// Get package info
let info = pacman.info("firefox")?;
println!("Name: {}", info.name);
println!("Version: {}", info.version);
println!("Description: {}", info.description);
println!("Depends: {:?}", info.depends);

// List installed packages
let installed = pacman.list_installed()?;
for package in installed {
    println!("{}: {}", package.name, package.version);
}

// List package files
let files = pacman.list_files("firefox")?;
for file in files {
    println!("{}", file.path);
}
```

### Configuration

```rust
// Set configuration options
pacman.set_config("ParallelDownloads", 5)?;
pacman.set_config("Color", "always")?;
pacman.set_config("VerbosePkgLists", true)?;

// Get configuration
let color = pacman.get_config("Color")?;

// Load configuration file
pacman.load_config("/etc/pacman.conf")?;

// Save configuration
pacman.save_config("/etc/pacman.conf")?;
```

### Hooks and Scripts

```rust
// Add hook
pacman.add_hook("post-install", "firefox", "/usr/bin/firefox --version")?;

// Remove hook
pacman.remove_hook("post-install", "firefox")?;

// Execute hooks
pacman.execute_hooks("post-install", "firefox")?;
```

## Package Format

### Arch Package (.pkg.tar.zst)

```
Package structure:
- .PKGINFO: Package metadata
- .BUILDINFO: Build information
- .INSTALL: Installation script
- .MTREE: File tree metadata
- .CHANGELOG: Package changelog
- usr/, etc/, var/: Package files
```

### Package Metadata

```rust
pub struct PackageMetadata {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgdesc: String,
    pub url: String,
    pub builddate: u64,
    pub packager: String,
    pub size: u64,
    pub arch: String,
    pub license: Vec<String>,
    pub depend: Vec<String>,
    pub optdepend: Vec<String>,
    pub conflict: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
}
```

## Integration with SigmaOS

- Works with AUR helper
- Integrates with dpkg-compat
- Works with portage
- Supports SigmaOS package format

## Requirements

- Package database storage
- Repository access
- Dependency resolution engine
- Package verification
- Configuration management

## Development

See [SigmaOS Development Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/development) for general guidelines.

## Testing

```bash
# Run unit tests
cargo test

# Run package tests
cargo test --test package

# Run repository tests
cargo test --test repository

# Run dependency tests
cargo test --test dependency
```

## License

MIT OR GPL-2.0 — Free and Open Source Software.
© SigmaOS Contributors
