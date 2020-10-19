use ninja::*;
use state::*;

pub fn run(program: Program) {
    
    let mut state = State::entry(&program);
    
    for _ in 0..80 {
        state.step();
    }
    
    program.seek(state.addr);

    state.print();

    //program.strings();
    //program.symbols();
}
