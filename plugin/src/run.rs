use ninja::*;
use state::*;

pub fn run(program: Program) {
    
    let mut state = State::entry(&program);

    for _ in 0..60 {
        state.step();
    }
}
