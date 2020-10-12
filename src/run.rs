use ninja::*;
use state::*;

pub fn run(program: Program) {
    
    let mut state = State::entry(&program);

    for i in 0..50 {
        state.step();

        if i % 4 == 0 {
            //state.print();
        }
    }

    state.print();

    program.symbols();
}
