use cpython::{Python};
use binja::binaryview::{BinaryView};

use ninja::*;
use python::*;
use debugger::*;

pub struct Project<'a, 'p> {
    pub program: Program<'a>,
    pub python: Python3<'p>,
    pub debugger: Debugger<'p>,
    py: cpython::Python<'p>,
}

impl<'a, 'p> Project<'a, 'p> {
    pub fn new(bv: &'a BinaryView, py: cpython::Python<'p>) -> Project<'a, 'p> {
        return Project {
            program: Program::new(bv),
            python: Python3::new(py),
            debugger: Debugger::new(py),
            py: py,
        };
    }
}
