use state;

pub fn puts(state: &mut state::State) {
    info!("0x{:x} Calling procedures puts (printing string at 0x{:x})", state.addr, state.regs.rdi);
    state.regs.rax = 0;
}

pub fn printf(state: &mut state::State) {
    info!("0x{:x} Calling procedures printf (printing string at 0x{:x})", state.addr, state.regs.rdi);
    state.regs.rax = 0;
}

pub fn fgets(state: &mut state::State) {
    info!("0x{:x} Calling procedures fgets", state.addr);
    state.regs.rax = 0;
}

pub fn unknown(state: &mut state::State) {
    error!("Calling unknown library");
}

struct procedure {
    pub name: String,
    pub function: u64,
}