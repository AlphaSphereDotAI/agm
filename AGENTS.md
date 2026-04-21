# Project Context for agm

## Project Overview

**agm (Agent Manager)** is a Rust-based CLI tool designed to help developers
manage, download, and configure AI agent skills and MCP servers using GitHub
as a primary registry.

- **Main Technologies:**
  - `clap` (CLI parser)
  - `tokio` (async runtime)
  - `reqwest` (HTTP client)
  - `serde` (serialization)
  - `miette` (error reporting)
- **Architecture:** A modular CLI application. The core command logic is being
  built out in subcommands (e.g., `search`, `install`).

## Building and Running

The project uses standard Rust tooling and `devenv` for environment management.

- **Setup Environment:** `devenv shell`
- **Build:** `cargo build`
- **Run CLI:** `cargo run -- [args]`
- **Run Tests:** `cargo test`
- **Linting:** `cargo clippy`
- **Formatting:** `cargo fmt`
