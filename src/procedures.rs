use state;

pub fn call(name: String, state: &mut state::State) {
    /**
     * Parses the name of function and matches it with the appropriate function.
     **/
    let mut_state = state; 
    match name.as_str() {
        "puts" => puts(mut_state),
        "printf" => printf(state),
        "fgets" => fgets(mut_state),
        "strlen" => strlen(mut_state),
        "atoi" => atoi(mut_state),
        _ => unknown(mut_state),
    }
}

/* These simulate various procedure calls */

/**
 * Call to puts
 */

fn puts(state: &mut state::State) {
    info!("0x{:x} Calling procedures puts() (printing string at 0x{:x})", state.addr, state.regs.x64.unwrap().rdi);
    state.regs.x64.unwrap().rax = 0;
}

/**
 * Call to printf
 */
fn printf(state: &mut state::State) {
    let x64_regs = &state.regs.x64;
    info!("0x{:x} Calling procedures printf() (printing string at 0x{:x})", state.addr, state.regs.x64.unwrap().rdi);
    x64_regs.unwrap().rax = 0;
}

/**
 * Call to fgets
 */
fn fgets(state: &mut state::State) {
    info!("0x{:x} Calling procedures fgets(), adding string ...", state.addr);
    state.stdin = String::from("1234");
    state.regs.x64.unwrap().rax = 0;
}

/** 
 * Call to strlen
 */
fn strlen(state: &mut state::State) {
    info!("0x{:x} Calling procedures strlen()", state.addr);
    state.regs.x64.unwrap().rax = 4;
}

/**
 * Call to atoi
 */ 
fn atoi(state: &mut state::State) {
    info!("0x{:x} Calling procedures atoi()", state.addr);
    state.regs.x64.unwrap().rax = 6;
}

fn unknown(state: &mut state::State) {
    error!("Calling unknown library");
}