# Architecture

This document describes the architecture and design principles of SABU.

## Workspace

SABU is a Cargo workspace with two parts:

- **`sabu_core`** — a separate crate with the business logic.
- **Application package** (workspace root) — contains `commands`, `i18n`, and `ui`.

The crate is named `sabu_core` because the name `core` conflicts with Rust's built-in `core` crate.

## Clean Architecture

SABU uses Clean Architecture.

- Code is split in layers. The business logic is in the center (`sabu_core`). The interface and other details are on the outside (`commands`, `i18n`, `ui`).
- Business rules do not depend on the interface, files, or frameworks.

## Core / GUI Decoupling

`sabu_core` is responsible for:

- CSV parsing
- Business logic
- Analytics data structures
- State management

Rules:

- `sabu_core` **must not** depend on any user interface component (`egui`, rendering, or layout logic).
- Its `Cargo.toml` must never list `egui` or `eframe`.
- The GUI only consumes `sabu_core`, through exposed APIs and command execution.

## Command Pattern

Every user action, state change, or application feature is a **Command**: a struct that implements a common interface (for example, `trait Command`).

- The GUI sends commands to an execution queue or handler.
- The GUI does not change core state directly.

The pattern must keep these benefits:

- Asynchronous background execution, without freezing the main UI thread.
- Tests for all actions without a UI or a headless display.

### Command IDs

Every command must have a unique id.

- The id is a `&str` constant in `SCREAMING_SNAKE_CASE`.
- The value uses the format `domain.action`, in lowercase `snake_case`.
- Ids are stable identifiers, for example for automation. Renaming one is a breaking change, not a refactor.

```rust
pub const FILE_SAVE: &str = "file.save";
```

## Performance

SABU handles large CSV files from electoral systems.

- Use concurrency, parallel streaming, and multi-threading where appropriate (for example, `rayon`).
- Avoid unnecessary heap allocations and deep copies.
- Never run blocking I/O on the main UI thread.
- CSV parsing and data aggregation must stay fast and must never block the main event loop.

## CSV Encoding

- CSV files are Latin-1 (ISO-8859-1).
- The parser decodes them as Latin-1 (for example, with `encoding_rs`) and converts the text to UTF-8 internally.

## Internationalization (i18n)

- Only `pt-BR` is supported. English or any other language may be added in the future.
- The program must be built to allow multiple languages. Adding a language must not need code changes.
- Translations are stored in JSON files, one per language (for example, `locales/pt-BR.json`).
- No user-facing text may be hardcoded in `core`, `commands`, or `ui`. All text goes through the `i18n` dictionary loader.
- If a key is missing in the active language, the application uses the `pt-BR` text.
- Numbers, dates, and plurals follow the format of the active language (currently `pt-BR`).

## Error Handling

- Never use `unwrap()` or `panic!` (including `expect()`) in production code.
- Tests may use `unwrap()`, `expect()`, and `panic!`.
- All operations that can fail return `Result` and handle errors explicitly.

## Logging

Logs are written to `logs.log` in `%LOCALAPPDATA%\Sabu`.

## Configuration

Configuration (language, theme, etc) is saved in `config.json` in `%LOCALAPPDATA%\Sabu`.

## Distribution

- The final `.exe` is an installer built with `cargo-wix` (WiX Toolset).
- Installation is per user, in `%LOCALAPPDATA%\Programs\Sabu`, without administrator rights.

## Single Responsibility Principle (SRP)

SRP is applied strictly in the whole project.

### What is a responsibility

A responsibility is one reason to change: one cohesive concept or capability. Examples: one command, one CSV parser, one analytics calculation, one UI component, one translation loader.

Test: if you cannot describe a file in one sentence without using "and", it has more than one responsibility.

### Granularity

- **Folder:** one feature or area (for example, `commands/`, `i18n/`, `csv/`). Create a new folder only for a new area, not for a single file.
- **File:** one concept. This is usually one struct or trait, its `impl` blocks, and the private helpers that only it uses. Each `Command` has its own file.
- **Function:** one reason to change.
- **Keep together:** a type and its `impl` blocks, its private helpers, and small types that exist only to support it (for example, its error enum).
- **Do not create** one file per function, or files with only a few lines that always change together with another file.
- **Split a file** when it gets a second reason to change, or when it holds unrelated types or functions.

### Rules

- Implementation files must not mix responsibilities. When a change adds a new responsibility, create a new file (and a new folder if it is a new area) instead of extending an unrelated file.
- An implementation file must not have the same name as its containing folder (for example, avoid `config/config.rs`); use a name that describes the responsibility instead (for example, `config/user_config.rs`).
- Tests for a responsibility live in the same folder as a separate `*_tests.rs` file.

### Module files (`mod.rs`)

- Every module folder uses a `mod.rs` file. Do not use the `folder.rs` style.
- `mod.rs` only declares submodules (`mod`, `pub mod`) and re-exports (`pub use`). It declares test files with `#[cfg(test)]`.
- `mod.rs` has no implementation logic, because it is not an implementation file.

## Module Responsibilities

| Location | Responsibility |
|---|---|
| `sabu_core/` | CSV parsing, business logic, analytics, state. No GUI code. |
| `src/commands/` | Command pattern handlers for all actions. |
| `src/i18n/` | JSON translation loader and manager. |
| `src/ui/` | `egui`/`eframe` interface components. |

Do not mix these responsibilities.