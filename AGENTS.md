# AGENTS.md — AI Agent Guidelines (SABU)

This document contains the project overview, architectural constraints, sample data reference, and development guidelines for any AI Agent working on the **SABU** codebase.

---

## 1. Project Overview

- **Project Name:** SABU (Sistema de Análise de Boletim de Urnas — Ballot Box Bulletin Analysis System)
- **Primary Language:** Rust (version 1.99.x)
- **Target Platform:** Windows. The application is built to run on Windows.
- **Product Name:** Sabu
- **Distribution:** The final `.exe` must be an installer executable, built with `cargo-wix` (WiX Toolset).
- **Installation:** Per-user installation (no administrator rights required) in `%LOCALAPPDATA%\Programs\Sabu`.
- **Graphical User Interface (GUI):** `egui` (via `eframe`)
- **Objective:** Process, analyze, and display large volumes of data extracted from Ballot Box Bulletins (CSV format) in an extremely fast, intuitive, and testable manner.

---

## 2. Architecture and Design Principles

### 2.1. Core / GUI Decoupling

- `core` is a **separate crate** inside a Cargo workspace. Its package name is `sabu_core`, because `core` conflicts with Rust's built-in `core` crate.
- **Golden Rule:** The `sabu_core` crate **must not** depend on any user interface components (`egui`, rendering, or visual layout logic). Its `Cargo.toml` must never list `egui` or `eframe`.
- The `core` is strictly responsible for CSV parsing, business logic, analytics data structures, and state management.
- The GUI layer (`egui`) acts purely as a consumer of `core`, interacting strictly via exposed APIs and command execution.

### 2.2. Command Pattern

- **Everything is a Command:** Every user action, state mutation, or application feature must be encapsulated as a structure conforming to a unified Command interface (e.g., `trait Command`).
- The GUI dispatches commands to an execution queue or handler rather than mutating core state directly.
- **Mandatory Benefits to Preserve:**
  - Asynchronous background execution without freezing the main UI thread.
  - Standalone testability for all application actions without needing an instantiated UI or headless display context.

### 2.3. High Performance & Large-Scale CSV Processing

- **Performance First:** The application handles large CSV files originating from electoral systems.
- Use concurrency, parallel streaming, and multi-threading wherever appropriate (e.g., leveraging `rayon`).
- Avoid unnecessary heap allocations, deep copies, and blocking synchronous I/O operations on the main UI thread.

### 2.4. Internationalization (i18n)

- **Supported Languages:** Only Brazilian Portuguese (`pt-BR`) is supported. English or any other language may be added in the future. The program must be built to allow multiple languages, without code changes to add a new one.
- **Default Language:** `pt-BR`.
- **Translation Storage:** Translations are stored in JSON files (e.g., `locales/pt-BR.json`). Each future language is a new JSON file in `locales/`.
- **Rule:** No user-facing text strings may be hardcoded in the source code (`core`, `commands`, or `ui`). All text labels, messages, and tooltips must resolve through the `i18n` dictionary loader.
- **Missing Keys:** If a translation key is missing in the active language, the application must fall back to the Portuguese (`pt-BR`) text.
- **Number, Date, and Plural Formatting:** Numbers, dates, and plurals must follow the format of the active application language (currently `pt-BR`).

### 2.5. Language for Code Comments and Documentation

- All code comments and documentation (including doc comments and README files) must be written **entirely in English**.
- This rule does not apply to user-facing text, which must follow the i18n rules in section 2.4.

### 2.6. Theme

- The interface must support a dark theme and a white theme.
- Both themes must use simple colors.

### 2.7. Error Handling

- Never use `unwrap()` or `panic!` (including `expect()`) in the code.
- All fallible operations must return `Result` and handle errors explicitly.

### 2.8. Logging

- The application must write logs to a `logs.log` file inside `%LOCALAPPDATA%\Sabu`.

### 2.9. Configuration

- The application must save its configuration (language, theme, etc) in a `config.json` file inside `%LOCALAPPDATA%\Sabu`.

