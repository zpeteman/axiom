# Usage

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Building

```bash
cargo build --release
```

### Running

To see the available options, run:

```bash
./target/release/axiom --help
```

To display a fractal:

```bash
./target/release/axiom --mode fractal
```

To display a fractal with animation:

```bash
./target/release/axiom --mode fractal --animate
```

To display ASCII art:

```bash
./target/release/axiom --mode art
```

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```
