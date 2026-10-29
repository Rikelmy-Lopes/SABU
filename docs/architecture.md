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

- Active language: `pt-BR` (default). It is the only active and maintained translation.
- The i18n layer supports multiple bundled languages. Each locale file is embedded through the registry in `src/i18n/language_catalog/bundled_languages.rs`.
- The translator loads every registered file into a language-to-dictionary map and falls back to `pt-BR` when a key or language is unavailable.
- The interface lists only languages returned by `Translator::available_languages()`.
- Adding a language requires a complete locale file, its `Language` metadata, and a registry entry. Until then, the language is neither loaded nor shown in the menu.
- No user-facing text may be hardcoded in `core`, `commands`, or `ui`. All text goes through the `i18n` dictionary loader.
- If a key is missing in the active language, the application uses the `pt-BR` text.
- Numbers, dates, and plurals follow the format of the active language (`pt-BR` or `en`).
- **Only `locales/pt-BR.json` may be created or updated now. Do not create or modify `locales/en.json` or any other locale file until that language is explicitly implemented.**

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

## Single Responsibility Principle (SRP)

Every responsibility has its own folder and its own file.

- A folder groups exactly one responsibility.
- Every folder has a `mod.rs` that contains only wiring: `mod` declarations, re-exports, and test module declarations.
- Implementation files hold one responsibility each. A new responsibility means a new folder and file, not a larger existing file.
- An implementation file must not share the folder's name (for example, avoid `config/config.rs`); use a descriptive name such as `config/user_config.rs`.
- Tests live beside the code they test in a separate `*_tests.rs` file, declared from the folder's `mod.rs`.

The current tree follows this rule:

```text
sabu_core/src/
├── lib.rs
├── analytics/    # aggregate.rs, dataset_analytics.rs, vote_group.rs,
│                 # votes_by_cargo.rs, votes_by_partido.rs, votes_by_votavel.rs
├── config/       # user_config.rs, language.rs, theme.rs
├── dataset/      # ballot_dataset.rs, summary.rs
├── error/        # core_error.rs
├── model/        # bulletin_record.rs
├── parser/       # columns.rs, csv_parser.rs, decode.rs, numbers.rs
├── paths/        # app_paths.rs
└── state/        # app_state.rs

src/
├── main.rs
├── startup/      # application.rs, configuration.rs, logging.rs,
│                 # startup_error.rs, window.rs
├── commands/
│   ├── command/        # action.rs
│   ├── history/        # command_history.rs
│   ├── load_dataset/   # load_command.rs
│   ├── clear_dataset/  # clear_command.rs
│   ├── set_language/   # language_command.rs
│   ├── set_theme/      # theme_command.rs
│   ├── dispatch/       # command_dispatcher.rs, command_event.rs,
│   │                   # dispatcher_error.rs, request.rs, worker.rs
│   └── test_support.rs
├── i18n/
│   ├── dictionary/        # translation_map.rs
│   ├── error/             # i18n_error.rs
│   ├── error_key/         # error_translation.rs
│   ├── language_catalog/  # bundled_languages.rs
│   ├── number_formatter/  # locale_number.rs
│   └── translator/        # text_resolver.rs
└── ui/
    ├── analytics_cache/  # cached_analytics.rs, table_cache.rs
    ├── content_panel/    # content_area.rs
    ├── file_bar/         # file_path_bar.rs
    ├── menu_bar/         # application_menu.rs
    ├── sabu_app/         # main_window.rs
    ├── status_bar/       # status_line.rs
    ├── status_message/   # message.rs
    ├── summary_grid/     # summary_table.rs
    ├── theme/            # visuals.rs
    ├── top_bar/          # header_panel.rs
    └── vote_table/       # vote_list.rs
```

## Module Responsibilities

| Location | Responsibility |
|---|---|
| `sabu_core/` | CSV parsing, business logic, analytics, configuration, and state. No GUI code. |
| `src/startup/` | Path discovery, logging, configuration loading, and window creation. |
| `src/commands/` | Command trait, individual commands, undo/redo history, and the worker dispatcher. |
| `src/i18n/` | Translation dictionaries, lookup fallback, error keys, and locale-aware formatting. |
| `src/ui/` | `egui`/`eframe` components. `SabuApp` only orchestrates them. |

Do not mix these responsibilities. Each responsibility below the top-level
folders must also have its own folder and file.

