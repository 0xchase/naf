use binaryninja::binaryview::{BinaryView, BinaryViewExt};
use binaryninja::symbol::SymbolType;
use expression;

pub struct Program<'a> {
    bv: &'a BinaryView,
}

impl<'a> Program<'a> {
    pub fn new(bv: &BinaryView) -> Program {
        bv.functions();
        return Program {
            bv: bv,
        }
    }

    pub fn seek(&self, addr: u64) {
        if let Err(_) = self.bv.metadata().navigate_to(self.bv.metadata().current_view(), addr) {
            error!("Failed to seek");
        }
    }

    pub fn strings(&self) {
        
    }

    pub fn offset(&self) -> u64 {
        return self.bv.metadata().current_offset();
    }

    pub fn symbols(&self) {
        for symbol in self.bv.symbols().into_iter() {
            info!("At symbol {} {} {}", symbol.address(), symbol.short_name(), symbol.full_name());
            match symbol.sym_type() {
                SymbolType::Function => info!(" > function"),
                SymbolType::LibraryFunction => info!(" > libraryfunction"),
                SymbolType::ImportAddress => info!(" > import address"),
                SymbolType::ImportedFunction => info!(" > imported function"),
                SymbolType::Data => info!(" > data"),
                SymbolType::ImportedData => info!(" > imported data"),
                SymbolType::External => info!(" > external"),
            }
        }
    }

    pub fn string_at_addr(&self, addr: u64) {
        //self.bv.string_at_addr(addr);
        //self.bv.offset_
        //self.bv.string_at_addr() as u64;
    }

    pub fn symbols2(&self) {
        for symbol in self.bv.symbols().into_iter() {
            info!("At symbol {} {} {}", symbol.address(), symbol.short_name(), symbol.full_name());
            match symbol.sym_type() {
                SymbolType::Data => info!("Found data: {}", symbol.full_name()),
                _ => (),
            }
        }
    }

    pub fn functions(&self) -> Vec<Function> {
        let mut vec: Vec<Function> = Vec::with_capacity(0);
        for function in &self.bv.functions() {
            vec.push(
                Function {
                    bv: self.bv,
                    name: String::from(function.symbol().full_name().to_ascii_lowercase()),
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
                name: String::from(function.symbol().full_name().to_ascii_lowercase()),
                addr: function.start(),
            })
        }
        return Err(String::from("Function not found"));
    }

