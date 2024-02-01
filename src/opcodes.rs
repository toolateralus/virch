use crate::memory::Memory;
#[allow(dead_code)]
#[repr(u8)]
pub enum Opcode {
    Halt,
    Jump,
    Store,
    Load,
    Add,
    Sub,
    Mul,
    Div,
    Cmpi, 
	Nop,
}

impl Opcode {
    pub fn from(value: u8) -> Result<Opcode, ()> {
        match value {
            0 => Ok(Opcode::Halt),
            1 => Ok(Opcode::Jump),
            2 => Ok(Opcode::Store),
            3 => Ok(Opcode::Load),
            4 => Ok(Opcode::Add),
            5 => Ok(Opcode::Sub),
            6 => Ok(Opcode::Mul),
            7 => Ok(Opcode::Div),
            8 => Ok(Opcode::Cmpi),
            9 => Ok(Opcode::Nop),
            _ => Err(()),
        }
    }
}

pub type Program = Vec<u8>;
pub struct ProgramBuilder { program: Program }

#[allow(dead_code)]
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
    pub fn instruction(&mut self, ins : Opcode) {
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