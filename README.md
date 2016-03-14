# Edit locally, compile remotely

<sup>proof of concept, might eat your laundry

`cargo at` is a cargo subcommand that connects to a remote host via SSH, syncs the local source code of your Rust project, and builds it on the remote host.

This is useful when you have access to a more powerful machine than your local one, and want to use it to build and run your Rust projects (or a host with a different architecture).

**Note**: Windows support is... tricky at the moment (SSH, OpenSSL and rsync are all problem childs)

## Prerequisites

* A working and accessible SSH server listening on the remote host
* `ssh` client installed on the local machine
* `rsync` installed on both machines (used for synchronizing the code)
* Rust and Cargo, installed on both machines
* The project you want to use this on must be located in a git repository

## Installation

* Install this crate on both machines: `cargo install cargo-at`
* Add Cargo's binary directory to your path (as instructed by Cargo)
* Try it out locally: Run `cargo at $USER@127.0.0.1 build` in the project root folder
* Try it out remotely: Like the previous step, but replace `$USER@127.0.0.1` with the remote host (eg. `me@ssh.example.com`)
