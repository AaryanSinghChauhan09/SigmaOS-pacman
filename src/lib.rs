// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-pacman - Arch Linux-inspired package manager
//! Part of SigmaOS multi-repository architecture

#![no_std]

#[path = "pacman.rs"]
pub mod pacman;

pub use pacman::Pacman;
