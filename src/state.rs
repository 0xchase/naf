use std::collections::HashMap;
use ninja;
use procedures;

pub struct State<'a> {
    program: &'a ninja::Program<'a>,
    pub addr: u64,
    pub memory: Memory,
    pub regs: Regsx64,
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
                    memory: Memory::new(),
                    regs: Regsx64::new(),
                    call_stack: Vec::new(),
                }
            }
        }

        return State {
            program: program,
            addr: 0,
            memory: Memory::new(),
            regs: Regsx64::new(),
            call_stack: Vec::new()
        }
    }

    fn step_ip(&mut self) {
        if let Ok(inst2) = self.program.inst_after(self.addr) {
            self.addr = inst2.addr;
        }
    }

    pub fn step(&mut self) {
        use ninja::LlilInst::*;
        use expression::Expr::*;
        use expression::eval_expression;
 
        if let Ok(inst) = self.program.inst_at(self.addr) {
            self.addr = inst.addr;

            match inst.llil {
                SetReg(llil) => {
                    let val = eval_expression(llil.expr, self);
                    self.regs.set(llil.reg, val);
                    info!("0x{:x} Set register to 0x{:x}", self.addr, val);
                    self.step_ip()
                }
                Push(llil) => {
                    match llil.expr {
                        Reg(r) => {
                            info!("0x{:x} Pushing register {}", self.addr, r.name);
                            self.memory.store(self.regs.rsp, self.regs.get(r.name));
                        },
                        _ => info!("0x{:x} Pushing other", self.addr),
                    }
                    self.regs.rsp -= 8;
                    self.step_ip()
                }
                If(llil) => {
                    let result: u64 = eval_expression(llil.condition, self);
                    info!("0x{:x} Comparing, flag has value 0x{:x}", self.addr, result);
                    if result == 0 {
                        self.addr = llil.target_false;
                        info!(" > Branching false");
                    } else {
                        self.addr = llil.target_true;
                        info!(" > Branching true");
                    }
                }
                Store(llil) => {
                    let val: u64 = eval_expression(llil.source_expr, self);
                    let addr: u64 = eval_expression(llil.dest_mem_expr, self);

                    self.memory.store(addr, val);

                    info!("0x{:x} Stored 0x{:x} at 0x{:x}", self.addr, val, addr);

                    self.step_ip();
                }
                Call(llil) => {
                    match llil.target {
                        Value(v) => {
                            if let Ok(function) = self.program.function_at(v) {
                                if function.name == String::from("puts") {
                                    procedures::puts(self);
                                } else if function.name == String::from("printf") {
                                    procedures::printf(self);
                                } else if function.name == String::from("fgets") {
                                    procedures::fgets(self);
                                } else if function.name == String::from("strlen") {
                                    procedures::strlen(self);
                                } else if function.name == String::from("atoi") {
                                    procedures::atoi(self); 
                                } else {
                                    if let Ok(inst2) = self.program.inst_after(self.addr) {
                                        self.call_stack.push(inst2.addr);
                                    } else {
                                        error!("0x{:x} Coudln't add to call stack", self.addr);
                                    }

                                    info!("0x{:x} Calling address 0x{:x}", self.addr, v);
                                    self.addr = v + 4;
                                }
                            } else {
                                error!("Couldn't call function");
                            }
                            self.step_ip()
                        },
                        _ => error!("0x{:x} Calling other", self.addr),
                    }
                }
                _ => {
                    error!("0x{:x} Stepping", self.addr);
                    self.step_ip()
                }
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

    pub fn print(&self) {
        info!("______________________________________");
        info!("\trax 0x{:x}\trbx 0x{:x}\trcx 0x{:x}", self.regs.rax, self.regs.rbx, self.regs.rcx);
        info!("\trdx 0x{:x}\tsi 0x{:x}\trdi 0x{:x}", self.regs.rdx, self.regs.rsi, self.regs.rdi);
        info!("\tr8 0x{:x}\tr9 0x{:x}\tr10 0x{:x}", self.regs.r8, self.regs.r9, self.regs.r10);
        info!("\tr11 0x{:x}\tr12 0x{:x}\tr13 0x{:x}", self.regs.r11, self.regs.r12, self.regs.r13);
        info!("\tr14 0x{:x}\tr15 0x{:x}\trip 0x{:x}", self.regs.r14, self.regs.r15, self.regs.rip);
        info!("\trbp 0x{:x}\trflags 0x{:x}\trsp 0x{:x}", self.regs.rbp, self.regs.rflags, self.regs.rsp);
        info!("");

        self.memory.print();

        info!("______________________________________");

    }
}

pub struct Memory {
    pub map: HashMap<u64, u64>
}

impl Memory {
    fn new() -> Memory {
        return Memory {
            map: HashMap::new()
        }
    }

    pub fn store(&mut self, addr: u64, value: u64) {
        self.map.insert(addr, value);
    }
    
    pub fn load(&mut self, addr: u64) -> u64 {
        return match self.map.get_key_value(&addr) {
            Some((&key, &value)) => return value,
            _ => 0
        };
    }

    pub fn print(&self) {
        for (key, value) in &self.map {
            info!("\t0x{:x}: {}", key, value);
        }
    }
}

pub struct Regsx64 {
    pub rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rsi: u64,
    pub rdi: u64,
    rsp: u64,
    rbp: u64,
    rip: u64,
    rflags: u64,
}

impl Regsx64 {
    fn new() -> Regsx64 {
        return Regsx64 {
            rax: 0xa, rbx: 0xb, rcx: 0xc, 
            rdx: 0xd, r8: 8, r9: 9, 
            r10: 10, r11: 11, r12: 12, 
            r13: 13, r14: 14, r15: 15, 
            rsi: 0, rdi: 0, rsp: 0x7fff00000000, 
            rbp: 0, rip: 0, rflags: 0
        };
    }
    pub fn set(&mut self, name: String, value: u64) {
        match name.as_str() {
            "rax" => self.rax = value,
            "rbx" => self.rbx = value,
            "rcx" => self.rcx = value,
            "rdx" => self.rdx = value,
            "r8" => self.r8 = value,
            "r9" => self.r9 = value,
            "r10" => self.r10 = value,
            "r11" => self.r11 = value,
            "r12" => self.r12 = value,
            "r13" => self.r13 = value,
            "r14" => self.r14 = value,
            "r15" => self.r15 = value,
            "rsi" => self.rsi = value,
            "rdi" => self.rdi = value,
            "rsp" => self.rsp = value,
            "rbp" => self.rbp = value,
            "rip" => self.rip = value,

            "eax" => self.rax = value,
            "ebx" => self.rbx = value,
            "ecx" => self.rcx = value,
            "edx" => self.rdx = value,
            "e8" => self.r8 = value,
            "e9" => self.r9 = value,
            "e10" => self.r10 = value,
            "e11" => self.r11 = value,
            "e12" => self.r12 = value,
            "e13" => self.r13 = value,
            "e14" => self.r14 = value,
            "e15" => self.r15 = value,
            "esi" => self.rsi = value,
            "edi" => self.rdi = value,
            "esp" => self.rsp = value,
            "ebp" => self.rbp = value,
            "eip" => self.rip = value,
            _ => error!("Couldn't set register {}", name)
        }
    }

    pub fn get(&self, name: String) -> u64 {
        match name.as_str() {
            "rax" => self.rax,
            "rbx" => self.rbx,
            "rcx" => self.rcx,
            "rdx" => self.rdx,
            "r8" => self.r8,
            "r9" => self.r9,
            "r10" => self.r10,
            "r11" => self.r11,
            "r12" => self.r12,
            "r13" => self.r13,
            "r14" => self.r14,
            "r15" => self.r15,
            "rsi" => self.rsi,
            "rdi" => self.rdi,
            "rsp" => self.rsp,
            "rbp" => self.rbp,
            "rip" => self.rip,

            "eax" => self.rax,
            "ebx" => self.rbx,
            "ecx" => self.rcx,
            "edx" => self.rdx,
            "e8" => self.r8,
            "e9" => self.r9,
            "e10" => self.r10,
            "e11" => self.r11,
            "e12" => self.r12,
            "e13" => self.r13,
            "e14" => self.r14,
            "e15" => self.r15,
            "esi" => self.rsi,
            "edi" => self.rdi,
            "esp" => self.rsp,
            "ebp" => self.rbp,
            "eip" => self.rip,
            _ => {
                error!("Couldn't get register {}", name);
                return 0;
            }
        }
    }
}