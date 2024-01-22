use cpu::Memory;

use crate::{opcodes::Instruction, cpu::{CPU, ProgramBuilder, MEM_SIZE}};

mod opcodes;
mod cpu;

fn main() {
    let cpu = CPU::new();
    let memory = Memory::new();
}

#[cfg(test)]
mod tests {

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
        builder.u32(0);
        builder.u32(250);
        builder.instruction(Instruction::Mul);

        builder.instruction(Instruction::Store);
        builder.u32(0);
        builder.u32(100);

        builder.instruction(Instruction::Halt);

        let mut cpu = CPU::new();
        let mut mem = Memory::new();

        builder.build(&mut mem);

        

    }
}