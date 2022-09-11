# Modular OS Kernel

## Description

A kernel based on the idea of an exokernel, written in 64-bit Rust to be used in the Modular OS project.

## Setup

1. Install cargo via [rustup](https://www.rust-lang.org/tools/install). Make sure rustup is updated to 1.62 or later via `rustup update`
2. Install the platform: `rustup target add x86_64-unknown-none`
3. Install the nightly toolchain for it: `rustup toolchain install nightly --target=x86_64-unknown-none`

## Build Individually

```
cargo +nightly \
    rustc --release --target=x86_64-unknown-none -- \
    -C code-model=kernel -Z plt=y
```

The built kernel will be in `./target/x86_64-unknown-none/release/libcyub_os_kernel.a`

