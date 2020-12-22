/*
 * Figure out CLI
 *
*/

use project::Project;
use symbolic_executor::*;
use bitvector::{BV, BVV, BVE, BVS};
use emulator::*;

pub fn run(proj: Project) {
    // Creates a new emulator at the main function
    let mut executor = SymbolicExecutor::entry(&proj.program.bv);

    for _ in 0..50 {
        executor.step();
    }

    let mut emulator = Emulator::new(&proj);

    //BV::Concrete(BVV::from_u64(5));
    //executor.state.print();
    //solver::test();
}
