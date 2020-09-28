include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn get_llil() -> Function {
    println!("Running test...");
    let r = unsafe {square(5)};
    println!("Running Rust main: {}\n", r);

    let f = Function {
        name: String::from("main"),
        blocks: vec![
            Block {
                address: 400000,
                llil: vec![
                    LlilInst {address: 400001, text: String::from("load things"), kind: LlilInstType::Load},
                    LlilInst {address: 400002, text: String::from("store things"), kind: LlilInstType::Store},
                    LlilInst {address: 400003, text: String::from("push things"), kind: LlilInstType::Push},
                    LlilInst {address: 400004, text: String::from("pop things"), kind: LlilInstType::Pop},
                    LlilInst {address: 400005, text: String::from("load things"), kind: LlilInstType::Load},
                ],
            },
        ],
    };
    return f;
}

pub fn get_initial_state() -> State {
    return State {
        regs: Regs {
            rax: 1,
            rbx: 2,
            rcx: 3,
            rdx: 4,
            rbp: 5,
            rsp: 6,
            rsi: 7,
            rdi: 8,
        },
        mem: 0,
    }
}

#[no_mangle]
extern "C" fn trigger1() {
    // stuff
    println!("Triggering some plugin action");
}

// ---------- State ---------- //

pub struct State {
    pub regs: Regs,
    pub mem: i32,
}

pub struct Regs {
    pub rax: i32, 
    pub rbx: i32, 
    pub rcx: i32, 
    pub rdx: i32, 
    pub rbp: i32, 
    pub rsp: i32, 
    pub rsi: i32, 
    pub rdi: i32,
}

// ---------------------------------------------------

/*
Function
    Dissasembly
    LLIL
        Blocks
            Instructions
    MLIL
    HLIL
*/

pub struct Function {
    pub name: String,
    pub blocks: Vec<Block>,
    //mlil: Vec<MlilInst>,
    //hlil: Vec<HlilInst>,
}

// ---------- Disassembly ---------- //

// ---------- LLIL ---------- //

pub struct Block {
    pub address: i32,
    pub llil: Vec<LlilInst>,
    //disassembly: Vec<String>,
}

pub enum LlilInstType {
    Load,
    Store,
    Push,
    Pop,
}

pub struct LlilInst {
    pub address: i32,
    pub text: String,
    pub kind: LlilInstType,
}

// ---------- MLIL ---------- //

// ---------- MLIL ---------- //