### 2.10. Single Responsibility Principle (SRP)

- Each responsibility must be placed in its own folder and its own file.
- Follow the Single Responsibility Principle (SRP) strictly: a file, module, struct, or function must have only one reason to change.
- Do not put unrelated responsibilities in the same file or folder.

---

## 3. Data Specification & Context

- **Sample Data:** A small sample dataset is available at `example/data.csv`. Use this file for understanding data structure, schema validation, and unit/integration testing.
- **Data Schema Reference:** Refer to `BOLETIM-URNA-README.md` for complete documentation on CSV columns, data types, and semantic definitions of electoral fields.
- **Protected Files:** Never change `example/data.csv` or `BOLETIM-URNA-README.md`. They are read-only references.
- **File Encoding:** CSV files use **Latin-1 (ISO-8859-1)** encoding. The parser must decode them as Latin-1 (e.g., using `encoding_rs`) and convert the text to UTF-8 internally. Tests must include accented characters (e.g., `ç`, `ã`, `é`).

---

## 4. Testing and Quality Guidelines

1. **Regression Prevention:**
   - Every new feature, behavior change, or bug fix **MUST** include automated unit or integration tests.
2. **Test Scope:**
   - Test `Command` implementations and `core` functions independently.
   - Use test fixtures based on `example/data.csv` or minimal mock CSV strings for parser and analyzer tests.
3. **Test Location:**
   - Tests must be written in separate files, in the same folder as the code they test.
   - Do not write inline tests (`#[cfg(test)] mod tests { ... }`) inside source files.
4. **Mandatory Quality Checks:**
   - Ensure `cargo check`, `cargo test`, and `cargo clippy` pass with zero warnings or errors before finalizing any modification.

---

## 5. Expected Code Structure

```text
sabu/
├── Cargo.toml               # Workspace root and application package
├── sabu_core/               # Separate crate: business logic, CSV parser, analytics engine (GUI independent)
│   ├── Cargo.toml
│   └── src/
├── example/
│   └── data.csv             # Sample dataset for testing and reference
├── locales/
│   ├── pt-BR.json           # Default Portuguese translations
│   └── ...                  # Future languages (one JSON file per language)
├── src/
│   ├── main.rs
│   ├── commands/            # Action handlers implementing the Command pattern
│   ├── i18n/                # JSON translation loader and manager
│   └── ui/                  # Graphical interface components (egui/eframe)
└── BOLETIM-URNA-README.md   # Data dictionary and column definitions
```

---

## 6. Instructions for AI Agents

When implementing features or fixing bugs in **SABU**:

1. **Maintain Architectural Boundaries:** Determine whether changes belong in `sabu_core`, `commands`, `i18n`, or `ui`. Do not mix concerns. Place each responsibility in a separate folder and file, following SRP (section 2.10).
2. **Encapsulate Actions:** Implement all user actions as discrete `Command` structs.
3. **Write Tests Alongside Changes:** Never complete a task without associated automated tests proving correctness.
4. **Enforce i18n Rules:** When adding new UI strings, update `pt-BR.json` immediately using consistent key naming conventions.
5. **Verify Performance:** Ensure CSV parsing and data aggregation routines maintain high performance and never block the main event loop.
6. **Use Clear Names:** Use clear, descriptive names and follow all Rust naming conventions as needed (`snake_case` for functions, variables, and modules; `UpperCamelCase` for types and traits; `SCREAMING_SNAKE_CASE` for constants).
7. **Keep `AGENTS.md` Updated:** When the project changes, update `AGENTS.md` with important information only.
8. **Ask When Unclear:** If something is unclear, ask. Do not guess.

---

## 7. Related Documentation

- [readme.md](readme.md) — Project overview, structure, and runtime files.
- [docs/architecture.md](docs/architecture.md) — Architecture and design principles.
- [docs/development.md](docs/development.md) — Development rules, testing, and quality checks.
- [docs/ui-design.md](docs/ui-design.md) — User interface guidelines.