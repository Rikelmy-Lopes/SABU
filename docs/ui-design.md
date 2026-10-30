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

- The interface supports a **dark theme** and a **light theme**.
- Both themes use simple colors.
- The selected theme is saved in `config.json`, in `%LOCALAPPDATA%\Sabu`.

## Text and Language

- No user-facing text is hardcoded in `ui`. Labels, messages, and tooltips come from the `i18n` dictionary loader.
- Only Brazilian Portuguese (`pt-BR`) is supported. English or any other language may be added in the future.
- The UI must be built to allow multiple languages.
- If a key is missing in the active language, the `pt-BR` text is shown.
- Numbers, dates, and plurals follow the format of the active language.
- The selected language is saved in `config.json`.

## Error Display

- Follow the error handling rules in [architecture.md](architecture.md#error-handling). The UI must not crash.
- Error messages shown to the user also come from the `i18n` dictionary.

## Goal

The interface must be fast and intuitive, even with large volumes of data.