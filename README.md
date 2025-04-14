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


## for new rust project

1. Create a new Rust project:
   ```bash
   cargo new <project_name>

   or

   cargo init --name <project_name>
   ```

2. Navigate to the new project directory:
   ```bash
   cd <project_name>
   ```

3. Copy the make file to the new directory.
   ```bash
   cp /path/to/Makefile .
   ```

4. Start building your Rust project!



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

