# Development Guide

Rules and practices for working on SABU.

## Language Rules

- All code comments and documentation (doc comments, README files) must be written **entirely in English**.
- User-facing text follows the i18n rules in [architecture.md](architecture.md#internationalization-i18n).

## Code Rules

- Use clear, descriptive names.
- Follow Rust naming conventions:
  - `snake_case` for functions, variables, and modules.
  - `UpperCamelCase` for types and traits.
  - `SCREAMING_SNAKE_CASE` for constants.
- Never use `unwrap()` or `panic!` (including `expect()`).
- Return `Result` from every operation that can fail, and handle errors explicitly.
- Implement every user action as a separate `Command` struct.
- Give every command a unique id, as a `&str` constant (for example, `FILE_OPEN` = `"file.open"`). Never rename an existing id. See [Command IDs](architecture.md#command-ids).
- Follow the Single Responsibility Principle (SRP) strictly:
  - Place each responsibility in a separate folder and a separate file.
  - A file, module, struct, or function must have only one reason to change.
- Keep `sabu_core` free of GUI dependencies.

## Data Files

- `example/data.csv` is a small sample dataset. Use it to understand the data structure, validate the schema, and write tests.
- `docs/BOLETIM-URNA-README.md` documents the CSV columns, data types, and meaning of the fields.
- **Never change** `example/data.csv` or `docs/BOLETIM-URNA-README.md`. They are read-only.
- CSV files use Latin-1 (ISO-8859-1). Decode them (for example, with `encoding_rs`) and convert to UTF-8 internally.

## Adding UI Text

1. Add the new key to `locales/pt-BR.json` right away, using consistent key names.
2. Use the key through the `i18n` loader. Never hardcode the text.

## Testing

- Every new feature, behavior change, or bug fix **must** include automated unit or integration tests.
- Test `Command` implementations and `core` functions independently.
- Use fixtures based on `example/data.csv`, or small mock CSV strings, for parser and analyzer tests.
- Tests must include accented characters (for example, `ç`, `ã`, `é`).
- Write tests in separate files, in the same folder as the code they test.
- Do not write inline tests (`#[cfg(test)] mod tests { ... }`) inside source files.

## Quality Checks

Before finishing any change, these must pass with zero warnings and zero errors:

```text
cargo check
cargo test
cargo clippy
```

## Checklist for Changes

1. Decide where the change belongs: `sabu_core`, `commands`, `i18n`, or `ui`.
2. Implement actions as `Command` structs, each with a unique id.
   Place each responsibility in a separate folder and file (SRP).
3. Add automated tests.
4. Update `pt-BR.json` for new UI text.
5. Check that performance is kept and the main event loop is not blocked.
6. Run the quality checks.
7. Update `AGENTS.md` if the project changed (important information only).
8. If something is unclear, ask. Do not guess.