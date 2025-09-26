# Install Rust via RustUp PPA
____

## Refreshing and Upgrading Your Ubuntu System Before Rust Installation

```bash
    sudo apt-get update && sudo apt-get upgrade
```

## Install Initial Packages Required for Rust Installation

```bash
    sudo apt install curl build-essential gcc make -y
```

## Download Rust Installation Script

```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Activating the Rust Environment

```bash
    source "$HOME/.cargo/env"
```

## Confirm Successful Rust Installation

```bash
    rustc -V
```

# Create a Test Rust Application
____

## Create a Rust Workspace

```bash
    mkdir ~/rust-projects
```

## Create the Rust Source File

```bash
    cd rust-projects && nano hello.rs
```

## Writing the “Hello, World!” Program with Rust

```rust
    fn main() {
    println!("Hello World, this is a test provided by grifill");
    }
```

## Compile Test Rust Program

```bash
    rustc hello.rs
```

## Running the Rust Program

```bash
    ./hello
```

# Managing Rust On Your System
____

## Update Rust

```bash
    rustup update
```

## Remove Rust

```bash
    rustup self uninstall
```
____

