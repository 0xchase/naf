use ninja::*;
use state::*;
use solver;

pub fn run(program: Program) {
    
    let mut state = State::entry(&program);
    
    for _ in 0..120 {
        state.step();
    }
    
    program.seek(state.addr);
    
    state.print();

    // -------------------------------

    solver::test();

    //program.strings();
    //program.symbols();
}
