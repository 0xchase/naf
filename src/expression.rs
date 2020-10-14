use binja::architecture::{CoreArchitecture, Register, RegisterInfo};
use binja::llil::{Expression, ValueExpr, Finalized, NonSSA, RegularNonSSA};
use state::State;

pub enum Expr {
    Reg(Reg),
    Value(u64),
    Sub(Sub),
    Add(Add),
    Undef(Undef),
}

pub struct Reg {
    pub name: String
}

pub struct Value {
    pub value: String
}

pub struct Undef {
    pub expr: String,
}

pub struct Sub { pub left: Box<self::Expr>, pub right: Box<self::Expr> }
pub struct Add { pub left: Box<self::Expr>, pub right: Box<self::Expr> }

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
        Sub (ref op) => {
            Expr::Sub(self::Sub {
                left: Box::new(build_expression(&op.left())),
                right: Box::new(build_expression(&op.right())),
            })
        }
        Add (ref op) => {
            Expr::Add(self::Add {
                left: Box::new(build_expression(&op.left())),
                right: Box::new(build_expression(&op.right())),
            })
        }
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
            //info!(" > Should add 0x{:x} and 0x{:x}", temp1, temp2);
            return 50;
        },
        _ => 1
    }
}
