#[macro_use]
extern crate log;
extern crate binja;
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

use binja::binaryview::{BinaryView};
use binja::command;
use binja::llil;
use program::*;
use debugger::*;
use cpython::{Python};
use project::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CorePluginInit() -> bool {
    binja::logger::init(log::LevelFilter::Trace).expect("Failed to set up logging");
    command::register_for_address("TEST ANALYSIS PLUGIN", "Description goes here", run_plugin1);
    
    true
}

pub fn run_plugin1(bv: &BinaryView, _addr: u64) {
    let gil = Python::acquire_gil();
    run::run(Project::new(bv, gil.python()));
}
