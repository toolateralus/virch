use crate::{opcodes::Instruction, cpu::{CPU, Program, ProgramBuilder}};

mod opcodes;
mod cpu;

fn main() {
    let mut cpu = CPU {
        ip:0, 
        registers:[0; 32]
    };

    let mut memory = [0; 4096];

    let mut builder = ProgramBuilder::new();

    builder.instruction(Instruction::Load);
    builder.u32(0);
    builder.u32(255);

    builder.instruction(Instruction::Load);
    builder.u32(1);
    builder.u32(255);

    builder.instruction(Instruction::Mul);

    builder.instruction(Instruction::Store);
    builder.u32(0);
    builder.u32(255);

    builder.instruction(Instruction::Halt);

    builder.build(&mut memory);

    let mut cycles = 0;

    loop {
        cycles += 1;
        if !cpu.cycle(&mut memory) {
            break;
        }
    }

    let slice = &memory[255..255 + 4];

    println!("
program ran for : {} cycles.
255-255+4: {:?}", cycles, slice);
}