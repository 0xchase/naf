use program::*;
use expression::*;

#[derive(Clone)]
pub struct StackTaintSource {
    pub instr: u64,
    pub index: usize,
    pub addr: u64,
}

#[derive(Clone)]
pub struct RegTaintSource {
    pub instr: u64,
    pub index: usize,
    pub reg: String,
}

#[derive(Clone)]
pub struct TaintState {
    pub addr: u64,
    pub index: usize,
    pub regs_tainted: Vec<String>,
    pub stack_tainted: Vec<u64>,
    pub reg_taint_sources: Vec<RegTaintSource>,
    pub stack_taint_sources: Vec<StackTaintSource>,
    //pub expressions_tainted: Vec<Expr>,
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
                        reg_taint_sources: Vec::new(),
                        stack_taint_sources: Vec::new(),
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
                reg_taint_sources: Vec::new(),
                stack_taint_sources: Vec::new(),
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
    pub fn taint_reg(state: &mut TaintState, reg: String, instr: u64, index: usize) {
        state.reg_taint_sources.push(RegTaintSource{
            instr: instr,
            index: index,
            reg: reg,
        })
    }

    pub fn taint_stack(state: &mut TaintState, addr: u64, instr: u64, index: usize) {
        state.stack_taint_sources.push(StackTaintSource{
            instr: instr,
            index: index,
            addr: addr,
        })
    }

