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
- Keep `sabu_core` free of GUI dependencies.
- **Apply the Single Responsibility Principle:** every responsibility gets its own folder and file.
  - `mod.rs` contains only wiring: `mod` declarations, re-exports, and test module declarations.
  - Implementation files must not mix responsibilities and must not share the folder's name.
  - A new responsibility requires a new folder and file, not a larger existing file.

## Data Files

- `example/data.csv` is a small sample dataset. Use it to understand the data structure, validate the schema, and write tests.
- `BOLETIM-URNA-README.md` documents the CSV columns, data types, and meaning of the fields.
- **Never change** `example/data.csv` or `BOLETIM-URNA-README.md`. They are read-only.
- CSV files use Latin-1 (ISO-8859-1). Decode them (for example, with `encoding_rs`) and convert to UTF-8 internally.

## Adding UI Text

1. Add the new key to `locales/pt-BR.json` right away, using consistent key names.
2. Use the key through the `i18n` loader. Never hardcode the text.
3. Do not create or modify `locales/en.json` or any other locale file until that language is explicitly implemented.

## Adding a Language

1. Add the language metadata in `sabu_core/src/config/language.rs`: variant, locale code, display-name key, and number formatting if needed.
2. Create `locales/<code>.json` with every key present in `locales/pt-BR.json`.
3. Register the new file in `src/i18n/language_catalog/bundled_languages.rs` with `include_str!`.
4. Add the language display-name key to every registered locale file, including `pt-BR.json`.
5. Confirm that the language appears in `Translator::available_languages()` and in the language menu.
6. Add tests for lookup, fallback, and number formatting.

Until a language passes all these steps, it must not be registered or offered in the interface. In particular, `locales/en.json` must not be created or modified before English is explicitly implemented.

## Testing

- Every new feature, behavior change, or bug fix **must** include automated unit or integration tests.
- Test `Command` implementations and `core` functions independently.
- Use fixtures based on `example/data.csv`, or small mock CSV strings, for parser and analyzer tests.
- Tests must include accented characters (for example, `ç`, `ã`, `é`).
- Write tests in separate files, in the same folder as the code they test.
- Do not write inline tests (`#[cfg(test)] mod tests { ... }`) inside source files.
- Declare each test module from its folder's `mod.rs` with `#[cfg(test)] #[path = "..."] mod ...;`.
- Shared test helpers belong in a dedicated file (for example, `commands/test_support.rs`) instead of being duplicated.

## Quality Checks

Before finishing any change, these must pass with zero warnings and zero errors:

```text
cargo check
cargo test
cargo clippy
```

## Checklist for Changes

1. Decide where the change belongs: `sabu_core`, `startup`, `commands`, `i18n`, or `ui`.
2. Create a separate folder and file for any new responsibility; keep `mod.rs` to wiring only.
3. Implement actions as `Command` structs.
4. Add automated tests in the folder they cover.
5. Update `pt-BR.json` for new UI text. Do not touch `en.json` or other locale files unless that language is being explicitly implemented.
6. Check that performance is kept and the main event loop is not blocked.
7. Run the quality checks.
8. Update `AGENTS.md` if the project changed (important information only).
9. If something is unclear, ask. Do not guess.
