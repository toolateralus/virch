use crate::{opcodes::Instruction};

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
    pub fn build(self, memory : &mut [u8; 4096]) {
        for (i, v) in self.program.iter().enumerate() {
            memory[i] = *v as u8;
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

pub struct CPU {
    pub ip: usize,
    pub registers: [i32; 32],
}

impl CPU {
    pub fn cycle(&mut self, memory: &mut [u8; 4096]) -> bool {
        let ins = Instruction::from(memory[self.ip]);
    
        match ins {
            Ok(instruction) => {
                match instruction {
                    Instruction::Halt => { return false; },
                    Instruction::Jump => { self.ip = memory[self.ip + 1] as usize; },
                    Instruction::Store => {
                        self.ip += 1;
                        let register = self.read_word(memory, self.ip);
                        self.ip += 4;
                        let address = self.read_word(memory, self.ip);
                        self.ip += 4;
                        self.write_word(memory, address, register);
                    },
                    Instruction::Load => {
                        self.ip += 1;
                        let register = self.read_word(memory, self.ip);
                        self.ip += 4;
                        let address = self.read_word(memory, self.ip);
                        self.ip += 4;
                        self.write_register(register, memory, address);
                    },
                    Instruction::Add => self.registers[0] = self.registers[0] + self.registers[1],
                    Instruction::Sub => self.registers[0] = self.registers[0] + self.registers[1],
                    Instruction::Mul => self.registers[0] = self.registers[0] + self.registers[1],
                    Instruction::Div => self.registers[0] = self.registers[0] + self.registers[1],
                    Instruction::Cmpi => self.registers[0] = (self.registers[0] == self.registers[1]) as i32,
                }
            }
            Err(_) => {
                // this is data.
                panic!("{:#?}", memory[self.ip]);
            }
        }
        return true;
    }

    pub fn write_register(&mut self, register: usize, memory: &mut [u8; 4096], address: usize) {
        if register > self.registers.len() {
            panic!("invalid register, out of range. :: {}", register);
        }

        self.registers[register] = self.read_word(memory, address) as i32;
    }

    pub fn write_word(&mut self, memory: &mut [u8; 4096], address: usize, register: usize) {
        if address > memory.len() || address + 4 > memory.len() {
            panic!("memory access out of bounds : {}", address);
        }

        let value = self.registers[register];

        Self::write_word_direct(value, memory, address);
    }

    pub fn read_word(&self, memory: &mut [u8; 4096], address: usize) -> usize {
        if address > memory.len() || address + 4 > memory.len() {
            panic!("memory access out of bounds : {}", address);
        }

        let bytes = &memory[address..address+4];
        let value = u32::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3]
        ]);
        value as usize
    }
    
    pub fn write_word_direct(value: i32, memory: &mut [u8; 4096], address: usize) {
        let bytes = value.to_be_bytes();
        memory[address..address+4].copy_from_slice(&bytes);
    }
}