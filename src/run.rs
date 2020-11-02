use state::*;
use project::*;

pub fn run(project: Project) {
    
    let mut state = State::entry(&project.program);

    for function in project.program.functions() {
        info!("{}", function.name);
    }

    let mut state = State::entry(&project.program);
    
    for _ in 0..120 {
        state.step();
    }
    
    project.program.seek(state.addr);
    
    state.print();
    
    
    /*
    let mut debugger = project.debugger;

    debugger.init();
    debugger.go_until(0x400600);
    debugger.regs_print();

    debugger.go_until(0x4006f6);
    debugger.regs_print();

    debugger.go_until(0x400767);
    debugger.regs_print();

    debugger.go_until(0x400815);
    debugger.regs_print();
    */

    //project.python.run("lskjdf");

    // -------------------------------

    //solver::test();

    //debugger.init();
    //debugger.breakpoint(0x4006f6);
    //debugger.go();

    //program.strings();
    //program.symbols();
    
}
