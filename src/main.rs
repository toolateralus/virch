use memory::Memory;

use crate::cpu::CPU;

mod opcodes;
mod cpu;
mod memory;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as a command line argument.");
    }
    let file_path = &args[1];
    let mut memory = Memory::from_bytes(file_path);
    let mut cpu = CPU::new();
    cpu.run(&mut memory);
    println!("Program exited with code: {}", cpu.registers[0]);
}

#[cfg(test)]
mod tests {
    use crate::{opcodes::{Instruction, ProgramBuilder}, memory::Memory};
    use super::*;
    
    #[test]
    fn cpu_initialization() {
        let cpu = CPU::new();
        assert_eq!(cpu.ip, 0);
    }

    #[test]
    fn program_builder() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Load);
        builder.u32(6);
        builder.i32(255);
        builder.u8(245);
        
        let mut memory = Memory::new();
        builder.build(&mut memory);

        let memory = memory.memory; // the inner array.
        assert_eq!(memory[0], Instruction::Load as u8, "instruction (load) failed to build.");
        assert_eq!(memory[1..5], [0x06,0x00,0x00,0x00], "u32 failed to build.");
        assert_eq!(memory[5..9], [0xFF,0x00,0x00,0x00], "i32 failed to build.");
        assert_eq!(memory[9], 245, "u8 failed to build.");
    }
    #[test]
    fn multiplication() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Mul);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(0, 250);
		cpu.write_register(1, 300);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.registers[0], 75000);
    }
    #[test]
    fn division() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Div);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(0, 500);
		cpu.write_register(1, 2);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.registers[0], 250);
    }
    #[test]
    fn modulus() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Div);

        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(0, 5);
		cpu.write_register(1, 3);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.registers[2], 2);
    }
    #[test]
    fn addition() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Add);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(0, 123);
		cpu.write_register(1, 1);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.registers[0], 124);
    }
    #[test]
    fn subtraction() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Sub);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(0, 432);
		cpu.write_register(1, 1);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.registers[0], 431);
    }
    #[test]
    fn loading() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Load);
		builder.u32(0);
		builder.i32(123);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.registers[0], 123);
    }
    #[test]
    fn storing() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Store);
		builder.u32(0);
		builder.i32(100);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(0, 432);
        cpu.cycle(&mut mem);
        
        assert_eq!(mem.read_i32(100), 432);
    }
    #[test]
    fn jumping() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Jump);
		builder.i32(100);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.ip, 100);
    }
    #[test]
    fn compare_equal() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Cmpi);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(0, 123);
		cpu.write_register(1, 123);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.registers[0], 1);
    }
    #[test]
    fn compare_not_equal() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Cmpi);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(0, 123);
		cpu.write_register(1, 321);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.registers[0], 0);
    }
}