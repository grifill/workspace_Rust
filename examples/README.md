# Install Rust via RustUp PPA
____

## Refreshing and Upgrading Your Ubuntu System Before Rust Installation

'''
sudo apt-get update && sudo apt-get upgrade
'''

## Install Initial Packages Required for Rust Installation

'''
sudo apt install curl build-essential gcc make -y
'''

## Download Rust Installation Script

'''
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
'''

## Activating the Rust Environment

'''
source "$HOME/.cargo/env"
'''

## Confirm Successful Rust Installation

'''
rustc -V
'''

# Create a Test Rust Application
____

## Create a Rust Workspace

'''
mkdir ~/rust-projects
'''

## Create the Rust Source File

'''
cd rust-projects && nano helloworld.rs
'''

## Writing the “Hello, World!” Program with Rust

'''
fn main() {
    println!("Hello World, this is a test provided by LinuxCapable.com");
}
'''

## Compile Test Rust Program

'''
rustc helloworld.rs
'''

## Running the Rust Program

'''
./helloworld
'''

# Managing Rust On Your System
____

## Update Rust

'''
rustup update
'''

## Remove Rust

'''
rustup self uninstall
'''
____



