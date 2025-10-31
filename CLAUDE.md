# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

rust-gpu is a compiler backend that enables writing GPU shaders in Rust. It uses a custom `rustc` codegen backend (`rustc_codegen_spirv`) to compile Rust to SPIR-V (the Vulkan shader format). The project is a monorepo containing the compiler, standard library, build tools, examples, and tests.

## Critical Version Constraint

**IMPORTANT**: This project requires a very specific Rust nightly toolchain version. The current version is `nightly-2025-06-30` (see `rust-toolchain.toml`). This is strictly enforced because the codegen backend depends on rustc's internal APIs. Always use this exact version.

## Building and Running

### Build the compiler backend
```bash
cargo build --release
```
This builds the `rustc_codegen_spirv` compiler backend (the SPIR-V code generator).

### Run examples
```bash
# Run the wgpu example (most common)
cargo run --bin example-runner-wgpu

# Run the Vulkan/ash example
cargo run --bin example-runner-ash

# Run the CPU software renderer example
cargo run --bin example-runner-cpu
```

Examples consist of two parts:
1. **Shader crates** in `examples/shaders/` - GPU code compiled to SPIR-V
2. **Runner crates** in `examples/runners/` - CPU code that runs the shaders using various graphics backends (wgpu, ash/Vulkan, or pure CPU)

### Testing

```bash
# Run all tests
cargo test && cargo compiletest && cargo difftest

# Run only unit tests
cargo test

# Run compile tests (end-to-end SPIR-V compilation validation)
cargo compiletest

# Run differential tests (compares Rust vs WGSL shader outputs)
cargo difftest
```

#### Compile test filtering and blessing
```bash
# Run specific compile tests by path substring
cargo compiletest arch/u image

# Bless (update expected output for) compile tests
cargo compiletest --bless

# Test specific SPIR-V environments
cargo compiletest --target-env=vulkan1.1
cargo compiletest --target-env=vulkan1.1,spv.1.3
```

## Repository Architecture

### Core Crates (`crates/`)

- **`rustc_codegen_spirv`**: The main compiler backend that plugs into rustc via `-Z codegen-backend`. Translates Rust MIR to SPIR-V. Contains the linker, optimization passes, and SPIR-T integration.

- **`spirv-std`**: Standard library for GPU shaders. Provides the `#[spirv(..)]` attribute and APIs for accessing GPU resources (textures, buffers, etc.). This is what shader code imports.

- **`spirv-builder`**: Build tool for compiling shader crates. Used in `build.rs` scripts to automatically build shaders and make them available via environment variables (e.g., `env!("my_shader.spv")`).

- **`rustc_codegen_spirv-types`**: Shared types used between the compiler and runtime.

- **`rustc_codegen_spirv-target-specs`**: Target specification files for different SPIR-V environments (Vulkan 1.0/1.1/1.2, SPIR-V 1.3/1.5, etc.).

### Examples (`examples/`)

- **`shaders/`**: GPU shader crates (e.g., `sky-shader`, `simplest-shader`, `compute-shader`). These are `dylib` crates with `#![no_std]` that get compiled to SPIR-V.

- **`runners/`**: Host applications that execute shaders:
  - `wgpu/`: Cross-platform WebGPU-based runner (most commonly used)
  - `ash/`: Vulkan runner using the ash bindings
  - `cpu/`: Software renderer for basic testing

### Tests (`tests/`)

- **`compiletests/`**: End-to-end compilation tests using the `compiletest` framework. Tests that Rust code compiles to valid SPIR-V.

- **`difftests/`**: Differential tests that compare Rust shader output against WGSL reference implementations to ensure correctness.

### Compilation Pipeline

1. Shader crate (Rust) → `rustc_codegen_spirv` → SPIR-V modules
2. Per-function SPIR-V modules → Linker → Merged module
3. Merged module → Inlining, optimization, structurization → Final module(s)
4. Final SPIR-V → `spirv-opt` (optional) → `spirv-val` → Output `.spv` file

The linker uses [SPIR-T](https://github.com/rust-gpu/spirt) (an experimental IR framework) for structurization and advanced transformations.

## Key Technical Details

### Shader Crate Setup

Shader crates must be configured as:
```toml
[lib]
crate-type = ["dylib"]

[dependencies]
spirv-std = { version = "0.9" }
```

Shader code uses:
- `#![no_std]` attribute (no standard library)
- `#[spirv(fragment)]`, `#[spirv(vertex)]`, `#[spirv(compute)]` for entry points
- `spirv_std::spirv` attribute for shader-specific annotations

### Build Script Pattern

Projects using shaders typically use `spirv-builder` in `build.rs`:
```rust
use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SpirvBuilder::new("path/to/shader", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    Ok(())
}
```

The compiled shader is accessible via: `include_bytes!(env!("shader_name.spv"))`

### Codegen Arguments

Debug/development flags can be passed via `RUSTGPU_CODEGEN_ARGS` environment variable:
```bash
# Disable validation, dump post-link SPIR-V
RUSTGPU_CODEGEN_ARGS="--no-spirv-val --dump-post-link=./output" cargo run

# Show all available flags
RUSTGPU_CODEGEN_ARGS="--help" cargo run
```

Common flags:
- `--dump-post-link DIR`: Dump final SPIR-V before spirv-opt
- `--dump-pre-link DIR`: Dump all modules before linking
- `--no-spirv-val`: Skip SPIR-V validation
- `--no-spirv-opt`: Skip SPIR-V optimization
- `--dump-spirt-passes DIR`: Dump SPIR-T transformations

### Profile Configuration

The workspace Cargo.toml includes important profile settings:
- Release mode enables incremental compilation for faster iteration
- Build dependencies (including `rustc_codegen_spirv`) should be compiled in release mode to avoid extremely slow compilation
- Dev profile uses `codegen-units = 1` to work around MSVC 64Ki export limit

## Platform Support

Target specs available in `crates/rustc_codegen_spirv-target-specs/target-specs/`:
- `spirv-unknown-vulkan1.0/1.1/1.2`
- `spirv-unknown-spv1.3/1.5`
- And others

Choose based on your target environment's capabilities.

## Common Issues

- **Slow shader compilation**: Ensure build-dependencies are compiled in release mode (see profile settings in workspace Cargo.toml)
- **Toolchain version errors**: Must use exact nightly version specified in `rust-toolchain.toml`
- **Invalid SPIR-V**: Use `--dump-post-link` to debug, check if issue is before or after `spirv-opt`
- **Linker errors with MSVC**: Use `codegen-units = 1` in dev profile

## SPIR-T Integration

The linker uses SPIR-T for:
- CFG structurization (converting goto-style control flow to structured if/loop/switch)
- Handling OpPhi instructions
- Advanced legalization passes

SPIR-T is always enabled as of version 0.8.0. The pipeline: `SPIR-V → SPIR-T → transformations → SPIR-V`
