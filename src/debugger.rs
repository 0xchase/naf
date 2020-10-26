use cpython::{Python, PyDict, PyResult};

enum DebugState<'p> {
    Alive(cpython::Python<'p>),
    Dead(&'p str),
}

pub struct Debugger<'p> {
    pub py: Python<'p>,
}

impl<'p> Debugger<'p> {
    /*
    pub fn new() -> Debugger<'p> {
        /*
        let binaryninja = py.import("binaryninja");
        let platform = py.import("platform");
        let bn_debugger = py.import("Vector35_debugger.gdb");
        
        let locals = PyDict::new(py);
        match bn_debugger {
            Ok(debugger) => {
                locals.set_item(py, "gdb", debugger);
                py.run("dbg = gdb.DebugAdapterGdb()", None, Some(&locals));
            }
            _ => error!("Unable to import binaryninja debugger"),
        }
        */

        return Debugger {
            name: String::from("/home/oem/github/ninja-analysis-framework/binaries/lockpicksim"),
            gil: Python::acquire_gil(),
            py: None,
        };
    }
    */

    pub fn init(&mut self) {
        self.py.run("import Vector35_debugger.gdb as gdb", None, None).expect("Couldn't import debugger");
        self.py.run("dbg = gdb.DebugAdapterGdb()", None, None).expect("Couldn't import debugger");
        self.py.run("dbg.exec(\"/home/oem/github/ninja-analysis-framework/binaries/lockpicksim\")", None, None).expect("Couldn't start debugger");
        info!("Initialized debugger");
    }

    pub fn breakpoint(&mut self, addr: u64) {
        self.py.run(&format!("dbg.breakpoint_set(0x{:x})", addr), None, None).expect("Couldn't set breakpoint");
        info!("Set breakpoint at 0x{:x}", addr);
    }

    pub fn go(&self) {
        self.py.run("dbg.go()", None, None).expect("Couldn't continue debugger");
        println!("Continuing debugger");
    }

    pub fn step_into(&self) {
        self.py.run("dbg.step_into()", None, None).expect("Couldn't step debugger");
        info!("Stepped debugger");
    }

    pub fn step_over(&self) {
        self.py.run("dbg.step_over()", None, None).expect("Couldn't step debugger");
        info!("Stepped debugger");
    }

}

pub fn test() {
    info!("Testing debugger");
    let gil = Python::acquire_gil();
    hello(gil.python()).unwrap();
}

pub fn hello(py: Python) -> PyResult<()> {
    let sys = py.import("sys")?;
    let binaryninja = py.import("binaryninja")?;
    let platform = py.import("platform")?;
    let bn_debugger = py.import("Vector35_debugger.gdb")?;

    let version: String = sys.get(py, "version")?.extract(py)?;

    let locals = PyDict::new(py);
    locals.set_item(py, "os", py.import("os")?)?;
    locals.set_item(py, "platform", py.import("platform")?)?;
    locals.set_item(py, "gdb", py.import("Vector35_debugger.gdb")?)?;
    locals.set_item(py, "binaryninja", py.import("binaryninja")?)?;

    let temp1 = py.run("dbg = gdb.DebugAdapterGdb()", None, Some(&locals));
    let temp2 = py.run("dbg.exec(\"/home/oem/github/ninja-analysis-framework/binaries/lockpicksim\", [])", None, Some(&locals))?;
    let temp3 = py.run("dbg.breakpoint_set(0x4006f6)", None, Some(&locals))?;
    
    //locals.set_item(py, "debugger", py.import("debugger")?)?;
    
    let user: String = py.eval("os.getenv('USER') or os.getenv('USERNAME')", None, Some(&locals))?.extract(py)?;
    let platform_string: String = py.eval("platform.system()", None, Some(&locals))?.extract(py)?;
    println!("Hello {}, I'm Python {}, on system {}", user, version, platform_string);
    Ok(())
}

