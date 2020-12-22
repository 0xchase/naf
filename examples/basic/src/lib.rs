extern crate naf_core;

use naf_core::TestTrait;

#[no_mangle]
fn main(proj: &dyn TestTrait) {
    proj.step();
    println!("Done running plugin");
    
}

