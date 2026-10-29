# Architecture

This document describes the architecture and design principles of SABU.

## Workspace

SABU is a Cargo workspace with two parts:

- **`sabu_core`** — a separate crate with the business logic.
- **Application package** (workspace root) — contains `commands`, `i18n`, and `ui`.

The crate is named `sabu_core` because the name `core` conflicts with Rust's built-in `core` crate.

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

- Clean support for Undo/Redo.
- Asynchronous background execution, without freezing the main UI thread.
- Tests for all actions without a UI or a headless display.

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

- Active language: `pt-BR` (default). `en` is planned for the future.
- Translations are stored in JSON files: `locales/pt-BR.json` and `locales/en.json`.
- No user-facing text may be hardcoded in `core`, `commands`, or `ui`. All text goes through the `i18n` dictionary loader.
- If a key is missing in the active language, the application uses the `pt-BR` text.
- Numbers, dates, and plurals follow the format of the active language (`pt-BR` or `en`).

## Error Handling

- Never use `unwrap()` or `panic!` (including `expect()`).
- All operations that can fail return `Result` and handle errors explicitly.

## Logging

Logs are written to `logs.log` in `%LOCALAPPDATA%\Sabu`.

## Configuration

Configuration (language, theme, etc) is saved in `config.json` in `%LOCALAPPDATA%\Sabu`.

## Distribution

- The final `.exe` is an installer built with `cargo-wix` (WiX Toolset).
- Installation is per user, in `%LOCALAPPDATA%\Programs\Sabu`, without administrator rights.

## Module Responsibilities

| Location | Responsibility |
|---|---|
| `sabu_core/` | CSV parsing, business logic, analytics, state. No GUI code. |
| `src/commands/` | Command pattern handlers for all actions. |
| `src/i18n/` | JSON translation loader and manager. |
| `src/ui/` | `egui`/`eframe` interface components. |

Do not mix these responsibilities.
