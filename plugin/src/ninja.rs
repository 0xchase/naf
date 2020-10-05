//use binja::binaryview::{BinaryView, BinaryViewExt, BinaryViewType, BinaryViewTypeExt};
use binja::binaryview::{BinaryView, BinaryViewExt};
use binja::string::BnString;

pub fn test(program: Program, addr: u64) {
    //info!("Analyzing program {} from 0x{:x}", program.name(), addr);
    for function in program.functions() {
        //info!(" > Analyzing function {} at 0x{:x}", function.name, function.addr);
        if function.name.eq("_start") {
            for block in function.blocks() {
                //info!("  >> Analyzing block at 0x{:x}", block.addr);
                for inst in block.llil() {
                    match inst {
                        LlilInst::Call(c) => {
                            info!("0x{:x} Calling 0x{:x}", c.address, c.target);
                        }
                        _ => {
                            info!("Instruction");
                        }
                    }
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
        
        for disass_block in self.bv.basic_blocks_containing(self.addr).into_iter() {
            for inst in disass_block.iter() {
                //info!("Instruction");
            }
            use llil::ExprInfo::*;

            
            if let Ok(llil) = disass_block.function().low_level_il() {
                for llil_block in llil.basic_blocks().into_iter() {

                    //info!(".");
                    for inst in llil_block.iter() {

                        use binja::llil::InstrInfo::*;
                        match inst.info() {
                            Call(op) => {
                                match op.target().info() { 
                                    binja::llil::ExprInfo::ConstPtr(p) => {
                                        //info!("0x{:x} Calling function at 0x{:x}", op.address(), p.value());
                                        vec.push( LlilInst::Call(self::Call {address: op.address(), target: p.value()}) );
                                    },
                                    _ => error!("0x{:x} Calling ????????", op.address()),
                                }
                            },
                            Goto(op) => {
                                //info!("0x{:x} Goto _", op.address())
                            },
                            If(op) => {
                                //info!("0x{:x} If _", op.address())
                            },
                            Jump(op) => {
                                //info!("0x{:x} Jump _", op.address())
                            },
                            JumpTo(op) => {
                                //info!("0x{:x} Jump to _", op.address())
                            },
                            Nop(op) => {
                                //info!("0x{:x} Nop", op.address())
                                vec.push(LlilInst::Nop)
                            },
                            NoRet(op) => {
                                //info!("0x{:x} NoRet", op.address())
                                vec.push(LlilInst::NoRet)
                            },
                            Push(op) => {
                                match op.operand().info() {
                                    binja::llil::ExprInfo::Reg(r) => {
                                        //info!("0x{:x} Push reg {:?}", op.address(), r.source_reg());
                                    },
                                    binja::llil::ExprInfo::Const(c) => {
                                        //info!("0x{:x} Push cons 0x{:x}", op.address(), c.value());
                                    },
                                    _ => {
                                        //info!("0x{:x} Push ???", op.address());
                                    }
                                }
                            },
                            Ret(op) => {
                                //info!("0x{:x} Ret", op.address())
                                vec.push(LlilInst::Ret)
                            },
                            SetFlag(op) => {
                                //info!("0x{:x} SetFlag _ _", op.address())
                            },
                            SetReg(op) => {
                                match op.source_expr().info() {
                                    Const(c) => {
                                        //info!("0x{:x} SetReg {:?} {}", op.address(), op.dest_reg(), c.value());
                                        vec.push( LlilInst::SetReg(self::SetReg {reg: String::from("const"), value: 5}) );
                                    },
                                    Reg(r) => {
                                        //info!("0x{:x} SetReg {:?} {:?}", op.address(), op.dest_reg(), r.source_reg());
                                        vec.push( LlilInst::SetReg(self::SetReg {reg: String::from("eax"), value: 5}) );
                                    },
                                    _ => {
                                        //info!("0x{:x} SetReg {:?} {:?}", op.address(), op.dest_reg(), op.source_expr());
                                    },
                                }
                            },
                            SetRegSplit(op) => {
                                //info!("0x{:x} SetRegSplit _ _", op.address())
                            },
                            Store(op) => {
                                //info!("0x{:x} Store _ in _", op.address())
                            },
                            Syscall(op) => {
                                //info!("0x{:x} Syscall _", op.address())
                            },
                            Trap(op) => {
                                //info!("0x{:x} Trap _", op.address())
                            },
                            Undef(op) => {
                                //error!("Undef")
                            },
                            Value(a, b) => {
                                //info!("Value a b")
                            },
                            _ => {
                                error!("Other")
                            }
                        }
                    /*
                        inst.visit_tree(&mut |e, info| {
                            //info!("visiting {:?}", e);
                            
                            match info {
                                Add(ref op) => {
                                    if let (Reg(ref r), Const(ref c)) = (op.left().info(), op.right().info()) {
                                        info!("    ADD (reg {:?}, const {:x})", r.source_reg(), c.value());
                                    } else {
                                        info!("    ADD(???)");
                                    }
                                    return VisitorAction::Halt;
                                },
                                Sub(ref op) => {
                                    return VisitorAction::Halt;
                                },
                                Const(ref op) => {
                                    info!("    Const 0x{:x}", op.value());
                                    return VisitorAction::Halt;
                                },
                                Reg(ref op) => {
                                    info!("    Reg {:?}", op.source_reg());
                                    return VisitorAction::Halt;
                                }
                                _ => {
                                    info!("    OTHER({:?})", e);
                                }
                            }

                            return VisitorAction::Descend;
                        });
                    */
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

pub enum LlilInst {
    Call(Call),
    SetReg(SetReg),
    Ret,
    NoRet,
    Nop,
}

pub struct Call {
    address: u64,
    target: u64,
}

pub struct SetReg {
    reg: String,
    value: u64,
}