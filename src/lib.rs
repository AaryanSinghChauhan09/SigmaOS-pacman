//! SigmaOS-pacman — Arch Linux-inspired package manager library.
//!
//! Modules:
//! - [`pacman`]: core package database, dependency resolution, transactions
//! - [`package`]: package/repository model
//! - [`cache`]: package cache with TTL
//! - [`installer`]: install/uninstall engine
//! - [`config`]/[`error`]/[`service`]: component plumbing shared with the CLI

pub mod cache;
pub mod config;
pub mod error;
pub mod installer;
pub mod package;
pub mod pacman;
pub mod service;

pub use error::{ComponentError, Result};
