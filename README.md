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
| Supported language | Brazilian Portuguese (`pt-BR`) only |
| Future languages | English or others can be added later (the program is built for multiple languages) |

## Project Structure

```text
sabu/
├── Cargo.toml               # Workspace root and application package
├── sabu_core/               # Business logic, CSV parser, analytics (GUI independent)
│   ├── Cargo.toml
│   └── src/
├── example/
│   └── data.csv             # Sample dataset (read-only)
├── locales/
│   ├── pt-BR.json           # Default Portuguese translations
│   └── ...                  # Future languages (one JSON file per language)
├── src/
│   ├── main.rs
│   ├── commands/            # Command pattern handlers
│   ├── i18n/                # Translation loader and manager
│   └── ui/                  # Interface components (egui/eframe)
├── docs/
│   ├── architecture.md
│   ├── development.md
│   ├── ui-design.md
│   ├── STATUS.md
│   └── BOLETIM-URNA-README.md   # Data dictionary and column definitions (read-only)
└── AGENTS.md                # Guidelines for AI agents
```

## Data

- A sample dataset is in `example/data.csv`.
- The columns and their meaning are described in `docs/BOLETIM-URNA-README.md`.
- CSV files use Latin-1 (ISO-8859-1) encoding. SABU converts them to UTF-8 internally.

## Files Created at Runtime

Both files are stored in `%LOCALAPPDATA%\Sabu`:

- `logs.log` — application logs.
- `config.json` — configuration (language, theme, etc).

## Documentation

- [Architecture](docs/architecture.md)
- [Development guide](docs/development.md)
- [UI design](docs/ui-design.md)
- [Status](docs/STATUS.md) — what is already done
- [AGENTS.md](AGENTS.md) — guidelines for AI agents working on this project