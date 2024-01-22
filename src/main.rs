use memory::Memory;

use crate::cpu::CPU;

mod opcodes;
mod cpu;
mod memory;
mod register;

use std::env;

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
    use crate::{memory::Memory, opcodes::{Instruction, ProgramBuilder}, register::*};
    use super::*;
    
    #[test]
    fn cpu_initialization() {
        let mut cpu = CPU::new();
		cpu.initialize();
        assert_eq!(cpu.ip, 0);
        assert_eq!(cpu.read_register(RT), 0);
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
		cpu.write_register(RAX, 250);
		cpu.write_register(RDI, 300);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.read_register(RAX), 75000);
    }
    #[test]
    fn division() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Div);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(RAX, 500);
		cpu.write_register(RDI, 2);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.read_register(RAX), 250);
    }
    #[test]
    fn modulus() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Div);

        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(RAX, 5);
		cpu.write_register(RDI, 3);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.read_register(RSI), 2);
    }
    #[test]
    fn addition() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Add);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(RAX, 123);
		cpu.write_register(RDI, 1);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.read_register(RAX), 124);
    }
    #[test]
    fn subtraction() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Sub);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(RAX, 432);
		cpu.write_register(RDI, 1);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.read_register(RAX), 431);
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
        
        assert_eq!(cpu.read_register(RAX), 123);
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
		cpu.write_register(RAX, 432);
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
		cpu.write_register(RAX, 123);
		cpu.write_register(RDI, 123);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.read_register(RAX), 1);
    }
    #[test]
    fn compare_not_equal() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Cmpi);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.write_register(cpu::A, 123);
		cpu.write_register(RDI, 321);
        cpu.cycle(&mut mem);
        
        assert_eq!(cpu.read_register(RAX), 0);
    }
    #[test]
    fn halting() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Halt);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
        assert!(!cpu.cycle(&mut mem));
    }
    #[test]
    fn no_operation() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Nop);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
        assert!(cpu.cycle(&mut mem));
    }
    #[test]
    fn cycle_counting() {
        let mut builder = ProgramBuilder::new();
        builder.instruction(Instruction::Nop);
        builder.instruction(Instruction::Nop);
        builder.instruction(Instruction::Nop);
        
        let mut mem = Memory::new();
        builder.build(&mut mem);
        
        let mut cpu = CPU::new();
		cpu.cycle(&mut mem);
		cpu.cycle(&mut mem);
		cpu.cycle(&mut mem);
        
        assert_eq!(cpu.read_register(RT), 3);
    }
}