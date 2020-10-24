use cpython::{Python, PyDict, PyResult};

pub fn test() {
    info!("Testing debugger");
    let gil = Python::acquire_gil();
    hello(gil.python()).unwrap();
}

pub fn hello(py: Python) -> PyResult<()> {
    let sys = py.import("sys")?;
    let binaryninja = py.import("binaryninja")?;
    let platform = py.import("platform")?;
    //let bn_debugger = py.import("debugger")?;

    let version: String = sys.get(py, "version")?.extract(py)?;

    let locals = PyDict::new(py);
    locals.set_item(py, "os", py.import("os")?)?;
    locals.set_item(py, "platform", py.import("platform")?)?;

    locals.set_item(py, "binaryninja", py.import("binaryninja")?)?;
    //locals.set_item(py, "debugger", py.import("debugger")?)?;
    

    let user: String = py.eval("os.getenv('USER') or os.getenv('USERNAME')", None, Some(&locals))?.extract(py)?;
    let platform_string: String = py.eval("platform.system()", None, Some(&locals))?.extract(py)?;
    //let import: String = py.eval("import binaryninja", None, Some(&locals))?.extract(py)?;

    println!("Hello {}, I'm Python {}, on system {}", user, version, platform_string);
    Ok(())
}

