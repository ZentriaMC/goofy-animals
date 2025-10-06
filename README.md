# Goofy Animals

[![Crates.io](https://img.shields.io/crates/v/goofy-animals)](https://crates.io/crates/goofy-animals)
[![License: MPL-2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://github.com/ZentriaMC/goofy-animals/blob/main/LICENSE)

A lightweight Rust library for generating fun, random names in the format `adjective-adjective-animal`.

## Overview

Goofy Animals generates names like:

- `healthy-frivolous-dove`
- `glorious-meager-polar-bear`
- `thankful-elastic-clownfish`

Perfect for:

- Container & machine names
- Test fixtures
- Temporary resources
- Development environments
- Anywhere you need a friendly, memorable identifier

## Features

- **No-std compatible** - Core functionality works without the standard library
- **Lightweight** - Uses compile-time string parsing for small binary size
- **Fast** - Efficient name generation with minimal overhead
- **Configurable** - Optional features for different use cases
- **Deterministic** - Support for seeded RNG for repeatable results

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
goofy-animals = "0.1.0"
```

## Usage

### Basic example

```rust
use goofy_animals::generate_name;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn main() {
    // Use a random seed
    let mut rng = ChaCha20Rng::from_os_rng();

    // Generate a random name
    let name = generate_name(&mut rng);
    println!("{}", name); // e.g., "vigilant-troubled-firefly"
}
```

### Deterministic names

```rust
use goofy_animals::generate_name;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn main() {
    // Use a fixed seed for deterministic output
    let mut rng = ChaCha20Rng::seed_from_u64(0x1337);

    let name = generate_name(&mut rng);
    assert_eq!(name, "dismal-outlying-moth");
}
```

### Just the parts

If you want the individual name components:

```rust
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use goofy_animals::generate_name_parts;

fn main() {
    let mut rng = ChaCha20Rng::from_os_rng();

    let (adj1, adj2, animal) = generate_name_parts(&mut rng);
    println!("First adjective: {}", adj1);
    println!("Second adjective: {}", adj2);
    println!("Animal: {}", animal);
}
```

## Feature flags

- `alloc` (default): Enables the `generate_name` function that returns a `String`
- `std`: _Used only for testing_
- `tracing`: Adds tracing instrumentation for debugging
- `examples`: Enables building the example binary `goofy-animal`

## Command line tool

With the `examples` feature enabled, you can build and run the simple CLI tool:

```bash
# Build with examples feature
cargo build --features=examples

# Run the binary
./target/debug/goofy-animal
# Output: handsome-modest-porcupine
```

Or you can install the CLI tool via `cargo install`:

```bash
cargo install --features=examples goofy-animals
```

## Development

This project uses a Nix flake for development environment setup. If you have Nix installed:

```bash
# Enter the development shell
nix develop
```

Or with direnv:

```bash
direnv allow
```

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](/LICENSE) file for details.

## Data

The library includes:

- 355 animal names
- 1300 adjectives

All words are in English.
