use state;

pub fn puts(state: &mut state::State) {
    info!("0x{:x} Calling procedures puts() (printing string at 0x{:x})", state.addr, state.regs.rdi);
    state.regs.rax = 0;
}

pub fn printf(state: &mut state::State) {
    info!("0x{:x} Calling procedures printf() (printing string at 0x{:x})", state.addr, state.regs.rdi);
    state.regs.rax = 0;
}

pub fn fgets(state: &mut state::State) {
    info!("0x{:x} Calling procedures fgets(), adding string 1234", state.addr);
    state.stdin = String::from("1234");
    state.regs.rax = 0;
}

pub fn strlen(state: &mut state::State) {
    info!("0x{:x} Calling procedures strlen()", state.addr);
    state.regs.rax = 4;
}

pub fn atoi(state: &mut state::State) {
    info!("0x{:x} Calling procedures atoi()", state.addr);
    state.regs.rax = 6;
}

pub fn unknown(state: &mut state::State) {
    error!("Calling unknown library");
}

struct _Procedure {
    pub name: String,
    pub function: u64,
}
