# Learning Project - GPU Programming with Rust

This is your self-contained learning project for experimenting with rust-gpu!

## Structure

```
learning-project/
├── shader/    - Your GPU shader code (runs on the GPU)
└── runner/    - Host application (runs on the CPU, displays your shader)
```

## How It Works

1. **Shader (`shader/`)**: Contains your GPU code written in Rust
   - `src/lib.rs`: Your vertex and fragment shaders
   - Compiles to SPIR-V (the GPU binary format)

2. **Runner (`runner/`)**: A window application that displays your shader
   - `src/main.rs`: Creates a window and renders your shader
   - `build.rs`: Compiles the shader during the build process

## Running Your Shader

From the `learning-project` directory:

```bash
cargo run -p learning-runner
```

This will:
1. Compile your shader to SPIR-V
2. Build the runner application
3. Open a window showing your shader!

## Modifying Your Shader

Edit `shader/src/lib.rs` to change what your shader does:

- **Change the color**: Modify the `vec4` in `main_fs`
  ```rust
  *output = vec4(1.0, 0.0, 0.0, 1.0);  // red
  *output = vec4(0.0, 1.0, 0.0, 1.0);  // green
  *output = vec4(0.2, 0.3, 1.0, 1.0);  // blue (current)
  ```

- **Change the triangle shape**: Modify the vertex positions in `main_vs`

## Learning Resources

- Shader code explanation: See comments in `shader/src/lib.rs`
- Main rust-gpu docs: https://rust-gpu.github.io/rust-gpu/book/
- Example shaders: `../examples/shaders/`

## Troubleshooting

If you get build errors:
- Make sure you're using the correct Rust toolchain (see `../rust-toolchain.toml`)
- Try `cargo clean` and rebuild
- Check that all dependencies are properly set up in the workspace

Happy shader coding!
