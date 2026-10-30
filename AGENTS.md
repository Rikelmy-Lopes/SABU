# AGENTS.md — AI Agent Guidelines (SABU)

Short guide for AI agents working on **SABU**. Details live in the linked docs. Read them before changing code.

---

## 1. Project Overview

- **Name:** SABU (Sistema de Análise de Boletim de Urnas — Ballot Box Bulletin Analysis System)
- **Language:** Rust (1.99.x)
- **Platform:** Windows
- **GUI:** `egui` (via `eframe`)
- **Objective:** Process, analyze, and display large volumes of Ballot Box Bulletin data (CSV) in a fast, intuitive, and testable way.
- **Distribution and install:** see [README.md](README.md) and [docs/architecture.md](docs/architecture.md).

---

## 2. Key Rules

Full rules are in the docs. These are the ones agents must never break:

- `sabu_core` never depends on `egui` or `eframe`. → [architecture.md](docs/architecture.md#core--gui-decoupling)
- Every action is a `Command` with a unique, stable id. → [architecture.md](docs/architecture.md#command-pattern)
- No `unwrap()`, `panic!`, or `expect()` in production code. Return `Result`. Tests may use them. → [architecture.md](docs/architecture.md#error-handling)
- No hardcoded user-facing text. Use `i18n` and `locales/pt-BR.json`. → [architecture.md](docs/architecture.md#internationalization-i18n)
- Never block the main UI thread. → [architecture.md](docs/architecture.md#performance)
- Each responsibility has its own folder and file (SRP). Every module folder uses `mod.rs`. → [architecture.md](docs/architecture.md#single-responsibility-principle-srp)
- Comments and documentation are written in English. → [development.md](docs/development.md#language-rules)
- Tests go in separate files, never inline. → [development.md](docs/development.md#testing)
- Never change `example/data.csv` or `docs/BOLETIM-URNA-README.md`. → [development.md](docs/development.md#data-files)

---

## 3. Code Structure

See [README.md](README.md#project-structure).

---

## 4. Instructions for AI Agents

Follow the [checklist for changes](docs/development.md#checklist-for-changes). It starts with reading [docs/STATUS.md](docs/STATUS.md).

---

## 5. Related Documentation

- [README.md](README.md) — Project overview, structure, and runtime files.
- [docs/architecture.md](docs/architecture.md) — Architecture and design principles.
- [docs/development.md](docs/development.md) — Development rules, testing, and quality checks.
- [docs/ui-design.md](docs/ui-design.md) — User interface guidelines.
- [docs/STATUS.md](docs/STATUS.md) — What is already done.