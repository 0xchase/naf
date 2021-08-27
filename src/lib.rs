#[macro_use]
extern crate log;
extern crate binaryninja;
//extern crate riscv_dis;
extern crate rayon;
extern crate z3;
extern crate cpython;

mod program;
mod state;
mod expression;
mod procedures;
mod run;
mod solver;
mod debugger;
mod project;
mod python;
mod debugger_ui;
mod emulator;
mod taint_tracker;

use binaryninja::binaryview::{BinaryView, BinaryViewExt};
use binaryninja::command;
use binaryninja::llil;
use program::*;
use debugger::*;
use cpython::{Python};
use project::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CorePluginInit() -> bool {
    binaryninja::logger::init(log::LevelFilter::Trace).expect("Failed to set up logging");
    command::register_for_address("Ninja Binary Analysis Plugin", "Description goes here", run_plugin1);
    
    true
}

pub fn run_plugin1(bv: &BinaryView, _addr: u64) {
    let gil = Python::acquire_gil();
    let arch = bv.default_arch().unwrap();
    info!("Architecture: {:?}", arch.name().as_str()); 
    run::run(Project::new(bv, gil.python()));
}
