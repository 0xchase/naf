#[macro_use]
extern crate log;
extern crate binja;
extern crate riscv_dis;
extern crate rayon;

//use binja::binaryview::{BinaryView, BinaryViewExt, BinaryViewType, BinaryViewTypeExt};
use binja::binaryview::{BinaryView, BinaryViewExt};

use binja::command;

//use binja::llil::{Liftable, LiftedExpr, LiftableWithSize, Mutable, NonSSA, LiftedNonSSA, Label};
use binja::llil;

mod liftcheck;
mod ninja;

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

    //command::register("LiftCheckAllFunctionsParallel", "welp", liftcheck::check_all_functions_parallel);

    true
}

pub fn run_plugin(bv: &BinaryView, addr: u64) {
    ninja::test(ninja::Program{bv}, addr);
}


pub fn test(bv: &BinaryView, addr: u64) {
    for function in &bv.functions() {
        info!("Found function {} at 0x{:x}", function.symbol().name(), function.start());
        if let Ok(llil) = function.low_level_il() {
            let blocks = function.basic_blocks();
            
            for block in blocks.into_iter() {
                info!(" > Found block at 0x{:x}", block.raw_start());
                
                
                for inst_addr in block.iter() {
                    if let Some(inst) = llil.instruction_at(inst_addr) {
                        //info!("  >> Instruction at 0x{:x}", inst_addr);
                    }
                }
                
            }
        }
    }
}