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
        let result = emulator.step();
        match result {
            Ok(_) => continue,
            Err(err) => {
                error!("Run error: {}", &err);
                break;
            },
        };
    }
    
    emulator.state.print();

    let mut tainter = TaintTracker::main(&proj.program);

    for _ in 0..20 {
        tainter.step();
        info!("Taint tracker is at address 0x{:x}, tainted regs are:", tainter.state.addr);
        for reg in &tainter.state.regs_tainted {
            info!(" > {}", reg);
        }
    }
    
}
