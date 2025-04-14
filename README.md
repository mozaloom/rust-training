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

A good starting point for a new Rust project.

## Getting Started

### Prerequisites

* Rust installed on your system
* Cargo package manager

### Installation

1. Update Rust to the latest stable version:
   ```bash
   rustup install stable
   rustup default stable
   rustc --version
   ```

2. Install Clippy for code linting:
   ```bash
   rustup component add clippy
   ```

### Adding Dependencies

Add the following dependencies to your `Cargo.toml` file:
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

### Building the Project

Build the project using Cargo:
```bash
cargo build
```

### Running the Project

Run the project with the following command:
```bash
cargo run -- play --name "Marco"
```

## Creating a New Rust Project

### Step 1: Create a New Project

Create a new Rust project using Cargo:
```bash
cargo new <project_name>
```

### Step 2: Navigate to the Project Directory

Navigate to the newly created project directory:
```bash
cd <project_name>
```

### Step 3: Move the Makefile

Move the Makefile to the new project directory:
```bash
cp /path/to/Makefile .
```

### Step 4: Start Building Your Project

Start building your Rust project!

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
      cargo run -- play --name "Marco"
   ```


## for new rust project

1. Create a new Rust project:
   ```bash
   cargo new <project_name>
   ```

2. Navigate to the new project directory:
   ```bash
   cd <project_name>
   ```

3. Move the make file to the new directory.
   ```bash
   cp /path/to/Makefile .
   ```

4. Start building your Rust project!



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

