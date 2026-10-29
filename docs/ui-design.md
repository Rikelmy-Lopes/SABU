# UI Design

Guidelines for the SABU graphical interface.

## Technology

- GUI library: `egui` (via `eframe`).
- The UI code is in `src/ui/`.
- The UI is only a consumer of `sabu_core`. It uses exposed APIs and commands.

## Interaction with the Core

- The UI sends commands to an execution queue or handler.
- The UI never changes core state directly.
- Long operations (such as CSV processing) run in the background.
- The main UI thread must never be blocked or frozen.

## Themes

- The interface supports a **dark theme** and a **white theme**.
- Both themes use simple colors.
- The selected theme is saved in `config.json`, in `%LOCALAPPDATA%\Sabu`.

## Text and Language

- No user-facing text is hardcoded in `ui`. Labels, messages, and tooltips come from the `i18n` dictionary loader.
- Default language: Brazilian Portuguese (`pt-BR`). It is the only active and maintained translation.
- The i18n layer supports multiple bundled languages. The language menu lists only languages returned by `Translator::available_languages()`.
- Adding a language requires a complete locale file, its `Language` metadata, and a registry entry in `src/i18n/language_catalog/bundled_languages.rs`.
- If a key is missing in the active language, the `pt-BR` text is shown.
- Numbers, dates, and plurals follow the format of the active language.
- The selected language is saved in `config.json`.
- **Do not create or modify `locales/en.json` or any other locale file until that language is explicitly implemented. Only `pt-BR.json` may be updated now.**

## Error Display

- Errors are returned as `Result` and handled explicitly. The UI must not crash (`unwrap()` and `panic!` are not allowed).
- Error messages shown to the user also come from the `i18n` dictionary.

## Component Organization

The UI follows the project-wide Single Responsibility Principle: every
responsibility has its own folder and file, and `mod.rs` only wires the folder's
files together. `SabuApp` is orchestration only; it must not render components
or mutate core state directly.

| Folder | Responsibility |
|---|---|
| `ui/sabu_app/` | Window orchestration and state/command integration. |
| `ui/top_bar/` | Composes the menu bar and file bar. |
| `ui/menu_bar/` | Menus and the `MenuAction` value. |
| `ui/file_bar/` | CSV path input and open action. |
| `ui/status_bar/` | Status line rendering. |
| `ui/status_message/` | Status message value. |
| `ui/content_panel/` | Central content layout. |
| `ui/summary_grid/` | Dataset summary grid. |
| `ui/vote_table/` | Analytics vote tables. |
| `ui/analytics_cache/` | Bounded cache of precomputed analytics. |
| `ui/theme/` | Dark and light visual themes. |

Each component renders one responsibility and returns an action value when the
user interacts with it. The window decides what command to dispatch.

## Goal

The interface must be fast and intuitive, even with large volumes of data.
