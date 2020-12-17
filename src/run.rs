/* 
 * Figure out CLI
 * 
 * 
*/

use state::*;
use project::*;
use emulator::*;
use taint_tracker::*;

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

    let mut tainter = TaintTracker::main(&proj.program);

    TaintTracker::taint_reg(&mut tainter.states[0], String::from("rax"), 0x00400812, 1);
    info!("Added rax as taint source at address 0x00400812, index = 1");

    for _ in 0..100 {
        tainter.step();
        for state in &tainter.states {
            info!("Taint tracker is at address 0x{:x}", state.addr);
        }
    }
    for state in tainter.states {
        for reg in &state.regs_tainted {
            info!(" > {}", reg);
        }
        for addr in &state.stack_tainted {
            info!(" > {}", addr);
        }
    }
    
}
