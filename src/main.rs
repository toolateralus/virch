use crate::{opcodes::Instruction, cpu::{CPU, ProgramBuilder}};

mod opcodes;
mod cpu;

fn main() {
    let mut cpu = CPU {
        ip:0, 
        registers:[0; 32]
    };

    let mut memory = [0; 4096];

    let mut builder = ProgramBuilder::new();

    builder.instruction(Instruction::Load); // 1
    builder.u32(0); // 5
    builder.u32(255); // 9

    builder.instruction(Instruction::Load); // 10
    builder.u32(1); // 14
    builder.u32(255); // 18

    builder.instruction(Instruction::Mul); // 19

    builder.instruction(Instruction::Store); // 20
    builder.u32(0); // 24
    builder.u32(255); // 28

    builder.instruction(Instruction::Halt); // 29

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
255-255+4: {}", cycles, u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]));
}