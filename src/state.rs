use std::collections::HashMap;
use program::Program;
use binaryninja::binaryview::{BinaryView, BinaryViewExt};

pub enum Arch {
    X86_64 = 0,
    Arm = 1,
    Aarch64 = 2
}
impl Arch {
    fn from_u64(value: u64) -> Arch {
        match value {
            0 => Arch::X86_64,
            1 => Arch::Arm,
            2 => Arch::Aarch64
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

pub struct RegsAarch64 {
    pub r0: u64,
    pub r1: u64,
    pub r2: u64,
    pub r3: u64,
    pub r4: u64,
    pub r5: u64,
    pub r6: u64,
    pub r7: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub r16: u64,
    pub r17: u64,
    pub r18: u64,
    pub r19: u64,
    pub r20: u64,
    pub r21: u64,
    pub r22: u64,
    pub r23: u64,
    pub r24: u64,
    pub r25: u64,
    pub r26: u64,
    pub r27: u64,
    pub r28: u64,
    pub r29: u64,
    pub r30: u64,
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub x30: u64,
    pub w0: u32,
    pub w1: u32,
    pub w2: u32,
    pub w3: u32,
    pub w4: u32,
    pub w5: u32,
    pub w6: u32,
    pub w7: u32,
    pub w8: u32,
    pub w9: u32,
    pub w10: u32,
    pub w11: u32,
    pub w12: u32,
    pub w13: u32,
    pub w14: u32,
    pub w15: u32,
    pub w16: u32,
    pub w17: u32,
    pub w18: u32,
    pub w19: u32,
    pub w20: u32,
    pub w21: u32,
    pub w22: u32,
    pub w23: u32,
    pub w24: u32,
    pub w25: u32,
    pub w26: u32,
    pub w27: u32,
    pub w28: u32,
    pub w29: u32,
    pub w30: u32,
    pub rflags: u64,
    pub rtemp: HashMap<String, u64>,   
}
pub struct RegsArm {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub rflags: u64,
    pub rtemp: HashMap<String, u64>,
}

#[derive(Default)]
pub struct Registers {
    pub x64: Option<Regsx64>,
    pub arm: Option<RegsArm>,
    pub aarch64: Option<RegsAarch64>
}

pub struct State {
    pub addr: u64,
    pub index: usize,
    pub memory: Memory,
    pub regs: Registers,
    pub call_stack: Vec<u64>,
    pub stdin: String,
    pub arch: u64
}

impl State {
    pub fn new(arch_string: String) -> State {
        let arch = Convert_Arch_To_Enum(arch_string);
        return State {
            addr: 0,
            index: 0,
            memory: Memory::new(),
            regs: Registers::new(arch),
            call_stack: Vec::new(),
            stdin: String::from(""),
            arch: arch
        }
    }

    pub fn print(&self) {
        info!("______________________________________");
        let x64_regs = self.regs.x64.unwrap();
        match Arch::from_u64(self.arch) {
            Arch::X86_64 => {
                info!("\trax 0x{:x}\trbx 0x{:x}\trcx 0x{:x}", x64_regs.rax, x64_regs.rbx, x64_regs.rcx);
                info!("\trdx 0x{:x}\tsi 0x{:x}\trdi 0x{:x}", x64_regs.rdx, x64_regs.rsi, x64_regs.rdi);
                info!("\tr8 0x{:x}\tr9 0x{:x}\tr10 0x{:x}", x64_regs.r8, x64_regs.r9, x64_regs.r10);
                info!("\tr11 0x{:x}\tr12 0x{:x}\tr13 0x{:x}", x64_regs.r11, x64_regs.r12, x64_regs.r13);
                info!("\tr14 0x{:x}\tr15 0x{:x}\trip 0x{:x}", x64_regs.r14, x64_regs.r15, x64_regs.rip);
                info!("\trbp 0x{:x}\trflags 0x{:x}\trsp 0x{:x}", x64_regs.rbp, x64_regs.rflags, x64_regs.rsp);
                info!("");
            },
            _ => {
                info!("Nothing to print");
            }
        }


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

impl Default for Regsx64 {
    fn default() -> Self {
        return Regsx64 {
            rax: 0x004006f6, rbx: 0x004008f0, rcx: 0x004008f0, 
            rdx: 0x7fffffffe188, r8: 0x00000000, r9: 0x7ffff7fe0d50, 
            r10: 0xfffffffffffff40c, r11: 0x7ffff7de6fc0, r12: 0x00400600, 
            r13: 0x7fffffffe170, r14: 0x00000000, r15: 0x00000000, 
            rsi: 0x7fffffffe178, rdi: 0x00000001, rsp: 0x7fffffffe088, 
            rbp: 0, rip: 0, rflags: 0, rtemp: HashMap::new(),
        };
    }
}

impl Registers {
    pub fn new(arch: u64) -> Registers {
        let mut registers = Registers {
            x64: None,
            arm: None,
            aarch64: None
        };
        match Arch::from_u64(arch){
            Arch::X86_64 => {
                registers.x64 = Some(Regsx64::new());
            },
            _ => {
                error!("Unknown architecture being implemented");
            }
        }
        return registers;
    }
    pub fn set(&mut self, name: String, value: u64, arch: u64) {
        match Arch::from_u64(arch) {
            Arch::X86_64 => {
                self.x64.unwrap().set(name, value);
            }
            _ => {
                error!("Setting registers of unknown architecture!");
            }
        }
    }
    pub fn get_stack_pointer(&mut self, arch: u64) -> u64 {
        match Arch::from_u64(arch) {
            Arch::X86_64 => {
                let x64 = self.x64.unwrap();
                return x64.rsp;
            }
            _ => {
                error!("Retrieving stack pointer of unknown register");
                return 0 as u64;
            }
        }
    }
    pub fn set_stack_pointer(&mut self, arch: u64) -> u64 {
        match Arch::from_u64(arch){
            Arch::X86_64 => {
                return self.x64.unwrap().rsp;
            }
            _ => {
                error!("Retrieving stack pointer of unknown register");
                return 0 as u64;
            }
        }  
    }
    pub fn get(&self, name: String, arch: u64) -> u64 {
        match Arch::from_u64(arch){
            Arch::X86_64 => {
                return self.x64.unwrap().get(name);
            }
            _ => {
                error!("Retrieving stack pointer of unknown register");
                return 0 as u64;
            }
        }
    }
}

impl Regsx64 {
    pub fn new() -> Regsx64 {
        // return Regsx64 {
        //     rax: 0x004006f6, rbx: 0x004008f0, rcx: 0x004008f0, 
        //     rdx: 0x7fffffffe188, r8: 0x00000000, r9: 0x7ffff7fe0d50, 
        //     r10: 0xfffffffffffff40c, r11: 0x7ffff7de6fc0, r12: 0x00400600, 
        //     r13: 0x7fffffffe170, r14: 0x00000000, r15: 0x00000000, 
        //     rsi: 0x7fffffffe178, rdi: 0x00000001, rsp: 0x7fffffffe088, 
        //     rbp: 0, rip: 0, rflags: 0, rtemp: HashMap::new(),
        // };
        return Default::default(); 
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

pub fn Convert_Arch_To_Enum(arch_string : String) -> u64 {
    match &*arch_string {
        "x86_64" => return Arch::X86_64 as u64,
        _ => return u64::MAX
    }
    
}