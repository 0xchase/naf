use std::collections::HashMap;
use program::Program;

pub struct State {
    pub addr: u64,
    pub index: usize,
    pub memory: Memory,
    pub regs: Regsx64,
    pub call_stack: Vec<u64>,
    pub stdin: String,
}

impl State {
    pub fn new() -> State {
        return State {
            addr: 0,
            index: 0,
            memory: Memory::new(),
            regs: Regsx64::new(),
            call_stack: Vec::new(),
            stdin: String::from(""),
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
    pub fn new() -> Memory {
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
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub rtemp: HashMap<String, u64>,
}

impl Regsx64 {
    pub fn new() -> Regsx64 {
        return Regsx64 {
            rax: 0x004006f6, rbx: 0x004008f0, rcx: 0x004008f0, 
            rdx: 0x7fffffffe188, r8: 0x00000000, r9: 0x7ffff7fe0d50, 
            r10: 0xfffffffffffff40c, r11: 0x7ffff7de6fc0, r12: 0x00400600, 
            r13: 0x7fffffffe170, r14: 0x00000000, r15: 0x00000000, 
            rsi: 0x7fffffffe178, rdi: 0x00000001, rsp: 0x7fffffffe088, 
            rbp: 0, rip: 0, rflags: 0, rtemp: HashMap::new(),
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
            _ => {
                self.rtemp.insert(name, value);
            },
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
            "fsbase" => 0,

            _ => {
                return match self.rtemp.get(&name) {
                    Some(i) => *i,
                    _ => 0 as u64,
                }
            }
        }
    }
}