use cpython::{Python};
use binaryninja::binaryview::{BinaryView, BinaryViewExt};

use program::*;
use python::*;
use debugger::*;
use debugger_ui::*;

pub struct Project<'a, 'p> {
    pub program: Program<'a>,
    pub python: Python3<'p>,
    pub debugger: Debugger<'p>,
    pub debugger_ui: DebuggerUI<'p>,
    pub arch: String, 
    py: cpython::Python<'p>,
}

impl<'a, 'p> Project<'a, 'p> {
    pub fn new(bv: &'a BinaryView, py: cpython::Python<'p>) -> Project<'a, 'p> {
        let default_arch = bv.default_arch().unwrap();
        let arch = default_arch.name().as_str().to_string();
        return Project {
            program: Program::new(bv),
            python: Python3::new(py),
            debugger: Debugger::new(py),
            debugger_ui: DebuggerUI::new(py),
            arch: arch, 
            py: py,
        };
    }
}
