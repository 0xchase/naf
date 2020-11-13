use state;

pub fn call(name: String, state: state::State) {
    // Todo, add code that calls the procedure
}

/* These simulate various procedure calls */

fn puts(state: &mut state::State) {
    info!("0x{:x} Calling procedures puts() (printing string at 0x{:x})", state.addr, state.regs.rdi);
    state.regs.rax = 0;
}

fn printf(state: &mut state::State) {
    info!("0x{:x} Calling procedures printf() (printing string at 0x{:x})", state.addr, state.regs.rdi);
    state.regs.rax = 0;
}

fn fgets(state: &mut state::State) {
    info!("0x{:x} Calling procedures fgets(), adding string 1234", state.addr);
    state.stdin = String::from("1234");
    state.regs.rax = 0;
}

fn strlen(state: &mut state::State) {
    info!("0x{:x} Calling procedures strlen()", state.addr);
    state.regs.rax = 4;
}

fn atoi(state: &mut state::State) {
    info!("0x{:x} Calling procedures atoi()", state.addr);
    state.regs.rax = 6;
}

fn unknown(state: &mut state::State) {
    error!("Calling unknown library");
}