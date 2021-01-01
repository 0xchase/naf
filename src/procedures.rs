use state;

pub fn call(name: String, state: &mut state::State) {
    /**
     * Parses the name of function and matches it with the appropriate function.
     **/
    let mut_state = state; 
    match name.as_str() {
        "puts" => puts(mut_state),
        "printf" => printf(mut_state),
        "fgets" => fgets(mut_state),
        "strlen" => strlen(mut_state),
        "atoi" => atoi(mut_state),
        _ => unknown(mut_state),
    }
    
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