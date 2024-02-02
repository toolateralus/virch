use crate::memory::Memory;
#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
pub enum Opcode {
    Halt,
    Jump,
	Store,
	Load,
	LoadPointer,
	LoadRegsiter,
	Add,
    Subtract,
    Multiply,
    Divide,
    CompareInteger, 
	NoOperation,
	Push,
	Pop,
	Count,
}

impl Opcode {
    pub fn from(value: u8) -> Result<Opcode, ()> {
        match value {
            v if v <= Opcode::Count as u8 => Ok(unsafe { std::mem::transmute(v) }),
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
    pub fn instruction(&mut self, ins : Opcode) -> &mut Self {
        let ins = ins as u8;
        self.program.push(ins);
		return &mut *self;
    }
    pub fn u8(&mut self, byte : u8) -> &mut Self {
        self.program.push(byte);
		return &mut *self;
    }
    pub fn u32(&mut self, int : u32) -> &mut Self {
        let bytes = u32::to_le_bytes(int);
        for byte in bytes.iter() {
            self.program.push(*byte);
        }
		return &mut *self;
    }
    pub fn i32(&mut self, int : i32) -> &mut Self {
        let bytes = i32::to_le_bytes(int);
        for byte in bytes.iter() {
            self.program.push(*byte);
        }
		return &mut *self;
    }
}