use binja::architecture::{CoreArchitecture, Register, RegisterInfo};
use binja::llil::{Expression, ValueExpr, Finalized, NonSSA, RegularNonSSA};
use state::State;

pub enum Expr {
    Reg(Reg),
    Flag(Flag),

    Load(Load),

    Value(u64),

    Add(Arithmetic), Sub(Arithmetic), And(Arithmetic), Or(Arithmetic), Xor(Arithmetic), Mul(Arithmetic), Divu(Arithmetic), Divs(Arithmetic), Modu(Arithmetic), Mods(Arithmetic),
    Lsl(Arithmetic), Lsr(Arithmetic), Asr(Arithmetic), Rol(Arithmetic), Ror(Arithmetic),
    MulsDp(Arithmetic), MuluDp(Arithmetic),
    DivuDp(DivDp), DivsDp(DivDp), ModuDp(DivDp), ModsDp(DivDp), 

    CmpE(Cmp),
    CmpSlt(Cmp),
    CmpSle(Cmp),
    CmpSge(Cmp),
    CmpSgt(Cmp),
    CmpNe(Cmp),
    CmpUlt(Cmp),
    CmpUle(Cmp),
    CmpUge(Cmp),
    CmpUgt(Cmp),

    Undef(Undef),
}

pub struct Reg {
    pub name: String
}

pub struct Load {
    pub source_mem: Box<self::Expr>,
}

pub struct Flag {
    pub name: String
}

pub struct Value {
    pub value: String
}

pub struct Cmp {
    pub left: Box<self::Expr>, 
    pub right: Box<self::Expr>
}

pub struct DivDp {
    pub high: Box<self::Expr>,
    pub low: Box<self::Expr>,
    pub right: Box<self::Expr>,
}

pub struct Undef {
    pub expr: String,
}

pub struct Arithmetic { pub left: Box<self::Expr>, pub right: Box<self::Expr> }

