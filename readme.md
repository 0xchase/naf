# ninja-analysis-framework

Static analysis framework built on binary ninja, written in Rust. 

## Build Instructions

- Download and install `rust, cargo, rustup, and llvm`
- Clone the repo
- Make sure to install `z3` and `sudo apt install libz3-dev`
- Clone the `binja-rs` repo while inside the ninja-analysis-framework folder (except with `--recurse-submodules flag`)
- run `cargo build`

Note: I added interoperability with cpp. This reqires some other prereqs not listed here.
