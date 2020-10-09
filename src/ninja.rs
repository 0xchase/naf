use binja::binaryview::{BinaryView, BinaryViewExt};
use expression;

pub struct Program<'a> {
    bv: &'a BinaryView,
}

impl<'a> Program<'a> {
    pub fn new(bv: &BinaryView) -> Program {
        return Program {
            bv: bv,
        }
    }
    pub fn functions(&self) -> Vec<Function> {
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

    pub fn function_at(&self, addr: u64) -> Result<Function, String> {
        let functions = self.bv.functions_at(addr);
        for function in &functions {
            return Ok(Function {
                bv: self.bv,
                name: String::from(function.symbol().name().to_ascii_lowercase()),
                addr: function.start()
            })
        }
        return Err(String::from("Function not found"));
    }

    pub fn block_at(&self, addr: u64) -> Result<Block, String> {
        for block in self.bv.basic_blocks_containing(addr).into_iter() {
            return Ok(Block {
                bv: self.bv,
                addr: block.raw_start()
            })
        }
        return Err(String::from("Couldn't find block at address"));
    }

    pub fn inst_after(&self, addr: u64) -> Result<Inst, String> {
        if let Ok(block) = self.block_at(addr) {
            let mut found: bool = false;
            for inst in block.llil() {
                if found {
                    return Ok(inst);
                }
                if inst.addr == addr {
                    found = true;
                }
            }
        }

        return Err(String::from("Couldn't find instruction"));
    }

    pub fn inst_at(&self, addr: u64) -> Result<Inst, String> {
        if let Ok(block) = self.block_at(addr) {
            for inst in block.llil() {
                if inst.addr == addr {
                    return Ok(inst);
                }
            }
        }

        return Err(String::from("Couldn't find instruction"));
    }

    pub fn name(&self) -> String {
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
    pub fn blocks(&self) -> Vec<Block> {
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
            }
        }
        return vec;
    }

    pub fn llil_start(&self) -> u64 {
        for block in self.blocks() {
            for inst in block.llil() {
                return inst.addr;
            }
        }
        return 0;
    }
}

pub struct Block<'a> {
    pub bv: &'a BinaryView,
    pub addr: u64,
    //disassembly: Vec<String>,
}

use binja::llil::InstrInfo::*;

impl<'a> Block<'a> {
    pub fn llil(&self) -> Vec<Inst> {
        let mut vec: Vec<Inst> = Vec::with_capacity(0);
        
        for disass_block in self.bv.basic_blocks_containing(self.addr).into_iter() {       
            if let Ok(llil) = disass_block.function().low_level_il() {
                for block in &llil.basic_blocks() {
                    for inst in &*block {
                        match inst.info() {
                            SetReg(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::SetReg(SetReg {addr: 5}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            SetRegSplit(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::SetRegSplit(SetRegSplit {addr: 5}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            SetFlag(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::SetFlag(SetFlag {addr: 5}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            Store(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::Store(Store {addr: 5}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            Push(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::Push(Push {expr: expression::build_expression(&op.operand())}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            Jump(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::Jump(Jump {addr: 5}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            JumpTo(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::JumpTo(JumpTo {addr: 5}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            Call(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::Call(Call {target: expression::build_expression(&op.target())}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            Ret(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::Ret(Ret {addr: 5}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            If(op) => vec.push(
                                Inst {
                                    addr: op.address(),
                                    llil: LlilInst::If(If {addr: 5}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                            _ => vec.push(
                                Inst {
                                    addr: 0,
                                    llil: LlilInst::Undef(Undef {addr: 0}),
                                    disass: String::from("mov eax, eax"),
                                }
                            ),
                        }
                    }
                }
            /*
                for llil_block in llil.basic_blocks().into_iter() {
                    //info!(" > Getting block for llil");

                    //info!(".");

                    for inst in llil_block.iter() {

                        use binja::llil::InstrInfo::*;
                        match inst.info() {
                            Call(op) => {
                                match op.target().info() { 
                                    binja::llil::ExprInfo::ConstPtr(p) => {
                                        vec.push( 
                                            Inst {
                                                addr: op.address(),
                                                llil: LlilInst::Call(self::Call {target: p.value()}),
                                                disass: String::from("call eax")
                                            }
                                        );
                                        //info!("0x{:x} Calling function at 0x{:x}", op.address(), p.value());
                                    },
                                    binja::llil::ExprInfo::Load(l) => {
                                        vec.push( 
                                            Inst {
                                                addr: op.address(),
                                                llil: LlilInst::Call(self::Call {target: 0x40000}),
                                                disass: String::from("call eax")
                                            }
                                        );
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
                                //vec.push(LlilInst::Nop)
                            },
                            NoRet(op) => {
                                //info!("0x{:x} NoRet", op.address())
                                //vec.push(LlilInst::NoRet)
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
                                //vec.push(LlilInst::Ret)
                            },
                            SetFlag(op) => {
                                //info!("0x{:x} SetFlag _ _", op.address())
                            },
                            SetReg(op) => {
                                match op.source_expr().info() {
                                    Const(c) => {
                                        //info!("0x{:x} SetReg {:?} {}", op.address(), op.dest_reg(), c.value());
                                        vec.push( 
                                            Inst {
                                                addr: op.address(),
                                                llil: LlilInst::SetReg(self::SetReg {reg: String::from("const"), value: 5}),
                                                disass: String::from("mov eax, eax"),
                                            }
                                        );
                                    },
                                    Reg(r) => {
                                        //info!("0x{:x} SetReg {:?} {:?}", op.address(), op.dest_reg(), r.source_reg());
                                        vec.push( 
                                            Inst {
                                                addr: op.address(),
                                                llil: LlilInst::SetReg(self::SetReg {reg: String::from("const"), value: 5}),
                                                disass: String::from("mov eax, eax")
                                            }
                                        );
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
                                /*vec.push( 
                                    Inst {
                                        addr: op.address(),
                                        llil: LlilInst::Store(
                                            self::Store {
                                                dest_mem_expr: op.dest_mem_expr(),
                                                source_expr: String::from("hi"),
                                            }
                                        ),
                                        disass: String::from("mov eax, eax"),
                                    }
                                );*/

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
                */
            }
        }
        return vec;
    }
}

pub struct Inst {
    pub addr: u64,
    pub llil: LlilInst,
    pub disass: String,
}

pub enum LlilInst {
    SetReg(SetReg),
    SetRegSplit(SetRegSplit),
    SetFlag(SetFlag),
    Store(Store),
    Push(Push),
    Jump(Jump),
    JumpTo(JumpTo),
    Call(Call),
    Ret(Ret),
    If(If),
    Undef(Undef),
}

pub struct SetReg {
    pub addr: u64,
}

pub struct SetRegSplit {
    pub addr: u64,
}

pub struct SetFlag {
    pub addr: u64,
}

pub struct Store {
    pub addr: u64,
}

pub struct Push {
    pub expr: expression::Expr,
}

pub struct Jump {
    pub addr: u64,
}

pub struct JumpTo {
    pub addr: u64,
}

pub struct Call {
    pub target: expression::Expr,
}

pub struct Ret {
    pub addr: u64,
}

pub struct If {
    pub addr: u64,
}

pub struct Undef {
    pub addr: u64,
}