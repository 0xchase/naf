//use binja::binaryview::{BinaryView, BinaryViewExt, BinaryViewType, BinaryViewTypeExt};
use binja::binaryview::{BinaryView, BinaryViewExt};
use binja::string::BnString;

pub fn test(program: Program, addr: u64) {
    info!("Analyzing program {} from 0x{:x}", program.name(), addr);
    for function in program.functions() {
        info!(" > Analyzing function {} at 0x{:x}", function.name, function.addr);
        if function.name.eq("sub_cca0") {
            for block in function.blocks() {
                info!("  >> Analyzing block at 0x{:x}", block.addr);
                for inst in block.llil() {
                    info!("  >>> Instruction");
                }
            }
        }
    }
}

// ------------------------------------------------

pub struct Program<'a> {
    pub bv: &'a BinaryView,
}
impl<'a> Program<'a> {
    fn functions(&self) -> Vec<Function> {
        let mut vec: Vec<Function> = Vec::with_capacity(0);
        for function in &self.bv.functions() {
            vec.push( 
                Function {
                    bv: self.bv,
                    name: String::from(function.symbol().name().to_ascii_lowercase()),
                    addr: function.start(),
                }
            )
        }
        return vec;
    }

    fn name(&self) -> String {
        return String::from("/bin/ls");
    }
}

pub struct Function<'a> {
    pub bv: &'a BinaryView,
    pub name: String,
    pub addr: u64,
    //mlil: Vec<MlilInst>,
    //hlil: Vec<HlilInst>,
}

impl<'a> Function<'a> {
    fn blocks(&self) -> Vec<Block> {
        let mut vec: Vec<Block> = Vec::with_capacity(0);

        for function in &self.bv.functions() {
            if function.start() == self.addr {
                for block in function.basic_blocks().into_iter() {
                    vec.push(
                        Block {
                            bv: self.bv,
                            addr: block.raw_start()
                        }
                    )
                }
                /*
                if let Ok(llil) = function.low_level_il() {
                    for block in llil.basic_blocks().into_iter() {
                        vec.push(
                            Block {
                                bv: self.bv,
                                addr: block.raw_start()
                            }
                        )
                    }
                }
                */
            }
        }

        return vec;
    }
}

pub struct Block<'a> {
    pub bv: &'a BinaryView,
    pub addr: u64,
    //disassembly: Vec<String>,
}

impl<'a> Block<'a> {
    fn llil(&self) -> Vec<LlilInst> {
        //info!("Checking llil");
        let mut vec: Vec<LlilInst> = Vec::with_capacity(0);
        
        for block in self.bv.basic_blocks_containing(self.addr).into_iter() {
            for inst in block.iter() {
                //info!("Instruction");
            }

            if let Ok(llil) = block.function().low_level_il() {
                for block in llil.basic_blocks().into_iter() {
                    info!("\nChecking block number {}", block.raw_start());
                    for temp in block.iter() {
                        //info!("Instruction");

                        use binja::llil::InstrInfo::*;
                        match temp.info() {
                            Call(op) => {
                                match op.target().info() { 
                                    binja::llil::ExprInfo::ConstPtr(p) => {
                                        info!("0x{:x} Calling pointer type", op.address())
                                    },
                                    binja::llil::ExprInfo::Const(c) => {
                                        info!("0x{:x} Calling constant type", op.address())
                                    },
                                    _ => error!("0x{:x} Calling ????????", op.address()),
                                }
                            },
                            Goto(op) => info!("0x{:x} Goto _", op.address()),
                            If(op) => info!("0x{:x} If _", op.address()),
                            Jump(op) => info!("0x{:x} Jump _", op.address()),
                            JumpTo(op) => info!("0x{:x} Jump to _", op.address()),
                            Nop(op) => info!("0x{:x} Nop", op.address()),
                            NoRet(op) => info!("0x{:x} NoRet", op.address()),
                            Push(op) => info!("0x{:x} Push _", op.address()),
                            Ret(op) => info!("0x{:x} Ret", op.address()),
                            SetFlag(op) => info!("0x{:x} SetFlag _ _", op.address()),
                            SetReg(op) => info!("0x{:x} SetReg _ _", op.address()),
                            SetRegSplit(op) => info!("0x{:x} SetRegSplit _ _", op.address()),
                            Store(op) => info!("0x{:x} Store _ in _", op.address()),
                            Syscall(op) => info!("0x{:x} Syscall _", op.address()),
                            Trap(op) => info!("0x{:x} Trap _", op.address()),
                            Undef(op) => error!("Undef"),
                            Value(a, b) => info!("Value a b"),
                            _ => error!("Other")
                        }
                    }
                }
            }
        }

        /*
        for function in &self.bv.functions() {
            info!("Checking function");
            if function.start() == self.addr {
                info!("Found function");
                if let Ok(llil) = function.low_level_il() {
                    for block in llil.basic_blocks().into_iter() {
                        if self.addr == block.raw_start() {
                            for addr in block.iter() {
                                info!("  >> Instruction at here");
                            }
                        }
                    }
                }
            }
        }
        */


        return vec;
    }
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
