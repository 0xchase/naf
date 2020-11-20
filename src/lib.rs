#[macro_use]
extern crate log;
extern crate binja;
extern crate rayon;
extern crate z3;
extern crate cpython;
extern crate libloading;

pub mod project;
pub mod program;
pub mod emulator;
pub mod state;

mod expression;
mod procedures;
mod run;
mod solver;
mod debugger;
mod python;
mod debugger_ui;

use binja::binaryview::{BinaryView};
use binja::command;
use binja::llil;
use program::*;
use debugger::*;
use cpython::{Python};
use project::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// External cpp functions
extern {
    fn call_cpp();
    fn call_ui();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CorePluginInit() -> bool {
    binja::logger::init(log::LevelFilter::Trace).expect("Failed to set up logging");
    command::register_for_address("TEST ANALYSIS PLUGIN", "Description goes here", run_plugin);
    command::register_for_address("TEST ANALYSIS PLUGIN MESSAGEBOX", "Description goes here", run_ui);
    command::register_for_address("TEST ANALYSIS PLUGIN SCRIPT", "Description goes here", run_script);

    unsafe {
        call_cpp();
    }

    true
}

#[no_mangle]
extern "C" fn call_rust() {
    println!("Called rust from cpp");
}

pub fn run_plugin(bv: &BinaryView, _addr: u64) {
    let gil = Python::acquire_gil();
    run::run(Project::new(bv, gil.python()));
}

pub fn run_script(bv: &BinaryView, _addr: u64) {
    let gil = Python::acquire_gil();

    let lib = libloading::Library::new("/home/oem/github/ninja-analysis-framework/examples/basic/target/debug/libbasic.so").expect("Couldn't load library");
    
    unsafe {
        let func: libloading::Symbol<unsafe extern fn(proj: Project)> = lib.get(b"main").expect("Couldn't find library function");
        func(Project::new(bv, gil.python()));
    }


    //run::run(Project::new(bv, gil.python()));
}

pub fn run_ui(bv: &BinaryView, _addr: u64) {
    unsafe {
        call_ui();
    }
}
