use crate::opcodes::Instruction;

pub type Program = Vec<u8>;
pub struct ProgramBuilder {
    program: Program,
}
pub const MEM_SIZE : usize = 4096;
pub const NUM_REGISTERS : usize = 8;

pub struct Memory {
    pub memory: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0; MEM_SIZE],
        }
    }
    pub fn write_i32(&mut self, addr: usize, value: i32) {
        let bytes = value.to_le_bytes();
        self.memory[addr..addr+4].copy_from_slice(&bytes);
    }
    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.memory[addr] = value;
    }
    pub fn read_i32(&mut self, addr: usize) -> i32 {
        let bytes = &self.memory[addr..addr+4];
        i32::from_le_bytes([bytes[0],bytes[1],bytes[2],bytes[3]])
    }
    pub fn read_u8(&mut self, addr: usize) -> u8 {
        self.memory[addr]
    }
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

pub struct CPU {
    pub ip: usize,
    pub registers: [i32; NUM_REGISTERS],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            ip:0, // we ignore the first 4 bytes : it's the exit/error code. 
            registers:[0; NUM_REGISTERS]
        }
    }
    pub fn consume<T>(&mut self) {
        let size = std::mem::size_of::<T>();
        self.ip += size;
    }
    pub fn cycle(&mut self, memory: &mut Memory) -> bool {
        let ins = memory.read_u8(self.ip);
        let ins = Instruction::from(ins);
    
        match ins {
            Ok(instruction) => {
                match instruction {
                    Instruction::Halt => { return false; },
                    Instruction::Jump => { self.ip = memory.read_u8(self.ip + 1) as usize; },
                    Instruction::Store => {
                        self.consume::<u8>();
                        let register = memory.read_i32(self.ip) as usize;
                        self.consume::<i32>();
                        let address = memory.read_i32(self.ip) as i32;
                        self.consume::<i32>();
                        memory.write_i32(register, address);
                    },
                    Instruction::Load => {
                        self.consume::<u8>();
                        let register = memory.read_i32(self.ip) as usize;
                        self.consume::<i32>();
                        let address = memory.read_i32(self.ip) as i32;
                        self.consume::<i32>();
                        self.write_register(register, address);
                    },
                    Instruction::Add => {
                        self.registers[0] = self.registers[0] + self.registers[1];
                        self.consume::<u8>();
                    },
                    Instruction::Sub => {
                        self.registers[0] = self.registers[0] - self.registers[1];
                        self.consume::<u8>();
                    },
                    Instruction::Mul => {
                        self.registers[0] = self.registers[0] * self.registers[1];
                        self.consume::<u8>();
                    },
                    Instruction::Div => {
                        self.registers[0] = self.registers[0] / self.registers[1];
                        self.consume::<u8>();
                    },
                    Instruction::Cmpi => {
                        self.registers[0] = (self.registers[0] == self.registers[1]) as i32;
                        self.consume::<u8>();
                    },
                }
            }
            Err(_) => {
                // this is data.
                panic!("{:#?}", memory.read_i32(self.ip));
            }
        }
        return true;
    }

    pub fn write_register(&mut self, register: usize, value: i32) {
        if register > self.registers.len() {
            panic!("register write out of bounds.. : register {} does not exist.", register);
        }
        self.registers[register] = value;
    }
}