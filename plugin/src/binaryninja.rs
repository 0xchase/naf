include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn test_me() {
    println!("Running test...");
    let r = unsafe {square(5)};
    println!("Running Rust main: {}", r);
}
