extern crate naf;

use naf::project::*;

fn run(proj: Project) {
    
    println!("Running plugin");

    for function in proj.program.functions() {
        println!("{}", function.name);
    }

    println!("Done running plugin");
    
}
