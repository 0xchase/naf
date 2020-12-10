use program::*;
use expression::*;

pub struct TaintState {
    pub addr: u64,
    pub index: usize,
    pub regs_tainted: Vec<String>,
    pub stack_tainted: Vec<u64>,
    pub expressions_tainted: Vec<Expr>,
}

pub struct TaintTracker<'a> {
    pub program: &'a Program<'a>,
    pub states: Vec<TaintState>,
}

impl<'a> TaintTracker<'a> {

    // Initialize a taint tracker at a given function
    pub fn track_fun(program: &'a Program, name: String) -> TaintTracker<'a> {
        for function in program.functions() {
            if function.name.eq(&name) {

                let temp = function.llil_start();

                return TaintTracker {
                    program: program,
                    states: vec![TaintState {
                        addr: temp,
                        index: 1,
                        regs_tainted: Vec::new(),
                        stack_tainted: Vec::new(),
                        expressions_tainted: Vec::new(),
                    }],
                }
            }
        }
        return TaintTracker {
            program: program,
            states: vec![TaintState {
                addr: 0,
                index: 1,
                regs_tainted: Vec::new(),
                stack_tainted: Vec::new(),
                expressions_tainted: Vec::new(),
            }],
        }
    }
    // Initialize a taint tracker at the entry point
    pub fn entry(program: &'a Program) -> TaintTracker<'a> {
        return Self::track_fun(program, String::from("_start"));
    }

    // Initialize a taint tracker at the main function
    pub fn main(program: &'a Program) -> TaintTracker<'a> {
        return Self::track_fun(program, String::from("main"));
    }

    // Todo: Add functions so the user can specify taint sources
    pub fn taint_reg() {
        
    }

    // Step the taint state over the next LLIL instruction
    pub fn step(&mut self) {
        for state in self.states.iter() {
            use LlilInst::*;
            
            let inst: Inst = self.program.inst_at(state.addr).expect("No such instruction");

            // This match statement should have an entry for each LLIL instruction
            match inst.llil {
                SetReg(llil) => {
                    let reg_clone: String = llil.reg.clone();
                    if self.expression_tainted(llil.expr, state) {
                        // Right hand side expression is tainted, so taint the destination register
                        if !state.regs_tainted.contains(&reg_clone) {
                            state.regs_tainted.push(llil.reg);
                        }
                    } else {
                        state.regs_tainted.retain(|r| r.ne(&reg_clone));
                        // Right hand side expression not tainted, so don't propogate taint and remove taint from destination if previously tainted
                    }
                }
                SetRegSplit(llil) => {
                    let high_reg_clone = llil.dest_reg_high.clone(); 
                    let low_reg_clone = llil.dest_reg_low.clone();
                    if self.expression_tainted(llil.source_expr, state) {
                        // Right hand side expression is tainted, so taint the destination register
                        if !state.regs_tainted.contains(&llil.dest_reg_high) {
                            state.regs_tainted.push(llil.dest_reg_high);
                        }
                        if !state.regs_tainted.contains(&llil.dest_reg_low) {
                            state.regs_tainted.push(llil.dest_reg_low);
                        }
                    } else {
                        state.regs_tainted.retain(|r| r.ne(&high_reg_clone) && r.ne(&low_reg_clone) );
                        // Right hand side expression not tainted, so don't propogate taint and remove taint from destination if previously tainted
                    }
                }
                SetFlag(llil) => {
                    let rflags = String::from("rflags");
                    if  !state.regs_tainted.contains(&rflags) {
                        state.regs_tainted.push(String::from("rflags"));
                    }
                }
                Store(llil) => {
                    if self.expression_tainted(llil.source_expr, state) {
                        state.expressions_tainted.push(llil.dest_mem_expr);
                    }
                }
                Push(llil) => {
                    if self.expression_tainted(llil.expr, state) {
                        // Would need emulator to push value at ($rsp), something like:
                        // self.state.stack_tainted.push(self.state.emulator.state.regs.rsp);
                    }
                }
                Jump(llil) => {
                    state.addr = self.program.inst_at(llil.addr).expect("Failed to make jump").addr;
                    return;
                }
                JumpTo(llil) => {
                    state.addr = self.program.inst_at(llil.addr).expect("Failed to make jump").addr;
                    return;
                }
                Call(llil) => {
                    // Todo check if args are tainted and taint return value
                }
                Ret(llil) => {
                    state.addr = self.program.inst_at(llil.addr).expect("Failed to make jump").addr;
                    return;
                }
                If(llil) => {
                    state.addr = self.program.inst_at(llil.target_true).expect("Failed to make jump").addr;
                    return;
                }
                Goto(llil) => {
                    state.addr = self.program.inst_at(llil.target).expect("Failed to make jump").addr;
                    return;
                }

                // Todo: Implement the other instructions

                _ => {
                    error!("0x{:x} Unimplemented instruction", state.addr);
                }
            }

            // TODO Change to be step after, not instruction
            state.addr = self.program.inst_after(state.addr).expect("Failed to get next instruction").addr;
        }
    }

    // This function should return true if any of the expression elements are tainted
    pub fn expression_tainted(&self, expr: Expr, state: &TaintState) -> bool {
        match expr {
            Expr::Value(v) => state.stack_tainted.contains(&v),
            Expr::Reg(r) => state.regs_tainted.contains(&r.name),
            Expr::Load(l) => self.expression_tainted(*l.source_mem, state),
    
            Expr::CmpE(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::CmpSlt(s) => self.expression_tainted(*s.left, state) < self.expression_tainted(*s.right, state),
            Expr::CmpSle(s) => self.expression_tainted(*s.left, state) <= self.expression_tainted(*s.right, state),
            Expr::CmpSge(s) => self.expression_tainted(*s.left, state) >= self.expression_tainted(*s.right, state),
            Expr::CmpSgt(s) => self.expression_tainted(*s.left, state) > self.expression_tainted(*s.right, state),
    
            Expr::CmpNe(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::CmpUlt(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::CmpUle(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::CmpUge(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::CmpUgt(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
    
            Expr::Add(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Sub(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::And(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Or(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Xor(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Mul(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Divu(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Divs(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Modu(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Mods(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            
            Expr::Lsl(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Lsr(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Asr(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Rol(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::Ror(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
    
            Expr::MulsDp(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
            Expr::MuluDp(s) => self.expression_tainted(*s.left, state) || self.expression_tainted(*s.right, state),
    
            Expr::Undef(s) => {error!("Undef expr {:?}", s.expr); false},
            _ => {error!("Unimplemented expr"); false}
        }
    }
}
