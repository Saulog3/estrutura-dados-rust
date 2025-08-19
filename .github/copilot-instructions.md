# Copilot Instructions for estrutura-dados-rust

This repository contains exercises and study code for the subject of Data Structures using Rust. The project is organized for educational clarity and modularity, with each data structure or topic in its own binary file under `src/bin/`.

## Project Structure
- `src/main.rs`: Entry point for a general test program (minimal, not the main focus).
- `src/bin/`: Each file is a separate Rust binary for a specific topic or exercise. Examples:
  - `listas.rs`: Linked list exercises (currently uses Vec, can be extended).
  - `pilhas.rs`: Stack exercises using `Vec`.
  - `filas.rs`: Queue exercises using `VecDeque`.
  - `aula1-ex01.rs`, `aula1-ex02.rs`, `aula1-exemplos.rs`: Simpler exercises and examples.
- No shared library code; each binary is self-contained.

## Developer Workflows
- **Run a specific exercise:**
  ```
  cargo run --bin listas
  cargo run --bin pilhas
  cargo run --bin filas
  ```
- **Build all binaries:**
  ```
  cargo build
  ```
- **Add a new exercise:**
  - Create a new `.rs` file in `src/bin/`.
  - Use a unique filename; it will be available as `cargo run --bin <filename>`.

## Patterns & Conventions
- Each exercise is a standalone binary (no cross-binary imports).
- Use idiomatic Rust collections (`Vec`, `VecDeque`) for basic data structures.
- User input (when present) uses `std::io`.
- Print statements are used for all output and demonstration.
- No custom modules, traits, or advanced Rust features unless required by the exercise.
- File comments and function names are in Portuguese, matching the educational context.

## Integration & External Dependencies
- No external crates are used (only Rust standard library).
- No CI/CD, tests, or automation scripts are present.

## Examples
- To implement a new data structure, follow the style of `pilhas.rs` or `filas.rs`:
  - Define a `main()` function.
  - Use a standard collection for storage.
  - Demonstrate operations (push, pop, etc.) with print statements.

## Key Files
- `README.md`: Explains project purpose, structure, and usage.
- `src/bin/filas.rs`, `src/bin/pilhas.rs`, `src/bin/listas.rs`: Canonical examples for new exercises.

---
For any new code, match the simplicity and educational focus of existing files. Avoid introducing complexity unless required for a specific data structure or topic.
