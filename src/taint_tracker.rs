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
    // Initialize a taint tracker at the entry point
    pub fn entry(program: &'a Program) -> TaintTracker<'a> {
        for function in program.functions() {
            if function.name.eq("_start") {

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

    // Initialize a taint tracker at the main function
    pub fn main(program: &'a Program) -> TaintTracker<'a> {
        for function in program.functions() {
            if function.name.eq("main") {

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

    // Todo: Add functions so the user can specify taint sources

    // Step the taint state over the next LLIL instruction
    pub fn step(&mut self) {
        use LlilInst::*;
        
        let inst: Inst = self.program.inst_at(self.state.addr).expect("No such instruction");

        // This match statement should have an entry for each LLIL instruction
        match inst.llil {
            SetReg(llil) => {
                if self.expression_tainted(llil.expr) {
                    // Right hand side expression is tainted, so taint the destination register
                } else {
                    // Right hand side expression not tainted, so don't propogate taint and remove taint from destination if previously tainted
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
