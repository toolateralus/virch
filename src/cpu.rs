use crate::{opcodes::Instruction, memory::Memory, register::*};
pub const NUM_REGISTERS : usize = 13;

pub struct CPU {
    pub ip: usize,
    pub registers: [i32; NUM_REGISTERS],
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            ip:0,
            registers:[0; NUM_REGISTERS],
        }
    }
    pub fn consume<T>(&mut self) {
        let size = std::mem::size_of::<T>();
        self.ip += size;
        self.write_register(IP, self.ip as i32);
        
    }
    pub fn cycle(&mut self, memory: &mut Memory) -> bool {
        let ins = memory.read_u8(self.ip);
        let ins = Instruction::from(ins);
    
        match ins {
            Ok(instruction) => {
                match instruction {
                    Instruction::Halt => { return false; },
                    Instruction::Jump => { self.ip = memory.read_i32(self.ip + 1) as usize; },
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
                        self.registers[A] = self.registers[A] + self.registers[B];
                    },
                    Instruction::Sub => {
                        self.consume::<u8>();
                        self.registers[A] = self.registers[A] - self.registers[B];
                    },
                    Instruction::Mul => {
                        self.consume::<u8>();
                        self.registers[A] = self.registers[A] * self.registers[B];
                    },
                    Instruction::Div => {
                        self.consume::<u8>();
                        self.registers[C] = self.registers[A] % self.registers[B];
                        self.registers[A] = self.registers[A] / self.registers[B];
                    },
                    Instruction::Cmpi => {
                        self.consume::<u8>();
                        self.registers[A] = (self.registers[A] == self.registers[B]) as i32;
                    },
                    Instruction::Nop => {
                        self.consume::<u8>();
                    },
                }
            }
            Err(_) => {
                panic!("unexpected data in program : {:#?} at {}", memory.read_i32(self.ip), self.ip);
            }
        }
		let cycles = self.read_register(RT) + 1;
		self.write_register(RT, cycles);
        return true;
    }

    pub fn write_register(&mut self, register: usize, value: i32) {
        if register > self.registers.len() {
            panic!("register write out of bounds.. : register {} does not exist.", register);
        }
        self.registers[register] = value;
    }
	pub fn read_register(&mut self, register: usize) -> i32 {
        if register > self.registers.len() {
            panic!("register write out of bounds.. : register {} does not exist.", register);
        }
        self.registers[register]
    }
    
    pub fn run(&mut self, memory: &mut Memory) {
		self.initialize();
        loop {
            if !self.cycle(memory) {
                break;
            }
        }
    }
    
    pub fn initialize(&mut self) {
        self.ip = 0;
        self.write_register(RT, 0);
    }
}