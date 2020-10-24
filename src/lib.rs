#[macro_use]
extern crate log;
extern crate binja;
extern crate riscv_dis;
extern crate rayon;
extern crate z3;
extern crate cpython;

mod ninja;
mod state;
mod expression;
mod liftcheck;
mod procedures;
mod run;
mod solver;
mod debugger;

use binja::binaryview::{BinaryView};
use binja::command;
use binja::llil;
use ninja::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CorePluginInit() -> bool {
    binja::logger::init(log::LevelFilter::Trace).expect("Failed to set up logging");
/*
    command::register_for_address("Rust", "aaa", |bv: &BinaryView, addr: u64| { 
        use llil::ExprInfo::*;
        use llil::VisitorAction;

        for block in &bv.basic_blocks_containing(addr) {
            let func = block.function();

            for inst_addr in block.iter() {
                info!("instruction at {:x}", inst_addr);
            }

            if let Ok(llil) = func.low_level_il() {
                if let Some(inst) = llil.instruction_at(addr) {
                    inst.visit_tree(&mut |e, info| {
                        info!("visiting {:?}", e);

                        if let Add(ref op) = *info {
                            let left = op.left().info();
                            let right = op.right().info();

                            if let (Reg(ref r), Const(ref c)) = (left, right) {
                                info!("reg {:?} added to constant {:x} in expr {:?}",
                                      r.source_reg(), c.value(), e);

                                return VisitorAction::Halt;
                            }
                        }

                        VisitorAction::Descend
                    });
                }
            }
        }
    });

    use binja::function::Function;

    command::register_for_function("LiftCheckFunction", "welp", |_bv: &BinaryView, func: &Function| {
        liftcheck::check_function(func);
        info!("liftcheck: Function check completed");
    });
*/
    command::register_for_address("CHASE PLUGIN", "aaa", run_plugin);
    true
}

pub fn run_plugin(bv: &BinaryView, _addr: u64) {
    //ninja::test(ninja::Program{bv}, addr);

    run::run(Program::new(bv));
}

/*
use state::*;
pub fn test(bv: &BinaryView) {

    let program = Program::new(bv);

    let mut state = State::entry(program);
    state.step();
    state.step();
    state.step();
    state.step();
    state.step();
    state.step();
    state.step();
    state.step();

    for function in program.functions() {
        //info!(" > Analyzing function {} at 0x{:x}", function.name, function.addr);
        if function.name.eq("_start") {
            for block in function.blocks() {
                //info!("  >> Analyzing block at 0x{:x}", block.addr);
                for inst in block.llil() {
                    match inst {
                        ninja::LlilInst::Call(c) => {
                            info!("0x{:x} Calling 0x{:x}", c.address, c.target);
                        },
                        _ => {
                            info!("Instruction");
                        }
                    }
                }
            }
        }
    }

}
*/