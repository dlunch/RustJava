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
- `jvm_rust/` - Rust-based JVM interpreter
- `java_runtime/` - Java standard library implementations
- `classfile/` - Class file parser
- `java_class_proto/` - Java class prototypes
- `test_utils/` - Shared test utilities
