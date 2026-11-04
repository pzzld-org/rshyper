# Quickstart Guide

***

Welcome to the quickstart guide for `rshyper`, a hypergraph implementation for Rust. This guide will help you get started with building and running the project.

## Table of Contents

- [Prerequisites](#prerequisites)
  - [Setup Rust](#setup-rust)
- [Building from the source](#building-from-the-source)

## Prerequisites

Before you begin, ensure you have the following prerequisites installed on your system:

- [Rust](https://www.rust-lang.org/) (version 1.85 or later)

Optionally, you may also want to install the following tools:

- [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) - A utility designed to streamline the installation of Rust binaries.

### Setup Rust

Ensure you have the latest version of Rust installed. You can install Rust using [rustup](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, I always recommend ensuring that rustup is updated to the latest version:

```bash
rustup update
```

And to add the latest nightly toolchain, which is often useful for development:

```bash
rustup toolchain install nightly
```

#### Adding additional targets

If necessary, add the `wasm32-*` target(s) if you plan to compile for WebAssembly:

```bash
rustup target add wasm32-unknown-unknown wasm32-p1 wasm32-p2
```

## Building from the source

Start by cloning the repository:

```bash
git clone https://github.com/pzzld-org/rshyper.git -b main --depth 1
```

Then, navigate to the project directory:

```bash
cd rshyper
```

Once you're in the project directory, you can build the project using `cargo`:

```bash
cargo build --workspace --release --all-features
```

Or, if you want to run the tests, you can use:

```bash
cargo test --workspace --release --all-features
```
