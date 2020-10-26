use ninja::*;
use state::*;
use solver;
use debugger::*;

pub fn run(program: Program, mut debugger: Debugger) {
    
    let mut state = State::entry(&program);
    
    for _ in 0..120 {
        state.step();
    }
    
    program.seek(state.addr);
    
    state.print();

    // -------------------------------

    solver::test();

    debugger.init();
    debugger.breakpoint(0x4006f6);
    debugger.go();

    //program.strings();
    //program.symbols();
}
