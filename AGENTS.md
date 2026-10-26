# RustJava Agent Guidelines

## Build/Test Commands
- **Build**: `cargo build` (workspace), `cargo build -p <crate>` (single crate)
- **Test all**: `cargo test`
- **Single test**: `cargo test <test_name>` or `cargo test -p <crate> <test_name>`
- **Format**: `cargo fmt`, **Lint**: `cargo clippy`

## Code Style
- **Edition**: Rust 2024, `#![no_std]` for core crates (use `alloc` crate)
- **Line width**: 150 chars (rustfmt.toml)
- **Indent**: 4 spaces, LF line endings
- **Imports**: Group `alloc`/`core` first, then external crates, then `crate::` local imports
- **Naming**: snake_case for functions/files, PascalCase for types/traits
- **Error handling**: Use `Result<T>` (alias for `result::Result<T, JavaError>`), never panic in library code
- **Async**: Use `#[async_trait::async_trait]` for async trait methods, `#[tokio::test]` for async tests

## Project Structure
- `jvm/` - Core JVM implementation (`#![no_std]`)
- `jvm-bytecode/` - JVM class and bytecode implementation
- `rustjava-runtime/` - Java standard library implementations
- `classfile/` - Class file parser
- `jvm-class-proto/` - Java class prototypes
- `jvm-types/` - Shared JVM metadata types
- `test-utils/` - Shared test utilities

## Testing Boundaries
- Keep `rustjava-runtime/tests/classes` limited to Java standard library class and API behavior.
- Test JVM and interpreter semantics, including class initialization, bytecode execution, and monitor behavior, with compiled Java fixtures under `test-data/src` and expected output under `test-data`, executed by `tests/test_class.rs`.
- Add or extend fixtures consumed by the existing `test_class` runner rather than adding Rust tests that directly invoke JVM APIs. For linkage errors that Java source cannot express directly, compile the caller against one class version and replace the target class in the fixture; keep the source variants and reproducible build instructions alongside it.
- Do not place JVM core behavior tests in the `rustjava-runtime` standard library test tree.

## Compatibility Sources
- Implement Java compatibility from public specifications, Javadocs, and observable behavior tests. Do not consult or reproduce OpenJDK or other Java runtime implementation source code; keep the implementation independent to avoid licensing and provenance concerns.