    // Step the taint state over the next LLIL instruction
    pub fn step(&mut self) {

        let mut new_states: Vec<TaintState> = Vec::new();

        for (idx, state) in self.states.iter_mut().enumerate() {
            use LlilInst::*;
            
            let inst: Inst = self.program.inst_at(state.addr).expect("No such instruction");
            let mut jumped: bool = false;

            // Taint register or stack address if at a taint source
            for source in state.reg_taint_sources.iter() {
                if source.instr == state.addr && source.index == state.index {
                    if !state.regs_tainted.contains(&source.reg) {
                        state.regs_tainted.push(source.reg.clone());
                    }
                }
            }
            for source in state.stack_taint_sources.iter() {
                if source.instr == state.addr && source.index == state.index {
                    if !state.stack_tainted.contains(&source.addr) {
                        state.stack_tainted.push(source.addr.clone());
                    }
                }
            }

            // This match statement should have an entry for each LLIL instruction
            match inst.llil {
                SetReg(llil) => {
                    let reg_clone: String = llil.reg.clone();
                    if Self::expression_tainted(&state, llil.expr, idx) {
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
                    if Self::expression_tainted(state, llil.source_expr, idx) {
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
                    if state.stack_tainted.contains(&llil.addr) {
                        if !state.regs_tainted.contains(&rflags) {
                            state.regs_tainted.push(String::from("rflags"));
                        }
                    } else {
                        state.regs_tainted.retain(|r| r.ne(&rflags));
                    }
                }
                Store(llil) => {
                    // Integrate with emulator to properly taint
                    if Self::expression_tainted(state, llil.source_expr, idx) {
                        //state.expressions_tainted.push(llil.dest_mem_expr);
                    } else {
                        // todo: if not evaluated using emulator add expr comparator
                        // state.expressions_tainted.retain(|e| e.ne(llil.dest_mem_expr));
                    }
                }
                Push(llil) => {
                    if Self::expression_tainted(state, llil.expr, idx) {
                        // Would need emulator to taint value at ($rsp), something like:
                        // self.state.stack_tainted.push(self.state.emulator.state.regs.rsp);
                    }
                }
                Jump(llil) => {
                    state.addr = self.program.inst_at(llil.addr).expect("Failed to make jump").addr;
                    jumped = true;
                }
                JumpTo(llil) => {
                    state.addr = self.program.inst_at(llil.addr).expect("Failed to make jump").addr;
                    jumped = true;
                }
                Call(llil) => {
                    // Todo check if args are tainted and taint return value
                }
                Ret(llil) => {
                    info!("Trying to jump to 0x{:x}", llil.addr);
                    state.addr = self.program.inst_at(llil.addr).expect("Failed to make jump").addr;
                    jumped = true;
                }
                If(llil) => {
                    state.addr = self.program.inst_at(llil.target_true).expect("Failed to make jump").addr;
                    new_states.push(
                        TaintState {
                            addr: llil.target_false,
                            index: 1,
                            regs_tainted: state.regs_tainted.clone(),
                            stack_tainted: state.stack_tainted.clone(),
                            reg_taint_sources: state.reg_taint_sources.clone(),
                            stack_taint_sources: state.stack_taint_sources.clone(),
                        });
                    jumped = true;
                }
                Goto(llil) => {
                    state.addr = self.program.inst_at(llil.target).expect("Failed to make jump").addr;
                    jumped = true;
                }

                // Todo: Implement the other instructions

                _ => {
                    error!("0x{:x} Unimplemented instruction", state.addr);
                }
            }

            // TODO Change to be step after, not instruction
            if !jumped {
                state.addr = self.program.inst_after(state.addr).expect("Failed to get next instruction").addr;
            }
        }

        // Add new states from queue
        for state in new_states {
            self.states.push(state);
        }

    }

    // This function should return true if any of the expression elements are tainted
    pub fn expression_tainted(state: &TaintState, expr: Expr, state_id: usize) -> bool {
        match expr {
            Expr::Value(_v) => false,
            Expr::Reg(r) => state.regs_tainted.contains(&r.name),
            Expr::Load(l) => Self::load_tainted(&state, *l.source_mem, state_id),
    
            Expr::CmpE(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::CmpSlt(s) => Self::expression_tainted(&state, *s.left, state_id) < Self::expression_tainted(&state, *s.right, state_id),
            Expr::CmpSle(s) => Self::expression_tainted(&state, *s.left, state_id) <= Self::expression_tainted(&state, *s.right, state_id),
            Expr::CmpSge(s) => Self::expression_tainted(&state, *s.left, state_id) >= Self::expression_tainted(&state, *s.right, state_id),
            Expr::CmpSgt(s) => Self::expression_tainted(&state, *s.left, state_id) > Self::expression_tainted(&state, *s.right, state_id),
    
            Expr::CmpNe(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::CmpUlt(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::CmpUle(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::CmpUge(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::CmpUgt(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
    
            Expr::Add(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Sub(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::And(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Or(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Xor(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Mul(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Divu(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Divs(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Modu(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Mods(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            
            Expr::Lsl(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Lsr(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Asr(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Rol(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::Ror(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
    
            Expr::MulsDp(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
            Expr::MuluDp(s) => Self::expression_tainted(&state, *s.left, state_id) || Self::expression_tainted(&state, *s.right, state_id),
    
            Expr::Undef(s) => {error!("Undef expr {:?}", s.expr); false},
            _ => {error!("Unimplemented expr"); false}
        }
    }

    // Checks if an expression used for load was tainted
    pub fn load_tainted(state: &TaintState, expr: Expr, state_id: usize) -> bool {
        match expr {
            Expr::Value(_v) => false,
            Expr::Reg(r) => state.regs_tainted.contains(&r.name),
            Expr::Load(l) => Self::load_tainted(&state, *l.source_mem, state_id),
    
            Expr::CmpE(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::CmpSlt(s) => Self::load_tainted(&state, *s.left, state_id) < Self::load_tainted(&state, *s.right, state_id),
            Expr::CmpSle(s) => Self::load_tainted(&state, *s.left, state_id) <= Self::load_tainted(&state, *s.right, state_id),
            Expr::CmpSge(s) => Self::load_tainted(&state, *s.left, state_id) >= Self::load_tainted(&state, *s.right, state_id),
            Expr::CmpSgt(s) => Self::load_tainted(&state, *s.left, state_id) > Self::load_tainted(&state, *s.right, state_id),
    
            Expr::CmpNe(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::CmpUlt(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::CmpUle(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::CmpUge(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::CmpUgt(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
    
            Expr::Add(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Sub(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::And(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Or(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Xor(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Mul(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Divu(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Divs(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Modu(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Mods(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            
            Expr::Lsl(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Lsr(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Asr(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Rol(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::Ror(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
    
            Expr::MulsDp(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
            Expr::MuluDp(s) => Self::load_tainted(&state, *s.left, state_id) || Self::load_tainted(&state, *s.right, state_id),
    
            Expr::Undef(s) => {error!("Undef expr {:?}", s.expr); false},
            _ => {error!("Unimplemented expr"); false}
        }
    }
    
}
