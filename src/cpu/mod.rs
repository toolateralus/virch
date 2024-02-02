use std::mem::size_of;

use crate::{opcodes::Opcode, memory::Memory, register::*};

#[cfg(test)]
mod test;

pub const NUM_REGISTERS : usize = 14;

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
        let ins = Opcode::from(ins);
    
        match ins {
            Ok(instruction) => {
                match instruction {
					Opcode::Count => {
						panic!("unexpected data in program : {:#?} at {}", memory.read_i32(self.ip), self.ip);
					}
                    Opcode::Halt => { return false; },
                    Opcode::Jump => { self.ip = memory.read_i32(self.ip + 1) as usize; },
                    Opcode::Store => {
                        self.consume::<u8>();
                        let register = memory.read_i32(self.ip) as usize;
                        self.consume::<i32>();
                        let address = memory.read_i32(self.ip) as usize;
                        self.consume::<i32>();
                        memory.write_i32(address, self.read_register(register));
                    },
                    Opcode::Load => {
                        self.consume::<u8>();
                        let register = memory.read_i32(self.ip) as usize;
                        self.consume::<i32>();
                        let value = memory.read_i32(self.ip);
                        self.consume::<i32>();
                        self.write_register(register, value);
                    },
					Opcode::LoadPointer => {
						self.consume::<u8>();
                        let register = memory.read_i32(self.ip) as usize;
                        self.consume::<i32>();
                        let addr = memory.read_i32(self.ip);
                        self.consume::<i32>();
						let value = memory.read_i32(addr as usize);
                        self.write_register(register, value);
					},
                    Opcode::LoadRegsiter => {
						self.consume::<u8>();
						let register_lhs = memory.read_i32(self.ip) as usize;
						self.consume::<i32>();
						let register_rhs = memory.read_i32(self.ip) as usize;
						self.consume::<i32>();
						let value = self.read_register(register_rhs);
						self.write_register(register_lhs, value);
					},
                    Opcode::Add => {
                        self.consume::<u8>();
                        self.registers[A] = self.registers[A] + self.registers[B];
                    },
                    Opcode::Subtract => {
                        self.consume::<u8>();
                        self.registers[A] = self.registers[A] - self.registers[B];
                    },
                    Opcode::Multiply => {
                        self.consume::<u8>();
                        self.registers[A] = self.registers[A] * self.registers[B];
                    },
                    Opcode::Divide => {
                        self.consume::<u8>();
                        self.registers[C] = self.registers[A] % self.registers[B];
                        self.registers[A] = self.registers[A] / self.registers[B];
                    },
                    Opcode::CompareInteger => {
                        self.consume::<u8>();
                        self.registers[A] = (self.registers[A] == self.registers[B]) as i32;
                    },
                    Opcode::NoOperation => {
                        self.consume::<u8>();
                    },
                    Opcode::Push => {
						self.consume::<u8>();
						let register = memory.read_i32(self.ip);
						self.consume::<i32>();
						let mut sp = self.read_register(SP);
						sp -= size_of::<i32>() as i32;
						self.write_register(SP, sp);
						memory.write_i32(sp as usize, self.read_register(register as usize));
					}
                    Opcode::Pop => {
						self.consume::<u8>();
						let register = memory.read_i32(self.ip);
						self.consume::<i32>();
						let mut sp = self.read_register(SP);
						self.write_register(register as usize, memory.read_i32(sp as usize));
						sp += size_of::<i32>() as i32;
						self.write_register(SP, sp);
					}
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