use binja::llil::{Instruction, Expression, ValueExpr, Finalized, NonSSA, LiftedNonSSA};
use binja::architecture::{CoreArchitecture, Register, RegisterInfo};
use z3;
use z3::ast;
use std::collections::HashMap;

enum Var {
    u64,
    String,
}

pub fn test() {
    info!("Running solver");

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    // --------------- Stuff is below this line --------------- //

    let mut vars: HashMap<&str, z3::ast::BV> = HashMap::new();

    vars.insert("bitvector1", ast::BV::new_const(&ctx, "bitvector1", 64*4));
    vars.insert("bitvector2", ast::BV::new_const(&ctx, "bitvector2", 64*4));

    vars.get("bitvector1");

    /*
    let i = ast::Int::new_const(&ctx, "x");

    solver.assert(&i._eq(&ast::Int::from_i64(&ctx, -3)));

    let x = ast::BV::from_int(&i, 64);
    assert_eq!(64, x.get_size());

    assert_eq!(solver.check(), SatResult::Sat);
    let model = solver.get_model().unwrap();;

    assert_eq!(-3, model.eval(&x.to_int(true)).unwrap().as_i64().expect("as_i64() shouldn't fail"));
    */

    info!("Done running solver");
}

pub struct Solver<'a> {
    solver: Option<z3::Solver<'a>>,
}

impl<'a> Solver<'a> {
    pub fn new() -> Solver<'a> {
        return Solver {
            solver: None,
        }
    }

    pub fn setup(&self) {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);

    }
}
