# NAF

Static analysis framework built on binary ninja, written in Rust. Will eventually support emulation with unicorn, simulations of various OS components, emulation and symbolic execution over binja LLIL, taint analysis, and more. 

## Build Instructions

- Download and install `rust, cargo, rustup, Binary Ninja, and llvm`
- Clone the repo
- Make sure to install `z3` and `sudo apt install libz3-dev`
- run `cargo build`
