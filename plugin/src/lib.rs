mod binaryninja;

use binaryninja::LlilInstType::*;

fn main() {
    let state = binaryninja::get_initial_state();
    let main = binaryninja::get_llil();

    println!("Analyzing function {}", main.name);
    for block in main.blocks {
        println!("Analyzng block {}", block.address);

        for inst in block.llil {
            match inst.kind {
                Load => {
                    println!("Loading from memory");
                    
                }
                Store => println!("Storing some stuff"),
                _ => println!("Unknown instruction at {}: {}", inst.address, inst.text),
            }
        }

        
    }
}

