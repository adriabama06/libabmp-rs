# libabmp-rs

A lightweight Rust library for reading and writing BMP files, implemented from scratch without external dependencies.

> **Note:** This is a hand-written Rust rewrite of [libabmp](../libabmp), a pure C library. The original C implementation was reimplemented in Rust from scratch — no automatic conversion tools were used.

## Features

- Supports only 24-bit BMP formats
- Pure Rust implementation (no external libraries)
- Direct Read/Write from memory or disk (depends on the functions you use)

## Build Instructions

### All Platforms

```bash
cargo build
```

### Run tests

```bash
cargo test
```

### Example Usage

```bash
cd example
cargo run
```

The example generates `output.bmp` using sample images from `samples/`.

## Project Structure

- `src/`: Library implementation
  - `abitmap.rs`: Core bitmap types and header structures
  - `create.rs`: Bitmap creation utilities
  - `read.rs`: In-memory BMP reading
  - `write.rs`: In-memory BMP writing
  - `file_read.rs`: File-based BMP reading
  - `file_write.rs`: File-based BMP writing
  - `get.rs`: Pixel position helper functions
  - `print.rs`: Pixel manipulation utilities
- `example/`: Demonstrates library usage
- `samples/`: Input/output sample images

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
