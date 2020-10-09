use std::collections::HashMap;
use ninja;

pub struct State<'a> {
    program: &'a ninja::Program<'a>,
    addr: u64,
    memory: HashMap<String, u64>,
    regs: HashMap<String, u64>,
    call_stack: Vec<u64>,
}

impl<'a> State<'a> {
    pub fn entry(program: &'a ninja::Program) -> State<'a> {
        info!("Creating state for {}", program.name());

        for function in program.functions() {
            if function.name.eq("main") {

                let temp = function.llil_start();
                return State {
                    program: program,
                    addr: temp,
                    memory: HashMap::new(),
                    regs: HashMap::new(),
                    call_stack: Vec::new(),
                }
            }
        }

        let mut reg_list: HashMap<String, u64> = HashMap::new();
        reg_list.insert(String::from("rax"), 1);
        reg_list.insert(String::from("rbx"), 2);
        reg_list.insert(String::from("rcx"), 3);
        reg_list.insert(String::from("rdx"), 4);
        reg_list.insert(String::from("rsi"), 5);
        reg_list.insert(String::from("rdi"), 6);
        reg_list.insert(String::from("r8"), 7);
        reg_list.insert(String::from("r9"), 8);
        reg_list.insert(String::from("r10"), 9);
        reg_list.insert(String::from("r11"), 10);
        reg_list.insert(String::from("r12"), 11);
        reg_list.insert(String::from("r13"), 12);
        reg_list.insert(String::from("r14"), 13);
        reg_list.insert(String::from("r15"), 14);
        reg_list.insert(String::from("rip"), 15);
        reg_list.insert(String::from("rbp"), 16);
        reg_list.insert(String::from("rflags"), 0);
        reg_list.insert(String::from("rsp"), 0x400000);

        return State {
            program: program,
            addr: 0,
            memory: HashMap::new(),
            regs: reg_list,
            call_stack: Vec::new()
        }
    }

    pub fn step(&mut self) {
        use ninja::LlilInst::*;
        use expression::Expr::*;
        
        if let Ok(inst) = self.program.inst_at(self.addr) {
            self.addr = inst.addr;

            match inst.llil {
                Push(llil) => {
                    match llil.expr {
                        Reg(r) => info!("0x{:x} Pushing register {}", self.addr, r.name),
                        _ => info!("0x{:x} Pushing other", self.addr),
                    }
                }
                Call(llil) => {
                    match llil.target {
                        Value(v) => {
                            if let Ok(inst2) = self.program.inst_after(self.addr) {
                                self.call_stack.push(inst2.addr);
                            } else {
                                error!("0x{:x} Coudln't add to call stack", self.addr);
                            }

                            info!("0x{:x} Calling address 0x{:x}", self.addr, v);
                            self.addr = v + 4;
                        },
                        _ => info!("0x{:x} Calling other", self.addr),
                    }
                }
                _ => {
                    info!("0x{:x} Stepping", self.addr);
                }
            }

            if let Ok(inst2) = self.program.inst_after(self.addr) {
                self.addr = inst2.addr;
            }
        } else {
            if let Some(temp) = self.call_stack.pop() {
                self.addr = temp;
                error!("0x{:x} Couldn't step... returning", self.addr);
            } else {
                error!("0x{:x} Couldn't step... exiting", self.addr);
            }
        }
    }
}
