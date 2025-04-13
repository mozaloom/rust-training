# Rust Training

A good starting point for a new Rust project.

## Usage


1. if it need an update 
   ```bash
      rustup install stable

      rustup default stable

      rustc --version

      cargo build
   ```


2. adding dependencies:
   ```bash

      [dependencies]
      clap = { version = "4.0", features = ["derive"] }

      rustup component add clippy

      cargo build
   ```

   
3. Run the project
   ```bash
      cargo run -- play --name "Marco"
   ```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