pub fn build_expression(expr: &Expression<CoreArchitecture, Finalized, NonSSA<RegularNonSSA>, ValueExpr>) -> Expr {
    use llil::ExprInfo::*;

    match expr.info() {
        Pop(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        FlagBit(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Adc(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Sbb(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Rlc(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Rrc(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Neg(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Not(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Sx(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Zx(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        LowPart(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        FlagCond(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        FlagGroup(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        BoolToInt(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        Unimpl(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),
        UnimplMem(ref op) => Expr::Undef(self::Undef {expr: String::from(format!("{:?}", expr))}),

        Reg(ref op) => {
            Expr::Reg(self::Reg {
                name: String::from(format!("{:?}", op.source_reg()))
            })
        }

        Flag(ref op) => {
            Expr::Flag(self::Flag {
                name: String::from(format!("{:?}", expr))
            })
        }

        Load(ref op) => {
            Expr::Load(self::Load {
                source_mem: Box::new(build_expression(&op.source_mem_expr()))
            })
        }

        Const(ref op) | ConstPtr(ref op) => {
            Expr::Value(op.value())
        }


        CmpE (ref op) => {Expr::CmpE(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        CmpSlt (ref op) => {Expr::CmpSlt(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        CmpSle (ref op) => {Expr::CmpSle(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        CmpSge (ref op) => {Expr::CmpSge(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        CmpSgt (ref op) => {Expr::CmpSgt(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        
        CmpNe (ref op) => {Expr::CmpNe(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        CmpUlt (ref op) => {Expr::CmpUlt(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        CmpUle (ref op) => {Expr::CmpUle(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        CmpUge (ref op) => {Expr::CmpUge(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        CmpUgt (ref op) => {Expr::CmpUgt(self::Cmp {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}

        Add (ref op) => {Expr::Add(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Sub (ref op) => {Expr::Sub(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        And (ref op) => {Expr::And(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Or (ref op) => {Expr::Or(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Xor (ref op) => {Expr::Xor(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Mul (ref op) => {Expr::Mul(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Divu (ref op) => {Expr::Divu(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Divs (ref op) => {Expr::Divs(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Modu (ref op) => {Expr::Modu(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Mods (ref op) => {Expr::Mods(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}

        Lsl (ref op) => {Expr::Lsl(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Lsr (ref op) => {Expr::Lsr(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Asr (ref op) => {Expr::Asr(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Rol (ref op) => {Expr::Rol(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        Ror (ref op) => {Expr::Ror(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}

        MulsDp (ref op) => {Expr::MulsDp(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}
        MuluDp (ref op) => {Expr::MuluDp(self::Arithmetic {left: Box::new(build_expression(&op.left())), right: Box::new(build_expression(&op.right()))})}

        DivuDp (ref op) => {Expr::DivuDp(self::DivDp {low: Box::new(build_expression(&op.low())), high: Box::new(build_expression(&op.high())), right: Box::new(build_expression(&op.right()))})}
        DivsDp (ref op) => {Expr::DivsDp(self::DivDp {low: Box::new(build_expression(&op.low())), high: Box::new(build_expression(&op.high())), right: Box::new(build_expression(&op.right()))})}
        ModuDp (ref op) => {Expr::ModuDp(self::DivDp {low: Box::new(build_expression(&op.low())), high: Box::new(build_expression(&op.high())), right: Box::new(build_expression(&op.right()))})}
        ModsDp (ref op) => {Expr::ModsDp(self::DivDp {low: Box::new(build_expression(&op.low())), high: Box::new(build_expression(&op.high())), right: Box::new(build_expression(&op.right()))})}

        
        Undef(ref op) => {
            Expr::Undef(self::Undef {
                expr: String::from(format!("{:?}", expr))
            })
        }
    }
}

pub fn eval_expression<'a>(expr: Expr, state: &State) -> u64 {
    match expr {
        Expr::Value(v) => v,
        Expr::Reg(r) => state.regs.get(r.name),
        Expr::Load(l) => eval_expression(*l.source_mem, state),

        Expr::CmpE(s) => if eval_expression(*s.left, state) == eval_expression(*s.right, state) {1} else {0},
        Expr::CmpSlt(s) => if (eval_expression(*s.left, state) as i64) < (eval_expression(*s.right, state) as i64) {1} else {0},
        Expr::CmpSle(s) => if (eval_expression(*s.left, state) as i64) <= (eval_expression(*s.right, state) as i64) {1} else {0},
        Expr::CmpSge(s) => if (eval_expression(*s.left, state) as i64) >= (eval_expression(*s.right, state) as i64) {1} else {0},
        Expr::CmpSgt(s) => if (eval_expression(*s.left, state) as i64) > (eval_expression(*s.right, state) as i64) {1} else {0},

        Expr::CmpNe(s) => if eval_expression(*s.left, state) != eval_expression(*s.right, state) {1} else {0},
        Expr::CmpUlt(s) => if eval_expression(*s.left, state) < eval_expression(*s.right, state) {1} else {0},
        Expr::CmpUle(s) => if eval_expression(*s.left, state) <= eval_expression(*s.right, state) {1} else {0},
        Expr::CmpUge(s) => if eval_expression(*s.left, state) >= eval_expression(*s.right, state) {1} else {0},
        Expr::CmpUgt(s) => if eval_expression(*s.left, state) > eval_expression(*s.right, state) {1} else {0},

        Expr::Add(s) => eval_expression(*s.left, state).overflowing_add(eval_expression(*s.right, state)).0,
        Expr::Sub(s) => eval_expression(*s.left, state).overflowing_sub(eval_expression(*s.right, state)).0,
        Expr::And(s) => eval_expression(*s.left, state) & eval_expression(*s.right, state),
        Expr::Or(s) => eval_expression(*s.left, state) | eval_expression(*s.right, state),
        Expr::Xor(s) => eval_expression(*s.left, state) ^ eval_expression(*s.right, state),
        Expr::Mul(s) => eval_expression(*s.left, state).overflowing_mul(eval_expression(*s.right, state)).0,
        Expr::Divu(s) => eval_expression(*s.left, state) / eval_expression(*s.right, state),
        Expr::Divs(s) => eval_expression(*s.left, state) / eval_expression(*s.right, state),
        Expr::Modu(s) => eval_expression(*s.left, state) % eval_expression(*s.right, state),
        Expr::Mods(s) => eval_expression(*s.left, state) % eval_expression(*s.right, state),
        
        Expr::Lsl(s) => eval_expression(*s.left, state) << eval_expression(*s.right, state),
        Expr::Lsr(s) => eval_expression(*s.left, state) >> eval_expression(*s.right, state),
        Expr::Asr(s) => eval_expression(*s.left, state) << eval_expression(*s.right, state),
        Expr::Rol(s) => eval_expression(*s.left, state) << eval_expression(*s.right, state),
        Expr::Ror(s) => eval_expression(*s.left, state) >> eval_expression(*s.right, state),

        Expr::MulsDp(s) => eval_expression(*s.left, state).overflowing_mul(eval_expression(*s.right, state)).0,
        Expr::MuluDp(s) => eval_expression(*s.left, state).overflowing_mul(eval_expression(*s.right, state)).0,

        Expr::Undef(s) => {error!("Undef expr {:?}", s.expr); 1},
        _ => {error!("Unimplemented expr"); 1}
    }
}
