# NAF

Static analysis framework built on binary ninja, written in Rust. Will eventually support emulation with unicorn, fuzing with afl++, simulations of various OS components, emulation and symbolic execution over binja LLIL, taint analysis, and more. 

## Build Instructions

- Download and install `rust, cargo, rustup, and llvm`
- Clone the repo
- Make sure to install `z3` and `sudo apt install libz3-dev`
- Clone the `binja-rs` repo while inside the ninja-analysis-framework folder (except with `--recurse-submodules flag`)
- run `cargo build`
