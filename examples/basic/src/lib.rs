extern crate naf;

use naf::project::*;
use naf::emulator::*;

#[no_mangle]
fn main(proj: Project) {
    
    println!("Running plugin");

    for function in proj.program.functions() {
        println!("{}", function.name);
    }

    let mut emulator = Emulator::main(&proj.program);

    for i in 0..10000 {
        emulator.step();
        println!("Step {}", i);
    }

    println!("Done running plugin");
    
}

