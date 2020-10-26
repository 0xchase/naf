use cpython::{Python};

pub struct Python3<'p> {
    py: Python<'p>,
}

impl<'p> Python3<'p> {
    pub fn new(py: cpython::Python<'p>) -> Python3<'p> {
        return Python3 {
            py: py
        };
    }

    pub fn run(&self, code: &str) {
        self.py.run(code, None, None).expect(&format!("Failed to execute: {}", code));
    }
}
