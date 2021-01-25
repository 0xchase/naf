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
    // Searches for the entry point and starts working with it there.
    pub fn entry(program: &'a Program) -> Emulator<'a> {
        for function in program.functions() {
            if function.name.eq("_start") {

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
    // Searches for the main function and starts working with it there. 
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

    pub fn step(&mut self) -> Result<String, String>{
        use LlilInst::*;
        use expression::Expr::*;
        use expression::eval_expression;
        let indexes: Vec<Index> = self.program.insts_at_addr(self.state.addr).expect("No instructions ad address");
        for index in indexes {
            let inst = index.inst;
        

        
        // let inst: Inst = self.program.inst_at(self.state.addr).expect("No such instruction");
        
        // This is where we check which instruction it is and set the state to the necessary values.
            match inst.llil {
                // set the value of llil thet 
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
                        /* This goes to the wrong address */
                        self.state.addr = llil.target_false;
                        info!(" > Branching false - addr = 0x{:x}", llil.target_false);
                    } else {
                        /* This goes to the wrong address */ 
                        self.state.addr = llil.target_true;
                        info!(" > Branching true - addr = 0x{:x}", llil.target_true);
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
                            info!("0x{:x} Call to function at address 0x{:x}", self.state.addr, v);
                            let state = &mut self.state; 
                            //TODO: Need to somehow get function name
                            /* This is valid but need to figure out whether we are calling a library function or not */
                            let res = self.program.function_at(v);
                            match res {
                                Ok(func) => procedures::call(func.name, state),
                                _ => info!("Couldn't find function value."),
                            };
                            // self.state.memory.store(self.state.addr, self.state.addr);
                            // self.state.regs.rsp -= 8;
                            // self.state.addr = v;
                        },
                        _ => error!("0x{:x} Calling other", self.state.addr),
                    }
                }
                Goto(llil) => {
                    info!("0x{:x} Goto instruction at {}", self.state.addr, llil.target);
                    self.state.index = llil.target as usize - 1;
                }
                Jump(llil) => {
                    info!("0x{:x} Jump instructiion to {}", self.state.addr, llil.addr);
                    self.state.addr = llil.addr;
                }
                Ret(llil) => {
                    info!("0x{:x} Return instruction at {}", self.state.addr, llil.addr);
                    self.state.addr = llil.addr; 
                },
                NoRet() => {
                    info!("0x{:x} NoRet instruction", self.state.addr);
                },
                Syscall() => {
                    info!("0x{:x} Syscall instruction", self.state.addr);
                },
                Nop() => {
                    info!("0x{:x} Nop instruction", self.state.addr);
                },
                Bp() => {
                    info!("0x{:x} Bp instruction", self.state.addr); 
                },
                Trap() => {
                    info!("0x{:x} Trap instruction", self.state.addr); 
                },
                LlilInst::Undef() => {
                    info!("0x{:x} Undef instruction", self.state.addr);
                },
                _ => {
                    error!("0x{:x} Unimplemented instruction", self.state.addr);
                }
            }
        }
        let nextInst = self.program.inst_after(self.state.addr);
        self.state.addr = match nextInst {
            Ok(inst) => inst.addr,
            Err(error) => return Err(String::from("Failed to get next instruction")),
        };
        return Ok(String::from("Successful Step!"));
    }

}
