//! SigmaOS Pacman
//!
//! Package manager implementation inspired by Arch Linux pacman

#![allow(warnings, unused)]

pub mod pacman;
pub mod package;
pub mod repository;
pub mod solver;
pub mod dependency;
pub mod transaction;
pub mod database;
pub mod config;
pub mod hook;
pub mod aur;
pub mod group;

pub use pacman::*;
pub use package::*;
pub use repository::*;
pub use dependency::*;
pub use transaction::*;
pub use database::*;
pub use config::*;
pub use hook::*;
pub use aur::*;
pub use group::*;
