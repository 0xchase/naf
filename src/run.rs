use state::*;
use project::*;

pub fn run(project: Project) {
    /*
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
    */
    
    let mut debugger = project.debugger;

    debugger.init();
    info!("Debugger at address 0x{:x}", debugger.ip());
    debugger.breakpoint(0x400600);
    debugger.go();
    info!("Debugger at address 0x{:x}", debugger.ip());
    debugger.breakpoint(0x4006f6);
    debugger.go();
    info!("Debugger at address 0x{:x}", debugger.ip());
    debugger.breakpoint(0x400767);
    debugger.go();
    info!("Debugger at address 0x{:x}", debugger.ip());
    info!("Debugger at address 0x{:x}", debugger.reg_read("rdi"));

    //project.python.run("lskjdf");

    // -------------------------------

    //solver::test();

    //debugger.init();
    //debugger.breakpoint(0x4006f6);
    //debugger.go();

    //program.strings();
    //program.symbols();
    
}
