# Status

What was already done in SABU. Agents must read this file before starting and update it after finishing.

## Done

- [x] Project rules and documentation written (`AGENTS.md`, `architecture.md`, `development.md`, `ui-design.md`).
- [x] `mod.rs` module style defined.
- [x] SRP responsibility and granularity rules defined.

## In progress

- (none)

## Planned

Do these in order. Move an item to "In progress" when you start it, and to "Done" when it is finished.

- [ ] Create the Cargo workspace: root application package and `sabu_core` crate.
- [ ] Define the `Command` trait, the command ids, and the execution queue (`commands/`).
- [ ] Create the `i18n` loader and `locales/pt-BR.json` (with `pt-BR` fallback).
- [ ] Create logging (`logs.log`) and configuration (`config.json`) in `%LOCALAPPDATA%\Sabu`.
- [ ] Create the CSV parser in `sabu_core` (Latin-1 to UTF-8), tested with `example/data.csv`.
- [ ] Create the analytics data structures and state in `sabu_core`.
- [ ] Create the `egui` interface with dark and light themes (`ui/`).
- [ ] Create the installer with `cargo-wix`.