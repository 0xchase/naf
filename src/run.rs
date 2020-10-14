use ninja::*;
use state::*;

pub fn run(program: Program) {
    
    let mut state = State::entry(&program);

    for i in 0..70 {
        state.step();
    }
    
    program.seek(state.addr);

    state.print();

    //program.symbols();
}
