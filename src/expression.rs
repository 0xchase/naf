use binja::architecture::{CoreArchitecture, Register, RegisterInfo};
use binja::llil::{Expression, ValueExpr, Finalized, NonSSA, RegularNonSSA};
use state::State;

pub enum Expr {
    Reg(Reg),
    Value(u64),

    Add(Arithmetic), Sub(Arithmetic), And(Arithmetic), Or(Arithmetic), Xor(Arithmetic), Mul(Arithmetic), Divu(Arithmetic), Divs(Arithmetic), Modu(Arithmetic), Mods(Arithmetic),
    Lsl(Arithmetic), Lsr(Arithmetic), Asr(Arithmetic), Rol(Arithmetic), Ror(Arithmetic),
    MulsDp(Arithmetic), MuluDp(Arithmetic),
    DivuDp(DivDp), DivsDp(DivDp), ModuDp(DivDp), ModsDp(DivDp), 

    Undef(Undef),
}

pub struct Reg {
    pub name: String
}

pub struct Value {
    pub value: String
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
        Reg(ref op) => {
            Expr::Reg(self::Reg {
                name: String::from(format!("{:?}", op.source_reg()))
            })
        }
        
        Const(ref op) | ConstPtr(ref op) => {
            Expr::Value(op.value())
        }

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

        _ => {
            Expr::Undef(self::Undef {
                expr: String::from(format!("{:?}", expr))
            })
        }
    }
}

pub fn eval_expression<'a>(expr: Expr, state: &State<'a>) -> u64 {
    match expr {
        Expr::Value(v) => v,
        Expr::Reg(r) => state.regs.get(r.name),
        Expr::Sub(s) => eval_expression(*s.left, state) - eval_expression(*s.right, state),
        Expr::Add(s) => {
            let temp1 = eval_expression(*s.left, state);
            let temp2 = eval_expression(*s.right, state);
            info!(" > Should add 0x{:x} and 0x{:x}", temp1, temp2);
            return 50;
        },

        _ => {error!("Unknown expr"); 1}
    }
}
