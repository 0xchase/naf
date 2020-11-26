use program::*;
use expression::*;

pub struct TaintState {
    pub addr: u64,
    pub index: usize,
    pub regs_tainted: Vec<String>,
    // Todo: Add structure to track stack tainting
}

pub struct TaintTracker<'a> {
    pub program: &'a Program<'a>,
    pub state: TaintState,
}

impl<'a> TaintTracker<'a> {

    // Initialize a taint tracker at a given function
    pub fn track_fun(program: &'a Program, name: String) -> TaintTracker<'a> {
        for function in program.functions() {
            if function.name.eq(&name) {

                let temp = function.llil_start();

                return TaintTracker {
                    program: program,
                    state: TaintState {
                        addr: temp,
                        index: 1,
                        regs_tainted: Vec::new(),
                    },
                }
            }
        }
        return TaintTracker {
            program: program,
            state: TaintState {
                addr: 0,
                index: 1,
                regs_tainted: Vec::new(),
            }
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

    // Step the taint state over the next LLIL instruction
    pub fn step(&mut self) {
        use LlilInst::*;
        
        let inst: Inst = self.program.inst_at(self.state.addr).expect("No such instruction");

        // This match statement should have an entry for each LLIL instruction
        match inst.llil {
            SetReg(llil) => {
                let reg_clone: String = llil.reg.clone(); // This is hideous pls fix
                if self.expression_tainted(llil.expr) {
                    // Right hand side expression is tainted, so taint the destination register
                    if !self.state.regs_tainted.contains(&reg_clone) {
                        self.state.regs_tainted.push(llil.reg);
                    }
                } else {
                    self.state.regs_tainted.retain(|r| r.ne(&reg_clone));
                    // Right hand side expression not tainted, so don't propogate taint and remove taint from destination if previously tainted
                }
            }
            SetRegSplit(llil) => {
                let high_reg_clone = llil.dest_reg_high.clone(); //more hideous
                let low_reg_clone = llil.dest_reg_low.clone();
                if self.expression_tainted(llil.source_expr) {
                    // Right hand side expression is tainted, so taint the destination register
                    if !self.state.regs_tainted.contains(&llil.dest_reg_high) {
                        self.state.regs_tainted.push(llil.dest_reg_high);
                    }
                    if !self.state.regs_tainted.contains(&llil.dest_reg_low) {
                        self.state.regs_tainted.push(llil.dest_reg_low);
                    }
                } else {
                    self.state.regs_tainted.retain(|r| r.ne(&high_reg_clone) && r.ne(&low_reg_clone) );
                    // Right hand side expression not tainted, so don't propogate taint and remove taint from destination if previously tainted
                }
            }
            SetFlag(llil) => {
            }
            Store(llil) => {
            }
            Push(llil) => {
            }
            Jump(llil) => {
            }
            JumpTo(llil) => {
            }
            Call(llil) => {
                if self.expression_tainted(llil.target) {
                    // Add target to tainted branches
                }
            }
            Ret(llil) => {
                // No side effects
            }
            If(llil) => {
                if self.expression_tainted(llil.condition) {
                    // Add dest branches to tainted branches
                }
            }
            Goto(llil) => {
                // No need to propogate taint for this instruction since there aren't any side effects
            }

            // Todo: Implement the other instructions

            _ => {
                error!("0x{:x} Unimplemented instruction", self.state.addr);
            }
        }

        // TODO Change to be step after, not instruction
        self.state.addr = self.program.inst_after(self.state.addr).expect("Failed to get next instruction").addr;
    }

    // This function should return true if any of the expression elements are tainted
    pub fn expression_tainted(&self, expr: Expr) -> bool {
        match expr {
            Expr::Value(v) => false,
            Expr::Reg(r) => self.state.regs_tainted.contains(&r.name),
            Expr::Load(l) => self.expression_tainted(*l.source_mem),
    
            Expr::CmpE(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::CmpSlt(s) => self.expression_tainted(*s.left) < self.expression_tainted(*s.right),
            Expr::CmpSle(s) => self.expression_tainted(*s.left) <= self.expression_tainted(*s.right),
            Expr::CmpSge(s) => self.expression_tainted(*s.left) >= self.expression_tainted(*s.right),
            Expr::CmpSgt(s) => self.expression_tainted(*s.left) > self.expression_tainted(*s.right),
    
            Expr::CmpNe(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::CmpUlt(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::CmpUle(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::CmpUge(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::CmpUgt(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
    
            Expr::Add(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Sub(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::And(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Or(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Xor(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Mul(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Divu(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Divs(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Modu(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Mods(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            
            Expr::Lsl(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Lsr(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Asr(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Rol(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::Ror(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
    
            Expr::MulsDp(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
            Expr::MuluDp(s) => self.expression_tainted(*s.left) || self.expression_tainted(*s.right),
    
            Expr::Undef(s) => {error!("Undef expr {:?}", s.expr); false},
            _ => {error!("Unimplemented expr"); false}
        }
    }
}