    pub fn function_containing(&self, addr: u64) -> Result<Function, String> {
        for block in self.bv.basic_blocks_containing(addr).into_iter() {
            let function = block.function();
            return Ok(Function {
                bv: self.bv,
                name: String::from(function.symbol().full_name().to_ascii_lowercase()),
                addr: function.start()
            })
        }

        return Err(String::from("Couldn't find function"));
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
    pub fn num_blocks(&self, addr: u64) -> usize {
        let blocks = self.bv.basic_blocks_containing(addr);
        return blocks.len();
    }

    // Gets the next instruction
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

    /** Retrieves the instructions at the address */
    pub fn insts_at_addr(&self, addr: u64) -> Result<Vec<Index>, String> { 
        let mut curr_index: u64 = 0; 
        let mut instructions = Vec::new();
        if let Ok(block) = self.block_at(addr) {
            for inst in block.llil() {
                if inst.addr == addr {

                    let return_index = Index {
                        inst: inst,
                        index: curr_index,
                    };

                    instructions.push(return_index);
                }
                curr_index = curr_index + 1; 
            }
            return Ok(instructions);
        }
        return Err(String::from("Could not any instructions at address"));
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

    pub fn length(&self) -> u64 {
        let mut count: u64 = 0;
        for block in self.blocks() {
            for inst in block.llil() {
                count += 1;
            }
        }

        return count;
    }

    pub fn llil_at_index(&self, index: usize) -> Result<Inst, String> {
        for function in &self.bv.functions() {
            if function.start() == self.addr {
                if let Ok(llil) = function.low_level_il() {
                    return Ok(build_inst(llil.instruction_from_idx(index)));
                }
            }
        }
        return Err(String::from("Instruction index is out of range"));
    }
}

pub struct Block<'a> {
    pub bv: &'a BinaryView,
    pub addr: u64,
    //disassembly: Vec<String>,
}

use binaryninja::llil::InstrInfo::*;

impl<'a> Block<'a> {
    pub fn llil(&self) -> Vec<Inst> {
        let mut vec: Vec<Inst> = Vec::with_capacity(0);
        
        let mut index: u64 = 0;

        for disass_block in self.bv.basic_blocks_containing(self.addr).into_iter() {       
            if let Ok(llil) = disass_block.function().low_level_il() {
                for block in &llil.basic_blocks() {
                    for inst in &*block {
                        vec.push(build_inst(inst));
                        index += 1;
                    }
                }

            }
        }
        return vec;
    }
}

// Rust bindings for diassemblly not working 
pub fn build_inst(inst: binaryninja::llil::Instruction<binaryninja::architecture::CoreArchitecture, binaryninja::llil::Finalized, binaryninja::llil::NonSSA<binaryninja::llil::RegularNonSSA>>) -> Inst {
    match inst.info() {
        SetReg(op) => {
            Inst {
                addr: op.address(),
                llil: LlilInst::SetReg(SetReg {reg: format!("{:?}", op.dest_reg()), expr: expression::build_expression(&op.source_expr())}),
                disass: String::from("mov eax, eax"),
            }
        },
        SetRegSplit(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::SetRegSplit(SetRegSplit {dest_reg_high: format!("{:?}", op.dest_reg_high()), dest_reg_low: format!("{:?}", op.dest_reg_low()), source_expr: expression::build_expression(&op.source_expr())}),
                disass: String::from("mov eax, eax"),
            },
        SetFlag(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::SetFlag(SetFlag {addr: 5}),
                disass: String::from("mov eax, eax"),
            },
        Store(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Store(Store {source_expr: expression::build_expression(&op.source_expr()), dest_mem_expr: expression::build_expression(&op.dest_mem_expr())}),
                disass: String::from("mov eax, eax"),
            },
        Push(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Push(Push {expr: expression::build_expression(&op.operand())}),
                disass: String::from("mov eax, eax"),
            },
        Jump(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Jump(Jump {addr: 5}),
                disass: String::from("mov eax, eax"),
            },
        JumpTo(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::JumpTo(JumpTo {addr: 5}),
                disass: String::from("mov eax, eax"),
            },
        Call(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Call(Call {target: expression::build_expression(&op.target())}),
                disass: String::from("mov eax, eax"),
            },
        Ret(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Ret(Ret {addr: 5}),
                disass: String::from("mov eax, eax"),
            },
        If(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::If(If {condition: expression::build_expression(&op.condition()), target_true: build_inst(op.true_target()).addr, target_false: build_inst(op.false_target()).addr}),
                disass: String::from("mov eax, eax"),
            },
        Nop(op) => 
            Inst {
                addr: op.address(),
                llil: LlilInst::Nop(),
                disass: String::from("mov eax, eax"),
            },
        NoRet(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::NoRet(),
                disass: String::from("mov eax, eax"),
            },
        Goto(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Goto(Goto {target: 115}),
                disass: String::from("mov eax, eax"),
            },
        Syscall(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Syscall(),
                disass: String::from("mov eax, eax"),
            },
        Bp(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Bp(),
                disass: String::from("mov eax, eax"),
            },
        Trap(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Trap(),
                disass: String::from("mov eax, eax"),
            },
        Undef(op) =>
            Inst {
                addr: op.address(),
                llil: LlilInst::Undef(),
                disass: String::from("mov eax, eax"),
            },
        _ =>
            Inst {
                addr: 0,
                llil: LlilInst::Undef(),
                disass: String::from("mov eax, eax"),
            },
    }
}

pub struct Inst {
    pub addr: u64,
    pub llil: LlilInst,
    pub disass: String,
}

pub struct Index {
    pub inst: Inst, 
    pub index: u64
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
    Nop(),
    NoRet(),
    Goto(Goto),
    Syscall(),
    Bp(),
    Trap(),
    Undef(),
}

pub struct SetReg {
    pub expr: expression::Expr,
    pub reg: String,
}

pub struct SetRegSplit {
    pub dest_reg_high: String,
    pub dest_reg_low: String,
    pub source_expr: expression::Expr,
}

pub struct SetFlag {
    pub addr: u64,
}

pub struct Store {
    pub dest_mem_expr: expression::Expr,
    pub source_expr: expression::Expr,
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
    pub condition: expression::Expr,
    pub target_true: u64,
    pub target_false: u64,
}

pub struct Goto {
    pub target: u64,
}

pub struct Return {
    pub target: u64,
}
