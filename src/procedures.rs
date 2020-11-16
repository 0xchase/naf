use state;

pub fn call(name: String, state: &state::State) -> u64 {
    // Todo, add code that calls the procedure
    return match &*name {
        "puts" => puts(),
        "printf" => printf(),
        _ => {
            error!("Unimplemented procedure: {}", name);
            0
        }
    }
}

/* These simulate various procedure calls */

fn puts() -> u64 {
    info!("Calling procedures puts()");
    return 0;
}

fn printf() -> u64 {
    info!("Calling procedures printf()");
    return 0;
}

fn fgets(state: &mut state::State) -> u64 {
    info!("0x{:x} Calling procedures fgets(), adding string 1234", state.addr);
    return 0;
}

fn strlen(state: &mut state::State) -> u64 {
    info!("0x{:x} Calling procedures strlen()", state.addr);
    return 0;
}

fn atoi(state: &mut state::State) -> u64 {
    info!("0x{:x} Calling procedures atoi()", state.addr);
    return 0;
}
