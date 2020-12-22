use binja::binaryview::{BinaryView, BinaryViewExt};
use binja::llil::{Instruction, Expression, ValueExpr, Finalized, NonSSA, LiftedNonSSA};
use binja::architecture::{CoreArchitecture, Register, RegisterInfo};
use symbolic_state::*;
use bitvector::*;
use z3;

pub struct SymbolicExecutor<'a, 'bv> {
    pub view: &'a BinaryView,
    pub func_addr: u64,
    pub state: SymbolicState<'bv>,
    pub index: usize,
}

impl<'a, 'bv> SymbolicExecutor<'a, 'bv> {

    pub fn entry(view: &'a BinaryView) -> SymbolicExecutor<'a, 'bv> {
        for function in &view.functions() {
            if function.symbol().name().to_ascii_lowercase().eq("main") {

                return SymbolicExecutor {
                    view: view,
                    state: SymbolicState::new(&z3::Config::new()),
                    func_addr: function.start(),
                    index: 0,
                };
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
                let val = self.eval_expr(&self.state, &op.source_expr());

                match val {
                    BV::Concrete(c) => info!("0x{:x} Set reg to something", &op.address()),
                    BV::Symbolic(s) => info!("0x{:x} Set reg to symbolic value", &op.address()),
                    BV::Expression(_) => {}
                }

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
                let _temp = &op.source_expr();
                error!("0x{:x} Set flag", &op.address());
            }
            Store(op) => {
                let dest;
                let val;

                dest = self.eval_expr(&self.state, &op.dest_mem_expr());
                val = self.eval_expr(&self.state, &op.source_expr());

                match (val, dest) {
                    (BV::Concrete(v), BV::Concrete(d)) => {
                        //self.state.memory.store(d, v);
                        info!("0x{:x} Stored value at addresss", &op.address());
                    }
                    _ => info!("0x{:x} Stored using symbolic value(s)", &op.address()),
                }

            }
            Push(op) => {
                let val = self.eval_expr(&self.state, &op.operand());

                //self.state.memory.store(self.state.regs.rsp, val);
                //self.state.regs.rsp -= 8;
                match val {
                    BV::Concrete(c) => info!("0x{:x} Pushed value concrete", &op.address()),
                    BV::Symbolic(s) => info!("0x{:x} Pushed symbolic value", &op.address()),
                    BV::Expression(_) => {}
                }
            }
            Jump(op) => {
                let _temp = &op.target();
            }
            JumpTo(op) => {
                let _temp = &op.target();
            }
            Call(op) => {
                let _temp = &op.target();
            }
            Ret(op) => {
                let _temp = &op.target();
            }
            If(op) => {
                let _temp = &op.condition();
            }
            Value(e, _) => {
                let _temp = &e;
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

    pub fn eval_expr(&self, state: &SymbolicState, expr: &Expression<CoreArchitecture, Finalized, binja::llil::NonSSA<binja::llil::RegularNonSSA>, ValueExpr>) -> BV<'bv> {
        use binja::llil::ExprInfo::*;

        match expr.info() {
            //Const(c) => BV::Concrete(BVV::from_u64(&self.ctx, c.value())),
            //ConstPtr(p) => BV::Concrete(BVV::from_u64(&self.ctx, p.value())),

            /*
            Pop(s) => BV::Concrete(
                self.state.memory.load(
                    match &self.state.regs.rsp {
                        BV::Concrete(v) => BVV::from_u64(*v),
                        BV::Symbolic(s) => BVV::from_u64(0),
                    }
                )
            ),
            */
            //Reg(r) => self.state.regs.get(format!("{:?}", r.source_reg())),

            _ => BV::Concrete(BVV::from_u64(0)),
            /*
            Add(s) => {
                match (self.eval_expression(&s.left()), self.eval_expression(&s.right())) {
                    (BV::Concrete(v1), BV::Concrete(v2)) => BV::Concrete(v1.overflowing_add(v2).0),
                    _ => BV::Concrete(BVV::from_u64(0)),

                }
            }
            */

            //self.eval_expression(&s.left()).overflowing_sub(eval_expression(&s.right())).0,
    /*
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

            Load(l) => eval_expression(&l.source_mem_expr(), state),

            CmpE(s) => if eval_expression(&s.left(), state) == eval_expression(&s.right(), state) {1} else {0},
            CmpSlt(s) => if eval_expression(&s.left(), state) < eval_expression(&s.right(), state) {1} else {0},
            CmpSle(s) => if eval_expression(&s.left(), state) <= eval_expression(&s.right(), state) {1} else {0},
            CmpSge(s) => if eval_expression(&s.left(), state) >= eval_expression(&s.right(), state) {1} else {0},
            CmpSgt(s) => if eval_expression(&s.left(), state) > eval_expression(&s.right(), state) {1} else {0},

            CmpNe(s) => if eval_expression(&s.left(), state) != eval_expression(&s.right(), state) {1} else {0},
            CmpUlt(s) => if eval_expression(&s.left(), state) < eval_expression(&s.right(), state) {1} else {0},
            CmpUle(s) => if eval_expression(&s.left(), state) <= eval_expression(&s.right(), state) {1} else {0},
            CmpUge(s) => if eval_expression(&s.left(), state) >= eval_expression(&s.right(), state) {1} else {0},
            CmpUgt(s) => if eval_expression(&s.left(), state) > eval_expression(&s.right(), state) {1} else {0},


            Add(s) => eval_expression(&s.left(), state).overflowing_add(eval_expression(&s.right(), state)).0,
            Sub(s) => eval_expression(&s.left(), state).overflowing_sub(eval_expression(&s.right(), state)).0,
            And(s) => eval_expression(&s.left(), state) & eval_expression(&s.right(), state),
            Or(s) => eval_expression(&s.left(), state) | eval_expression(&s.right(), state),
            Xor(s) => eval_expression(&s.left(), state) ^ eval_expression(&s.right(), state),
            Mul(s) => eval_expression(&s.left(), state).overflowing_mul(eval_expression(&s.right(), state)).0,
            Divu(s) => eval_expression(&s.left(), state) / eval_expression(&s.right(), state),
            Divs(s) => eval_expression(&s.left(), state) / eval_expression(&s.right(), state),
            Modu(s) => eval_expression(&s.left(), state) % eval_expression(&s.right(), state),
            Mods(s) => eval_expression(&s.left(), state) % eval_expression(&s.right(), state),


            Lsl(s) => eval_expression(&s.left(), state) << eval_expression(&s.right(), state),
            Lsr(s) => eval_expression(&s.left(), state) >> eval_expression(&s.right(), state),
            Asr(s) => eval_expression(&s.left(), state) << eval_expression(&s.right(), state),
            Rol(s) => eval_expression(&s.left(), state) << eval_expression(&s.right(), state),
            Ror(s) => eval_expression(&s.left(), state) >> eval_expression(&s.right(), state),

            MulsDp(s) => eval_expression(&s.left(), state).overflowing_mul(eval_expression(&s.right(), state)).0,
            MuluDp(s) => eval_expression(&s.left(), state).overflowing_mul(eval_expression(&s.right(), state)).0,

            Undef(s) => {error!("Undef expr"); 1},
            */

        }
    }

    pub fn call(&mut self, target: u64) {
        info!("Call");
    }
}
