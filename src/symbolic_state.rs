use std::collections::HashMap;
use solver::Solver;
use bitvector::*;
use z3;

pub struct SymbolicState<'bv> {
    pub memory: Memory,
    pub regs: Regsx64<'bv>,
    pub stdin: String,
}

impl<'bv> SymbolicState<'bv> {
    pub fn new(cfg: &z3::Config) -> SymbolicState<'bv> {
        return SymbolicState {
            memory: Memory::new(),
            regs: Regsx64::new(),
            stdin: String::from("Here is some test stdin"),
        }
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

    pub fn load(&self, addr: u64) -> u64 {
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

pub struct Regsx64<'bv> {
    pub rax: BV<'bv>,
    pub rbx: BV<'bv>,
    pub rcx: BV<'bv>,
    pub rdx: BV<'bv>,
    pub r8: BV<'bv>,
    pub r9: BV<'bv>,
    pub r10: BV<'bv>,
    pub r11: BV<'bv>,
    pub r12: BV<'bv>,
    pub r13: BV<'bv>,
    pub r14: BV<'bv>,
    pub r15: BV<'bv>,
    pub rsi: BV<'bv>,
    pub rdi: BV<'bv>,
    pub rsp: BV<'bv>,
    pub rbp: BV<'bv>,
    pub rip: BV<'bv>,
    pub rflags: BV<'bv>,
    pub rtemp: HashMap<String, BV<'bv>>,
}

impl<'bv> Regsx64<'bv> {
    pub fn new() -> Regsx64<'bv> {
        return Regsx64 {
            rax: BV::Concrete(BVV::from_u64(0x004006f6)),
            rbx: BV::Concrete(BVV::from_u64(0x004008f0)),
            rcx: BV::Concrete(BVV::from_u64(0x004008f0)),
            rdx: BV::Concrete(BVV::from_u64(0x7fffffffe138)),
            r8: BV::Concrete(BVV::from_u64(0x00000000)),
            r9: BV::Concrete(BVV::from_u64(0x7ffff7fe0d50)),
            r10: BV::Concrete(BVV::from_u64(0xfffffffffffff40c)),
            r11: BV::Concrete(BVV::from_u64(0x7ffff7de5fc0)),
            r12: BV::Concrete(BVV::from_u64(0x00400600)),
            r13: BV::Concrete(BVV::from_u64(0x7fffffffe170)),
            r14: BV::Concrete(BVV::from_u64(0x00000000)),
            r15: BV::Concrete(BVV::from_u64(0x00000000)),
            rsi: BV::Concrete(BVV::from_u64(0x7fffffffe128)),
            rdi: BV::Concrete(BVV::from_u64(0x00000000)),
            rsp: BV::Concrete(BVV::from_u64(0x7fffffffe088)),
            rbp: BV::Concrete(BVV::from_u64(0)),
            rip: BV::Concrete(BVV::from_u64(0)),
            rflags: BV::Concrete(BVV::from_u64(0)),
            rtemp: HashMap::new(),
        };
    }

    pub fn set(&mut self, name: String, value: BV<'bv>) {

        match name.as_str() {
            "rax" => self.rax = value.clone(),
            "rbx" => self.rbx = value.clone(),
            "rcx" => self.rcx = value.clone(),
            "rdx" => self.rdx = value.clone(),
            "r8" => self.r8 = value.clone(),
            "r9" => self.r9 = value.clone(),
            "r10" => self.r10 = value.clone(),
            "r11" => self.r11 = value.clone(),
            "r12" => self.r12 = value.clone(),
            "r13" => self.r13 = value.clone(),
            "r14" => self.r14 = value.clone(),
            "r15" => self.r15 = value.clone(),
            "rsi" => self.rsi = value.clone(),
            "rdi" => self.rdi = value.clone(),
            "rsp" => self.rsp = value.clone(),
            "rbp" => self.rbp = value.clone(),
            "rip" => self.rip = value.clone(),

            "eax" => self.rax = value.clone(),
            "ebx" => self.rbx = value.clone(),
            "ecx" => self.rcx = value.clone(),
            "edx" => self.rdx = value.clone(),
            "e8" => self.r8 = value.clone(),
            "e9" => self.r9 = value.clone(),
            "e10" => self.r10 = value.clone(),
            "e11" => self.r11 = value.clone(),
            "e12" => self.r12 = value.clone(),
            "e13" => self.r13 = value.clone(),
            "e14" => self.r14 = value.clone(),
            "e15" => self.r15 = value.clone(),
            "esi" => self.rsi = value.clone(),
            "edi" => self.rdi = value.clone(),
            "esp" => self.rsp = value.clone(),
            "ebp" => self.rbp = value.clone(),
            "eip" => self.rip = value.clone(),
            _ => {
                self.rtemp.insert(name, value.clone());
            },
        }
    }

    pub fn get(&self, name: String) -> BV<'bv> {
        match name.as_str() {
            "rax" => self.rax.clone(),
            "rbx" => self.rbx.clone(),
            "rcx" => self.rcx.clone(),
            "rdx" => self.rdx.clone(),
            "r8" => self.r8.clone(),
            "r9" => self.r9.clone(),
            "r10" => self.r10.clone(),
            "r11" => self.r11.clone(),
            "r12" => self.r12.clone(),
            "r13" => self.r13.clone(),
            "r14" => self.r14.clone(),
            "r15" => self.r15.clone(),
            "rsi" => self.rsi.clone(),
            "rdi" => self.rdi.clone(),
            "rsp" => self.rsp.clone(),
            "rbp" => self.rbp.clone(),
            "rip" => self.rip.clone(),

            "eax" => self.rax.clone(),
            "ebx" => self.rbx.clone(),
            "ecx" => self.rcx.clone(),
            "edx" => self.rdx.clone(),
            "e8" => self.r8.clone(),
            "e9" => self.r9.clone(),
            "e10" => self.r10.clone(),
            "e11" => self.r11.clone(),
            "e12" => self.r12.clone(),
            "e13" => self.r13.clone(),
            "e14" => self.r14.clone(),
            "e15" => self.r15.clone(),
            "esi" => self.rsi.clone(),
            "edi" => self.rdi.clone(),
            "esp" => self.rsp.clone(),
            "ebp" => self.rbp.clone(),
            "eip" => self.rip.clone(),
            "fsbase" => self.rbp.clone(),

            _ => {
                return match self.rtemp.get(&name) {
                    Some(i) => i.clone(),
                    _ => self.rax.clone(), // This is not correct
                }
            }
        }
    }
}
