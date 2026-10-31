# SigmaOS Pacman

An early `no_std` Rust model of a pacman-style package API. The public `Pacman`
type exposes `sync`, `install`, `remove`, and `search` operations over a package
database interface.

## Current behavior and limits

The current methods are placeholders: `sync`, `install`, and `remove` return
success without changing a package database, and `search` returns an empty
list. This crate does not fetch repositories, verify signatures, write package
files, or manage a host system. Do not use an `Ok(())` result as evidence that
an operation occurred.

Before adding a real backend, define repository trust and signature
verification, dependency resolution, transaction planning, filesystem
ownership, rollback, and failure reporting. Keep preview and apply behavior
separate so callers can inspect changes before mutation.

## Development and licensing

The GitHub workflow runs `cargo test --all-targets --locked`; check its result
for the commit being reviewed. The Cargo manifest declares GPL-3.0-or-later,
while Rust source files currently contain Apache-2.0 and MIT SPDX headers.
Resolve that licensing mismatch before redistribution.
