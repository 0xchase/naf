use z3;
use z3::ast;

enum Var {
    u64,
    String,
}

pub fn test() {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    

    let mut vars: Vec<z3::ast::Int> = Vec::new();
    vars.push(ast::Int::new_const(&ctx, 5.to_string()));

    for var in &vars {
        solver.assert(&var.ge(&ast::Int::from_i64(&ctx, 0)));
        solver.assert(&var.le(&ast::Int::from_i64(&ctx, 10)));
    }

    let n1 = ast::Int::from_i64(&ctx, 55);
    let n2 = ast::Int::from_i64(&ctx, 55);


}