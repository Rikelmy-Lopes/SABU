# SABU

**SABU** (*Sistema de Análise de Boletim de Urnas* — Ballot Box Bulletin Analysis System) processes, analyzes, and displays large volumes of data from Ballot Box Bulletins in CSV format. It is built to be extremely fast, intuitive, and testable.

## Overview

| Item | Value |
|---|---|
| Product name | Sabu |
| Language | Rust (1.99.x) |
| Platform | Windows |
| GUI | `egui` (via `eframe`) |
| Installer | `.exe` built with `cargo-wix` (WiX Toolset) |
| Install location | `%LOCALAPPDATA%\Programs\Sabu` (per user, no administrator rights) |
| Default language | Brazilian Portuguese (`pt-BR`), the only active translation |
| Other languages | Supported through the bundled-language registry; none active yet |

## Getting Started

SABU targets Windows with Rust 1.99.x.

```text
cargo run                                                    # launch the application
cargo test --workspace                                       # run all tests
cargo clippy --workspace --all-targets -- -D warnings        # lint
```

To load data, type the CSV path in the top bar and press **Abrir**, or drop a
CSV file onto the window. The current interface shows a summary, votes by
office, votes by votable, and votes by party, with undo/redo, language, and
theme controls.

## Project Structure

SABU follows the Single Responsibility Principle (SRP) strictly: every
responsibility has its own folder and file. Each `mod.rs` contains only module
wiring and re-exports; implementation lives in sibling files.

```text
sabu/
├── Cargo.toml
├── sabu_core/               # Business logic, CSV parser, analytics (GUI independent)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── analytics/
│       ├── config/
│       ├── dataset/
│       ├── error/
│       ├── model/
│       ├── parser/
│       ├── paths/
│       └── state/
├── example/
│   └── data.csv             # Sample dataset (read-only)
├── locales/
│   ├── pt-BR.json           # Active Portuguese translations (update this)
│   └── en.json              # Reserved; not active (do not modify yet)
├── src/
│   ├── main.rs              # Entry point only
│   ├── startup/             # Paths, logging, configuration, and window
│   ├── commands/            # Command trait, commands, history, dispatcher
│   ├── i18n/                # Dictionaries, lookup, error keys, formatting
│   └── ui/                  # Interface components (egui/eframe)
├── docs/
│   ├── architecture.md
│   ├── development.md
│   └── ui-design.md
├── AGENTS.md                # Guidelines for AI agents
└── BOLETIM-URNA-README.md   # Data dictionary and column definitions (read-only)
```

## Data

- A sample dataset is in `example/data.csv`.
- The columns and their meaning are described in `BOLETIM-URNA-README.md`.
- CSV files use Latin-1 (ISO-8859-1) encoding. SABU converts them to UTF-8 internally.

## Languages

Only Brazilian Portuguese (`pt-BR`) is active. The i18n layer supports adding
more languages through `src/i18n/language_catalog/bundled_languages.rs`, but a
language must be fully translated and registered before it is loaded or shown in
the interface.

Only `locales/pt-BR.json` may be created or updated now. Do not create or modify
`locales/en.json` or any other locale file until that language is explicitly
implemented.

## Files Created at Runtime

Both files are stored in `%LOCALAPPDATA%\Sabu`:

- `logs.log` — application logs.
- `config.json` — configuration (language, theme, etc).

## Documentation

- [Architecture](docs/architecture.md)
- [Development guide](docs/development.md)
- [UI design](docs/ui-design.md)
- [AGENTS.md](AGENTS.md) — guidelines for AI agents working on this project
