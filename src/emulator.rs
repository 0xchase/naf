use std::collections::HashMap;
use program::*;
use state::*;
use procedures;

pub struct Emulator<'a> {
    pub program: &'a Program<'a>,
    pub state: State,
}

impl<'a> Emulator<'a> {
    pub fn new(program: &'a Program, state: State) -> Emulator<'a> {
        return Emulator {
            program: program,
            state: state,
        }
    }

    pub fn main(program: &'a Program) -> Emulator<'a> {
        for function in program.functions() {
            if function.name.eq("main") {

                let temp = function.llil_start();

                return Emulator {
                    program: program,
                    state: State {
                        addr: temp,
                        index: 0,
                        memory: Memory::new(),
                        regs: Regsx64::new(),
                        call_stack: Vec::new(),
                        stdin: String::from(""),
                    },
                }
            }
        }
        return Emulator {
            program: program,
            state: State {
                addr: 0,
                index: 0,
                memory: Memory::new(),
                regs: Regsx64::new(),
                call_stack: Vec::new(),
                stdin: String::from(""),
            },
        }
    }

    pub fn step(&mut self) {
        use LlilInst::*;
        use expression::Expr::*;
        use expression::eval_expression;

        let inst: Inst = self.program.inst_at(self.state.addr).expect("No such instruction");

        match inst.llil {
            SetReg(llil) => {
                let val = eval_expression(llil.expr, &self.state);
                self.state.regs.set(llil.reg, val);
                info!("0x{:x} Set register to 0x{:x}", self.state.addr, val);
            }
            SetRegSplit(llil) => {
                let val = eval_expression(llil.source_expr, &self.state);
                let high: u32 = (val >> 32) as u32;
                let low: u32 = val as u32;

                self.state.regs.set(llil.dest_reg_low, low as u64);
                self.state.regs.set(llil.dest_reg_high, high as u64);
                info!("0x{:x} Set registers to 0x{:x} and 0x{:x}", self.state.addr, low, high);
            }
            Push(llil) => {
                match llil.expr {
                    Reg(r) => {
                        info!("0x{:x} Pushing register {}", self.state.addr, r.name);
                        self.state.memory.store(self.state.regs.rsp, self.state.regs.get(r.name));
                    },
                    _ => info!("0x{:x} Pushing other", self.state.addr),
                }
                self.state.regs.rsp -= 8;
            }
            If(llil) => {
                let result: u64 = eval_expression(llil.condition, &self.state);
                info!("0x{:x} Compare returned 0x{:x}", self.state.addr, result);
                if result == 0 {
                    self.state.addr = llil.target_false;
                    info!(" > Branching false");
                } else {
                    self.state.addr = llil.target_true;
                    info!(" > Branching true");
                }
            }
            Store(llil) => {
                let val: u64 = eval_expression(llil.source_expr, &self.state);
                let addr: u64 = eval_expression(llil.dest_mem_expr, &self.state);

                self.state.memory.store(addr, val);

                info!("0x{:x} Stored 0x{:x} at 0x{:x}", self.state.addr, val, addr);
            }
            Call(llil) => {
                match llil.target {
                    Value(v) => {
                        info!("Calling procedure (unimplemented)");
                    },
                    _ => error!("0x{:x} Calling other", self.state.addr),
                }
            }
            Goto(llil) => {
                info!("0x{:x} Goto instruction at {}", self.state.addr, llil.target);
                self.state.index = llil.target as usize - 1;
            }
            _ => {
                error!("0x{:x} Unimplemented instruction", self.state.addr);
            }
        }

        self.state.addr = self.program.inst_after(self.state.addr).expect("Failed to get next instruction").addr;
    }
}
