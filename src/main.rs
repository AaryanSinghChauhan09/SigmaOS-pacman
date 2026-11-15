use anyhow::Result;
use sigmaos_package_component::cache::PackageCache;
use sigmaos_package_component::config::Config;
use sigmaos_package_component::installer::PackageInstaller;
use sigmaos_package_component::package::{Package, PackageRepository, Version};
use sigmaos_package_component::service::Service;

fn main() -> Result<()> {
    env_logger::init();

    let config = Config::default();
    let mut service = Service::new(config);

    service.start()?;
    println!("Service started successfully");

    // Demonstrate package management
    let mut repo = PackageRepository::new();
    let pkg = Package::new(
        "test-pkg".to_string(),
        Version::new(1, 0, 0),
        "Test package".to_string(),
    );
    repo.add_package(pkg);
    println!(
        "Package repository: {} packages",
        repo.list_packages().len()
    );

    // Demonstrate installation
    let mut installer = PackageInstaller::default();
    let mut transaction = installer.create_transaction("test-pkg".to_string());
    installer
        .install(&mut transaction)
        .map_err(|e| anyhow::anyhow!(e))?;
    println!("Installation transaction completed");

    // Demonstrate cache
    let cache = PackageCache::default();
    println!("Package cache initialized with {} entries", cache.len());

    service.stop()?;
    println!("Service stopped successfully");

    Ok(())
}
