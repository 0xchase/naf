use std::collections::HashMap;
use ninja;
use binja;

pub struct State<'a> {
    program: ninja::Program<'a>,
    addr: u64,
    memory: HashMap<String, u64>,
    regs: HashMap<String, u64>,
}

use ninja::LlilInst::*;
impl<'a> State<'a> {
    pub fn entry(program: ninja::Program) -> State {
        for function in program.functions() {
            let temp = function.first_llil_addr();

            if function.name.eq("main") {
                return State {
                    program: program,
                    addr: temp,
                    memory: HashMap::new(),
                    regs: HashMap::new()
                }
            }
        }

        return State {
            program: program,
            addr: 0,
            memory: HashMap::new(),
            regs: HashMap::new()
        }
    }

    pub fn step(&mut self) {
        info!("Stepping state at {:x}", self.addr);

        if let Ok(next_inst) = self.program.next_inst(self.addr) {
            match next_inst {
                Call(op) => {
                    self.addr = op.addr;
                },
                SetReg(op) => {
                    self.addr = op.addr;
                }
            }
        } else {
            info!("Couldn't get next instruction");
        }
    }
}