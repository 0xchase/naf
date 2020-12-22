use binja::binaryview::{BinaryView, BinaryViewExt};
use binja::llil::{Instruction, Expression, ValueExpr, Finalized, NonSSA, LiftedNonSSA};
use binja::architecture::{CoreArchitecture, Register, RegisterInfo};
use state::*;

pub struct Emulator<'a> {
    pub view: &'a BinaryView,
    pub func_addr: u64,
    pub state: State,
    pub index: usize,
}

impl<'a> Emulator<'a> {

    pub fn entry(view: &'a BinaryView) -> Emulator<'a> {
        for function in &view.functions() {
            if function.symbol().name().to_ascii_lowercase().eq("main") {

                return Emulator {
                    view: view,
                    state: State::new(),
                    func_addr: function.start(),
                    index: 0,
                }
            }
        }
        panic!("Failed to find entry point");
    }

    pub fn step(&mut self) {
        if let Some(function) = self.view.functions_at(self.func_addr).iter().last() {
            if let Ok(llil) = function.low_level_il() {
                if self.index >= llil.instruction_count() {
                    self.index = 0;
                    info!("Set index back to 0");
                }
                self.run_inst(llil.instruction_from_idx(self.index));
            }
        }
    }

    pub fn step_count(&mut self, count: usize) {
        if let Some(function) = self.view.functions_at(self.func_addr).iter().last() {
            if let Ok(llil) = function.low_level_il() {
                for i in 0..count {
                    if self.index >= llil.instruction_count() {
                        self.index = 0;
                        info!("Set index back to 0");
                    }
                    self.run_inst(llil.instruction_from_idx(self.index));
                }
            }
        }
    }

    fn run_inst(&mut self, inst: binja::llil::Instruction<binja::architecture::CoreArchitecture, binja::llil::Finalized, binja::llil::NonSSA<binja::llil::RegularNonSSA>>) {
        use binja::llil::InstrInfo::*;

        match inst.info() {
            SetReg(op) => {
                let reg = format!("{:?}", op.dest_reg());
                let val = self.eval_expression(&op.source_expr());

                info!("0x{:x} Set reg {} to 0x{:x}", &op.address(), reg, val);

                self.state.regs.set(reg, val);
            }
            SetRegSplit(op) => {
                /*
                let val = eval_expression(&op.source_expr(), &self.state);
                let high: u32 = (val >> 32) as u32;
                let low: u32 = val as u32;

                let reg_high = format!("{:?}", op.dest_reg_high());
                let reg_low = format!("{:?}", op.dest_reg_low());
                
                self.state.regs.set(reg_low, BitVector::Concrete(0));
                self.state.regs.set(reg_high, BitVector::Concrete(0));

                info!("0x{:x} Set regs to 0x{:x} and 0x{:x}", &op.address(), low, high);
                */
            }
            SetFlag(op) => {
                let temp = &op.source_expr();
                error!("0x{:x} Set flag", &op.address());
            }
            Store(op) => {
                let dest;
                let val;
                
                dest = self.eval_expression(&op.dest_mem_expr());
                val = self.eval_expression(&op.source_expr());

                info!("0x{:x} Stored 0x{:x} at 0x{:x}", &op.address(), val, dest);

                self.state.memory.store(dest, val);
                
            }
            Push(op) => {
                let val = self.eval_expression(&op.operand());
                
                self.state.memory.store(self.state.regs.rsp, val);
                self.state.regs.rsp -= 8;
            }
            Jump(op) => {
                let temp = &op.target();
            }
            JumpTo(op) => {
                let temp = &op.target();
            }
            Call(op) => {
                let temp = &op.target();
            }
            Ret(op) => {
                let temp = &op.target();
            }
            If(op) => {
                let temp = &op.condition();
            }
            Value(e, _) => {
                let temp = &e;
            }
            Nop(op) => {
                error!("Hit nop");
            }
            NoRet(op) => {
                error!("Hit noret");
            }
            Goto(op) => {
                error!("Hit goto");
            }
            Syscall(op) => {
                error!("Hit syscall");
            }
            Bp(op) => {
                error!("Hit breakpoint");
            }
            Trap(op) => {
                error!("Hit trap");
            }
            Undef(op) => {
                error!("Undefined instruction");
            }
        }

        self.index += 1;
    }

    pub fn call(&mut self, target: u64) {
        info!("Call");
    }

    fn eval_expression(&self, expr: &Expression<CoreArchitecture, Finalized, binja::llil::NonSSA<binja::llil::RegularNonSSA>, ValueExpr>) -> u64 {
        use binja::llil::ExprInfo::*;
    
        match expr.info() {
            Const(c) => c.value(),
            ConstPtr(p) => p.value(),
    
            Pop(s) => self.state.memory.load(self.state.regs.rip),
            Reg(r) => self.state.regs.get(format!("{:?}", r.source_reg())),
            
    
            Flag(f) => 52,
            FlagBit(f) => 53,
    
            Adc(s) | Sbb(s) | Rlc(s) | Rrc(s) => 54,
            DivuDp(s) | DivsDp(s) | ModuDp(s) | ModsDp(s) => 55,
            Neg(s) | Not(s) => 56,
            Sx(s) | Zx(s) => 57,
            LowPart(s) => 58,
            FlagCond(s) => 59,
            FlagGroup(s) => 60,
            BoolToInt(s) => 61,
            Unimpl(s) => 62,
            UnimplMem(s) => 63,
    
            Load(l) => self.eval_expression(&l.source_mem_expr()),
    
            CmpE(s) => if self.eval_expression(&s.left()) == self.eval_expression(&s.right()) {1} else {0},
            CmpSlt(s) => if self.eval_expression(&s.left()) < self.eval_expression(&s.right()) {1} else {0},
            CmpSle(s) => if self.eval_expression(&s.left()) <= self.eval_expression(&s.right()) {1} else {0},
            CmpSge(s) => if self.eval_expression(&s.left()) >= self.eval_expression(&s.right()) {1} else {0},
            CmpSgt(s) => if self.eval_expression(&s.left()) > self.eval_expression(&s.right()) {1} else {0},
    
            CmpNe(s) => if self.eval_expression(&s.left()) != self.eval_expression(&s.right()) {1} else {0},
            CmpUlt(s) => if self.eval_expression(&s.left()) < self.eval_expression(&s.right()) {1} else {0},
            CmpUle(s) => if self.eval_expression(&s.left()) <= self.eval_expression(&s.right()) {1} else {0},
            CmpUge(s) => if self.eval_expression(&s.left()) >= self.eval_expression(&s.right()) {1} else {0},
            CmpUgt(s) => if self.eval_expression(&s.left()) > self.eval_expression(&s.right()) {1} else {0},
    
            
            Add(s) => self.eval_expression(&s.left()).overflowing_add(self.eval_expression(&s.right())).0,
            Sub(s) => self.eval_expression(&s.left()).overflowing_sub(self.eval_expression(&s.right())).0,
            And(s) => self.eval_expression(&s.left()) & self.eval_expression(&s.right()),
            Or(s) => self.eval_expression(&s.left()) | self.eval_expression(&s.right()),
            Xor(s) => self.eval_expression(&s.left()) ^ self.eval_expression(&s.right()),
            Mul(s) => self.eval_expression(&s.left()).overflowing_mul(self.eval_expression(&s.right())).0,
            Divu(s) => self.eval_expression(&s.left()) / self.eval_expression(&s.right()),
            Divs(s) => self.eval_expression(&s.left()) / self.eval_expression(&s.right()),
            Modu(s) => self.eval_expression(&s.left()) % self.eval_expression(&s.right()),
            Mods(s) => self.eval_expression(&s.left()) % self.eval_expression(&s.right()),
    
            
            Lsl(s) => self.eval_expression(&s.left()) << self.eval_expression(&s.right()),
            Lsr(s) => self.eval_expression(&s.left()) >> self.eval_expression(&s.right()),
            Asr(s) => self.eval_expression(&s.left()) << self.eval_expression(&s.right()),
            Rol(s) => self.eval_expression(&s.left()) << self.eval_expression(&s.right()),
            Ror(s) => self.eval_expression(&s.left()) >> self.eval_expression(&s.right()),
    
            MulsDp(s) => self.eval_expression(&s.left()).overflowing_mul(self.eval_expression(&s.right())).0,
            MuluDp(s) => self.eval_expression(&s.left()).overflowing_mul(self.eval_expression(&s.right())).0,
    
            Undef(s) => {error!("Undef expr"); 1},
            
            _ => 0,
        }
    }
    
}
