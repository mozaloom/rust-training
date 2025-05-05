# Rust Hello Marco

A simple Rust CLI application that demonstrates how to build command-line tools with Rust.

## Project Structure

```
hello-marco/
├── Cargo.toml       # Project configuration and dependencies
├── Makefile         # Build automation
├── src/
│   ├── lib.rs       # Library code
│   └── main.rs      # Application entry point
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable version)

## Setup

1. Update your Rust installation:
   ```bash
   rustup install stable
   rustup default stable
   rustc --version
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Usage

Run the application with:
```bash
cargo run -- play --name "Marco"
```

## Dependencies

This project uses [clap](https://github.com/clap-rs/clap) for command-line argument parsing.

Add dependencies to `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

## Development

Install clippy for linting:
```bash
rustup component add clippy
```

## License

See the [LICENSE](LICENSE) file for details.
