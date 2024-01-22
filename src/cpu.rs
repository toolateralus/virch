use crate::{opcodes::Instruction, memory::Memory};
pub const NUM_REGISTERS : usize = 8;
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
                        let address = memory.read_i32(self.ip) as usize;
                        self.consume::<i32>();
                        memory.write_i32(address, self.registers[register]);
                    },
                    Instruction::Load => {
                        self.consume::<u8>();
                        let register = memory.read_i32(self.ip) as usize;
                        self.consume::<i32>();
                        let value = memory.read_i32(self.ip);
                        self.consume::<i32>();
                        self.write_register(register, value);
                    },
                    Instruction::Add => {
                        self.consume::<u8>();
                        self.registers[0] = self.registers[0] + self.registers[1];
                    },
                    Instruction::Sub => {
                        self.consume::<u8>();
                        self.registers[0] = self.registers[0] - self.registers[1];
                    },
                    Instruction::Mul => {
                        self.consume::<u8>();
                        self.registers[0] = self.registers[0] * self.registers[1];
                    },
                    Instruction::Div => {
                        self.consume::<u8>();
                        self.registers[0] = self.registers[0] / self.registers[1];
                        self.registers[2] = self.registers[0] % self.registers[1];
                    },
                    Instruction::Cmpi => {
                        self.consume::<u8>();
                        self.registers[0] = (self.registers[0] == self.registers[1]) as i32;
                    },
                }
            }
            Err(_) => {
                panic!("unexpected data in program : {:#?} at {}", memory.read_i32(self.ip), self.ip);
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

    pub fn run(&mut self, memory: &mut Memory) {
        loop {
            if !self.cycle(memory) {
                break;
            }
        }
    }
}