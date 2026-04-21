# Project Context: agm (Agent Manager)

## Project Overview

**agm (Agent Manager)** is a Rust-based CLI tool designed to help
developers manage, download, and configure AI agent skills and Model
Context Protocol (MCP) servers. It uses GitHub as a primary registry
for discovering and installing these components.

- **Main Technologies:**
  - **Language:** Rust (2024 edition)
  - **CLI Parser:** `clap` (with `derive` features)
  - **Async Runtime:** `tokio`
  - **HTTP Client:** `reqwest` (with `rustls-tls`)
  - **Serialization:** `serde` and `serde_json`
  - **Error Handling:** `miette` (fancy reporting) and `anyhow`
  - **Testing:** `assert_cmd`, `predicates`

- **Architecture:**
  - Modular CLI structure with subcommands.
  - Core logic resides in :
    - `src/`
      - `cli`
      - `client`
      - `config`
      - `registry`
      - `utils`

## Building and Running

The project uses standard Rust tooling and `devenv` for environment management.

- **Setup Environment:** `devenv shell`
- **Build:** `cargo build`
- **Run CLI:** `cargo run -- [args]`
- **Run Tests:** `cargo test`
- **Linting:** `cargo clippy`
- **Formatting:** `cargo fmt`

## Development Conventions

### Quality Standards

- **Documentation:** All public functions and methods must be documented.
- **Non-Interactive:** Prefer non-interactive commands; use `CI=true`.

### Commit Guidelines

Follow the conventional commits format: `<type>(<scope>): <description>`.
Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`.

## Key Directories

- `src/`: Application source code.
- `tests/`: Integration tests.
