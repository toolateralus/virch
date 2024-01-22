use crate::memory::Memory;


#[allow(dead_code)]
#[repr(u8)]
pub enum Instruction {
    Halt,
    Jump,
    Store,
    Load,
    Add,
    Sub,
    Mul,
    Div,
    Cmpi, // compare integer.
}

impl Instruction {
    pub fn from(value: u8) -> Result<Instruction, ()> {
        match value {
            0 => Ok(Instruction::Halt),
            1 => Ok(Instruction::Jump),
            2 => Ok(Instruction::Store),
            3 => Ok(Instruction::Load),
            4 => Ok(Instruction::Add),
            5 => Ok(Instruction::Sub),
            6 => Ok(Instruction::Mul),
            7 => Ok(Instruction::Div),
            8 => Ok(Instruction::Cmpi),
            _ => Err(()),
        }
    }
}


pub type Program = Vec<u8>;
pub struct ProgramBuilder {
    program: Program,
}
impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
        }
    }
    pub fn build(self, memory : &mut Memory) {
        for (i, v) in self.program.iter().enumerate() {
            memory.write_u8(i, *v as u8);
        }
    }
    pub fn instruction(&mut self, ins : Instruction) {
        let ins = ins as u8;
        self.program.push(ins);
    }
    pub fn u8(&mut self, byte : u8) {
        self.program.push(byte);
    }
    pub fn u32(&mut self, int : u32) {
        let bytes = u32::to_le_bytes(int);
        for byte in bytes.iter() {
            self.program.push(*byte);
        }
    }
    pub fn i32(&mut self, int : i32) {
        let bytes = i32::to_le_bytes(int);
        for byte in bytes.iter() {
            self.program.push(*byte);
        }
    }
}