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
   let mut mainNotFound = false;

    let mut result = Emulator::main(&proj.program, proj.arch);
    
    match (result) {
        Some(e) => {
            let mut emulator = e; 
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
        }
        None => {
            info!("Couldn't find main.");
            mainNotFound = true;
        }
    }

    

    // let mut tainter = TaintTracker::main(&proj.program);

    // for _ in 0..20 {
    //     tainter.step();
    //     info!("Taint tracker is at address 0x{:x}, tainted regs are:", tainter.state.addr);
    //     for reg in &tainter.state.regs_tainted {
    //         info!(" > {}", reg);
    //     }
    // }
    
}
