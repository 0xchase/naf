use cpython::{Python, PyDict, PyResult};

pub struct DebuggerUI<'p> {
    pub py: Python<'p>,
}

impl<'p> DebuggerUI<'p> {
    pub fn new(py: cpython::Python<'p>) -> DebuggerUI<'p> {
        return DebuggerUI {
            py: py,
        }
    }

    pub fn init(&mut self) {
        self.py.run("import Vector35_debugger.ui as ui", None, None).expect("Couldn't import debugger");
        //self.py.run("dbg = ui.DebugAdapterGdb()", None, None).expect("Couldn't import debugger");
        self.py.run("ui.cb_process_run(\"/home/oem/github/ninja-analysis-framework/binaries/lockpicksim\")", None, None).expect("Couldn't start debugger");
        info!("Initialized debugger");
    }

    pub fn breakpoint(&mut self, addr: u64) {
        self.py.run(&format!("dbg.breakpoint_set(0x{:x})", addr), None, None).expect("Couldn't set breakpoint");
        info!("Set breakpoint at 0x{:x}", addr);
    }

    pub fn go(&self) {
        self.py.run("dbg.go()", None, None).expect("Couldn't continue debugger");
        info!("Continuing debugger");
    }

    pub fn step_into(&self) {
        self.py.run("dbg.step_into()", None, None).expect("Couldn't step debugger");
        info!("Stepped debugger");
    }

    pub fn step_over(&self) {
        self.py.run("dbg.step_over()", None, None).expect("Couldn't step debugger");
        info!("Stepped debugger");
    }

    pub fn quit(&self) {
        self.py.run("dbg.quit()", None, None).expect("Couldn't quit debugger");
        info!("Quit debugger");
    }

    pub fn ip(&self) -> u64 {
        match self.py.eval("dbg.reg_read(\"rip\")", None, None) {
            Ok(i) => return i.extract(self.py).expect("Failed to get rip"),
            Err(_) => return 0,
        }
    }

    pub fn regs_print(&self) {

        let rax: u64 = self.py.eval("dbg.reg_read(\"rax\")", None, None).expect("Failed to get rax").extract(self.py).expect("Failed to get rax");
        let rbx: u64 = self.py.eval("dbg.reg_read(\"rbx\")", None, None).expect("Failed to get rbx").extract(self.py).expect("Failed to get rbx");
        let rcx: u64 = self.py.eval("dbg.reg_read(\"rcx\")", None, None).expect("Failed to get rcx").extract(self.py).expect("Failed to get rcx");
        let rdx: u64 = self.py.eval("dbg.reg_read(\"rdx\")", None, None).expect("Failed to get rdx").extract(self.py).expect("Failed to get rdx");
        let rsi: u64 = self.py.eval("dbg.reg_read(\"rsi\")", None, None).expect("Failed to get rsi").extract(self.py).expect("Failed to get rsi");
        let rdi: u64 = self.py.eval("dbg.reg_read(\"rdi\")", None, None).expect("Failed to get rdi").extract(self.py).expect("Failed to get rdi");
        let rip: u64 = self.py.eval("dbg.reg_read(\"rip\")", None, None).expect("Failed to get rip").extract(self.py).expect("Failed to get rip");
        let rbp: u64 = self.py.eval("dbg.reg_read(\"rbp\")", None, None).expect("Failed to get rbp").extract(self.py).expect("Failed to get rbp");
        let rsp: u64 = self.py.eval("dbg.reg_read(\"rsp\")", None, None).expect("Failed to get rsp").extract(self.py).expect("Failed to get rsp");

        info!("rax: 0x{:012x}\trbx: 0x{:012x}\trcx: 0x{:012x}", rax, rbx, rcx);
        info!("rdx: 0x{:012x}\trsi: 0x{:012x}\trdi: 0x{:012x}", rdx, rsi, rdi);
        info!("rip: 0x{:012x}\trbp: 0x{:012x}\trsp: 0x{:012x}", rip, rbp, rsp);
    }

    pub fn reg_read(&self, reg: &str) -> u64 {
        match self.py.eval(&format!("dbg.reg_read(\"{}\")", reg), None, None) {
            Ok(i) => return i.extract(self.py).expect("Failed to get rip"),
            Err(_) => return 0,
        }
    }

}
