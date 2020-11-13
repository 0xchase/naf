/* 
 * Figure out CLI
 * 
 * 
*/

use state::*;
use project::*;
use emulator::*;

pub fn run(proj: Project) {

    for function in proj.program.functions() {
        info!("{}", function.name);        
    }

    // Creates a new emulator at the main function
    let mut emulator = Emulator::main(&proj.program);
    
    for _ in 0..50 {
        emulator.step();
    }
    
    emulator.state.print();
    
}
