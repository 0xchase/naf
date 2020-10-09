use binja::architecture::{CoreArchitecture, Register, RegisterInfo};
use binja::llil::{Expression, ValueExpr, Finalized, NonSSA, RegularNonSSA};

pub enum Expr {
    Reg(Reg),
    Value(u64),
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
        _ => {
            Expr::Undef(self::Undef {
                expr: String::from(format!("{:?}", expr))
            })
        }
    }
}
