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
    fn test_cpu_initialization() {
        let cpu = CPU::new();
        assert_eq!(cpu.ip, 0);
    }

    #[test]
    fn test_program_builder() {
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
    fn test_program_output() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Load);
        builder.u32(0);
        builder.u32(250);
        builder.instruction(Instruction::Load);
        builder.u32(1);
        builder.u32(250);
        builder.instruction(Instruction::Mul);
        
        builder.instruction(Instruction::Store);
        builder.u32(0);
        builder.u32(100);

        builder.instruction(Instruction::Halt);

        let mut cpu = CPU::new();
        let mut mem = Memory::new();

        builder.build(&mut mem);
        
        cpu.run(&mut mem);
        assert_eq!(mem.read_i32(100), 62500);
    }
}